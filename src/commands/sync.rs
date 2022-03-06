use crate::client::{self, BillReturnType};
use crate::database::supabase::Supabase;
use crate::database::{create_bills, find_bills};
use crate::utils::date;
use crate::utils::log;
use crate::utils::progress;
use chrono::{Duration, Utc};
use clap::Args;
use futures::future::join_all;
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
    let mut print_type = log::PrintType::DEFAULT;

    let _with_slack = &args.with_slack.unwrap_or_default();
    if *_with_slack == true {
        print_type = log::PrintType::SLACK;
    }

    let mut client = client::Client::new();
    client.auth_from_storage().await?;

    let started = Instant::now();
    let init_page = 1 as i32;
    let init_count = 1 as i32;

    let mut date_from: String = args.from.as_ref().unwrap_or(&"".to_string()).to_string();
    let mut date_to: String = args.to.as_ref().unwrap_or(&"".to_string()).to_string();

    // TODO:
    if date_from == "" && date_to == "" {
        log::print(
            &format!(
                "SYNC [1/3] {}SUPABASE 데이터베이스에 저장된 청구건들 중 아직 통지완료되지 않은 건들을 조회합니다.",
                progress::LOOKING_GLASS,
            ),
            &log::PrintType::DEFAULT,
        )
        .await;

        let supabase_client = Supabase::new();
        let mut bills: Vec<client::DtlVo> = vec![];
        let bill_rows = find_bills(&supabase_client, "open_status_code=in.%28\"141\"%29").await?;
        let pb = ProgressBar::new(bill_rows.len() as u64);

        log::print(
            &format!(
                "SYNC [2/3] {}각 청구건의 최신 통지 상태를 조회합니다.",
                progress::TRUCK,
            ),
            &log::PrintType::DEFAULT,
        )
        .await;

        for bill in bill_rows {
            pb.inc(1);
            let bill_response = client
                .fetch_a_bill(
                    &bill.registration_proc_number,
                    &bill.open_status_code.unwrap_or("".to_string()),
                    &bill.dept_sn.unwrap_or("1".to_string()),
                )
                .await;

            match bill_response {
                Ok(b) => {
                    match b {
                        BillReturnType::BillWithFiles(res) => {
                            bills.push(res.dtlVo);
                        }
                        _ => {}
                    };
                }
                Err(_) => {}
            }
        }

        pb.finish_and_clear();
        log::print(
            &format!(
                "SYNC [3/3] {}조회한 내역을 데이터베이스에 저장합니다.",
                progress::DISK,
            ),
            &print_type,
        )
        .await;

        let _result = create_bills(&supabase_client, &bills).await;

        return Ok(());
    }

    if date_from == "" {
        date_from =
            date::KstDateTime::from(Utc::now() - Duration::days(1)).format(Some("%Y-%m-%d"));
    };

    if date_to == "" {
        date_to = date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d"));
    };

    log::print(
        &format!(
            "SYNC [1/3] {}{}~{} 청구 내역을 확인합니다.",
            progress::LOOKING_GLASS,
            date_from,
            date_to,
        ),
        &print_type,
    )
    .await;

    let response = match client
        .fetch_bills(&init_page, &date_from, &date_to, &init_count)
        .await
    {
        Ok(_response) => _response,
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    };

    let total_count = &response.vo.totalPage;
    let pb = ProgressBar::new(*total_count as u64);

    log::print(
        &format!(
            "SYNC [2/3] {}청구 내역 {}건을 조회합니다.",
            progress::TRUCK,
            total_count
        ),
        &print_type,
    )
    .await;

    match client
        .fetch_bills(&init_page, &date_from, &date_to, total_count)
        .await
    {
        Ok(response) => {
            let mut bills: Vec<client::DtlVo> = vec![];
            let supabase_client = Supabase::new();

            let mut i = 0;
            let once_loop_len = 30;
            let mut is_last_index = false;

            while i < response.list.len() {
                let mut end_index = i + once_loop_len;
                if response.list.len() < i + once_loop_len {
                    end_index = response.list.len();
                    is_last_index = true;
                }

                pb.inc((end_index - i) as u64);

                let fetch_bills_awaits = response.list[i..end_index].iter().map(|bill| {
                    client.fetch_a_bill(
                        &bill.rqestProcRegstrNo,
                        &bill.insttRqestProcStCd,
                        &bill.deptSn,
                    )
                });

                let results = join_all(fetch_bills_awaits).await;
                for bill in results {
                    match bill {
                        Ok(b) => {
                            match b {
                                BillReturnType::BillWithFiles(res) => {
                                    bills.push(res.dtlVo);
                                }
                                _ => {
                                    eprintln!("error2가 발생했습니다.")
                                }
                            };
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            panic!();
                        }
                    }
                }

                if is_last_index == true {
                    i = end_index;
                } else {
                    i += once_loop_len;
                }
            }

            pb.finish_and_clear();

            log::print(
                &format!(
                    "SYNC [3/3] {}조회한 내역을 데이터베이스에 저장합니다.",
                    progress::DISK,
                ),
                &print_type,
            )
            .await;

            let _result = create_bills(&supabase_client, &bills).await;

            log::print(
                &format!(
                    "SYNC {} 총 {}건 동기화 완료! - {}",
                    progress::SPARKLE,
                    total_count,
                    HumanDuration(started.elapsed())
                ),
                &print_type,
            )
            .await;
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };

    Ok(())
}
