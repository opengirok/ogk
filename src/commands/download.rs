use crate::client::{self, BillReturnType};
use crate::files::FileManager;
use crate::utils::date;
use crate::utils::log;
use crate::utils::progress;
use chrono::prelude::*;
use clap::Args;
use indicatif::{HumanDuration, ProgressBar};
use std::error::Error;
use std::time::Instant;

#[derive(Args, Debug)]
pub struct Commands {
    #[clap(long = "from", required = false)]
    from: Option<String>,
    #[clap(long = "to", required = false)]
    to: Option<String>,
    #[clap(long = "with-slack-notification", required = false)]
    with_slack: Option<bool>,
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
    let from_date = match &args.from {
        Some(date) => date.to_owned(),
        None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
    };
    let to_date = match &args.to {
        Some(td) => td.to_owned(),
        None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
    };

    let mut print_type = log::PrintType::DEFAULT;
    let _with_slack = &args.with_slack.unwrap_or_default();
    if *_with_slack == true {
        print_type = log::PrintType::SLACK;
    }

    let started = Instant::now();
    let mut client = client::Client::new();
    client.auth_from_storage().await?;

    let init_page = 1 as i32;
    let init_count = 1 as i32;

    log::print(
        &format!(
            "DOWNLOAD [1/4] {}{} ~ {} 기간 동안의 청구 내역을 조회합니다.",
            progress::LOOKING_GLASS,
            &from_date,
            &to_date
        ),
        &log::PrintType::DEFAULT,
    )
    .await;

    let response = client
        .fetch_bills(&init_page, &from_date, &to_date, &init_count)
        .await?;

    let fm = FileManager::new().await.unwrap();
    let total_count = &response.vo.totalPage;
    let mut download_count = 0;

    log::print(
        &format!(
            "DOWNLOAD [2/4] {}청구 내역 {}건 중 공개된 파일을 찾아 다운로드 합니다.",
            progress::DISK,
            total_count
        ),
        &print_type,
    )
    .await;

    let pb = ProgressBar::new(*total_count as u64);

    match client
        .fetch_bills(&init_page, &from_date, &to_date, total_count)
        .await
    {
        Ok(response) => {
            for bill in &response.list {
                pb.inc(1);

                if &bill.deptSn == "2" {
                    let _response_bill = client
                        .fetch_a_bill(
                            &bill.rqestProcRegstrNo,
                            &bill.insttRqestProcStCd,
                            &bill.deptSn,
                        )
                        .await?;
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

            log::print(
                &format!(
                    "DOWNLOAD [3/4] {} 다운로드한 파일을 원격 저장소에 저장합니다.",
                    progress::WRITE,
                ),
                &print_type,
            )
            .await;

            if download_count > 0 {
                let _result2 = fm.upload().await;
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    };

    log::print(
        &format!(
            "DOWNLOAD [4/4] {} 총 {}건 다운로드 및 원격 저장소 업로드 완료! - {}",
            progress::SPARKLE,
            download_count,
            HumanDuration(started.elapsed())
        ),
        &print_type,
    )
    .await;

    Ok(())
}
