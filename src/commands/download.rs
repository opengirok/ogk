use crate::client::{self, BillReturnType, DntcFile};
use crate::files::FileManager;
use crate::utils::{date, log, progress};
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
    let mut client = client::Client::new().await?;
    client.auth_from_storage().await?;

    let init_page = 1 as i32;
    let init_count = 1 as i32;

    log::print(
        &format!(
            "[{}] DOWNLOAD [1/5] {}{} ~ {} 기간 동안의 청구 내역을 조회합니다.",
            client.username,
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

    log::print(
        &format!(
            "[{}] DOWNLOAD [2/5] {}다운로드 받기 전 원격 저장소 최신 정보를 확인합니다.",
            client.username,
            progress::HAND_WITH_EYE,
        ),
        &print_type,
    )
    .await;

    fm.sync_with_remote().await;

    log::print(
        &format!(
            "[{}] DOWNLOAD [3/5] {}청구 내역 {}건 중 공개된 파일을 찾아 다운로드 합니다.",
            client.username,
            progress::DISK,
            total_count
        ),
        &print_type,
    )
    .await;

    let pb = ProgressBar::new(*total_count as u64);
    let mut downloaded_files: Vec<DntcFile> = vec![];

    match client
        .fetch_bills(&init_page, &from_date, &to_date, total_count)
        .await
    {
        Ok(response) => {
            for bill in &response.list {
                pb.inc(1);

                let _response_bill = client
                    .fetch_a_bill(
                        &bill.rqestProcRegstrNo,
                        &bill.insttRqestProcStCd,
                        &bill.deptSn,
                    )
                    .await?;
                match _response_bill {
                    BillReturnType::BillWithFiles(response) => {
                        let mut _result = fm
                            .download(&client, &response, bill)
                            .await
                            .unwrap()
                            .unwrap_or_default();
                        downloaded_files.append(&mut _result);
                    }
                    _ => {}
                };
            }

            pb.finish_and_clear();

            log::print(
                &format!(
                    "[{}] DOWNLOAD [4/5] {}다운로드한 총 {}개의 파일을 원격 저장소에 저장합니다.",
                    client.username,
                    progress::WRITE,
                    downloaded_files.len()
                ),
                &print_type,
            )
            .await;

            if downloaded_files.len() > 0 {
                let _result2 = fm.upload().await;
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    };

    let downloaded_file_names = downloaded_files
        .iter()
        .map(|d| format!("- {}", d.uploadFileOrginlNm))
        .collect::<Vec<String>>()
        .join("\n");

    log::print(
        &format!(
            "[{}] DOWNLOAD [5/5] {} 다운로드 및 원격 저장소 업로드 완료! - {}\n{}",
            client.username,
            progress::SPARKLE,
            HumanDuration(started.elapsed()),
            &downloaded_file_names
        ),
        &print_type,
    )
    .await;

    Ok(())
}
