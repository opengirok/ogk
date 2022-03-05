use crate::client::{self, BillReturnType};
use crate::files::FileManager;
use crate::utils::date;
use crate::utils::progress;
use chrono::prelude::*;
use clap::{AppSettings, Subcommand};
use console::style;
use indicatif::{HumanDuration, ProgressBar};
use std::error::Error;
use std::time::Instant;

#[derive(Subcommand)]
#[clap(author, version, about, long_about = None)]
pub enum Commands {
  #[clap(setting(AppSettings::ArgRequiredElseHelp))]
  Bill {
    #[clap(long = "id")]
    id: String,
  },
  Bills {
    #[clap(long = "from", required = false)]
    from: Option<String>,
    #[clap(long = "to", required = false)]
    to: Option<String>,
  },
}

async fn download_bill(bill_id: &str) -> Result<(), Box<dyn Error>> {
  let mut client = client::Client::new();
  client.auth_from_storage().await?;

  let _response = client
    .fetch_a_bill_by_registration_proc_number(bill_id)
    .await?;

  match _response {
    BillReturnType::BillWithFiles(response) => {
      let fm = FileManager::new().await.unwrap();
      let _result = fm.download(&client, &response).await.unwrap();
    }
    BillReturnType::RedirectedBillWithFiles(_) => {}
    BillReturnType::None => {}
  }

  Ok(())
}

async fn download_bills(from_date: &str, to_date: &str) -> Result<(), Box<dyn Error>> {
  let started = Instant::now();
  let mut client = client::Client::new();
  client.auth_from_storage().await?;

  let init_page = 1 as i32;
  let init_count = 1 as i32;

  println!(
    "{} {}{} ~ {} 기간 동안의 청구 내역을 조회합니다.",
    style("[1/3]").bold().dim(),
    progress::LOOKING_GLASS,
    &from_date,
    &to_date
  );

  let response = client
    .fetch_bills(&init_page, from_date, to_date, &init_count)
    .await?;

  let fm = FileManager::new().await.unwrap();
  let total_count = &response.vo.totalPage;
  let mut download_count = 0;

  println!(
    "{} {}청구 내역 {}건 중 공개된 파일을 찾아 다운로드 합니다.",
    style("[2/3]").bold().dim(),
    progress::DISK,
    total_count
  );

  let pb = ProgressBar::new(*total_count as u64);

  match client
    .fetch_bills(&init_page, from_date, to_date, total_count)
    .await
  {
    Ok(response) => {
      for bill in &response.list {
        pb.inc(1);

        if &bill.deptSn == "2" {
          let _response_bill = client.fetch_a_bill(&bill).await?;
          match _response_bill {
            BillReturnType::BillWithFiles(response) => {
              let _result = fm.download(&client, &response).await.unwrap();
              download_count = download_count + 1;
            }
            _ => {}
          };
        }
      }

      pb.finish_and_clear();

      println!(
        "{} {} 다운로드한 파일을 원격 저장소에 저장합니다.",
        style("[3/3]").bold().dim(),
        progress::WRITE,
      );

      if download_count > 0 {
        let _result2 = fm.upload().await;
      }
    }
    Err(e) => {
      panic!("{}", e);
    }
  };

  println!(
    "{} 총 {}건 다운로드 및 원격 저장소 업로드 완료! - {}",
    progress::SPARKLE,
    download_count,
    HumanDuration(started.elapsed())
  );

  Ok(())
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
  match args {
    Commands::Bill { id } => {
      let _result = download_bill(id).await;
    }
    Commands::Bills { from, to } => {
      let from_date = match from {
        Some(date) => date.to_owned(),
        None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
      };
      let to_date = match to {
        Some(td) => td.to_owned(),
        None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
      };

      let _result = download_bills(&from_date, &to_date).await;
    }
  }

  Ok(())
}
