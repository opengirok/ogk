use crate::client;
use crate::utils::date;
use chrono::prelude::*;
use clap::{AppSettings, Subcommand};
use std::error::Error;

#[derive(Subcommand)]
#[clap(about = "Fetch query to open.go.kr", author, long_about = None, version)]
pub enum Commands {
    #[clap(
        about = "Fetch bills with date range",
        setting(AppSettings::ArgRequiredElseHelp)
    )]
    Bills {
        #[clap(long = "from", required = false)]
        from: Option<String>,
        #[clap(long = "page", required = false)]
        page: Option<i32>,
        #[clap(long = "to", required = false)]
        to: Option<String>,
        #[clap(long = "page-size", required = false)]
        page_size: Option<i32>,
    },
}

async fn fetch_bills(
    page: &i32,
    from_date: &str,
    to_date: &str,
    page_size: &i32,
) -> Result<(), Box<dyn Error>> {
    let mut client = client::Client::new().await?;
    client.auth_from_storage().await?;

    println!("{}, {}, {}", page, from_date, to_date);

    let response = client
        .fetch_bills(page, from_date, to_date, page_size)
        .await?;

    let pretty_response = serde_json::to_string_pretty(&response)?;
    println!("{}", pretty_response);

    Ok(())
}

pub async fn run(args: &Commands) -> Result<(), Box<dyn Error>> {
    match args {
        Commands::Bills {
            from,
            page,
            page_size,
            to,
        } => {
            let from_date = match from {
                Some(date) => date.to_owned(),
                None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
            };
            let to_date = match to {
                Some(td) => td.to_owned(),
                None => date::KstDateTime::from(Utc::now()).format(Some("%Y-%m-%d")),
            };

            let _page = match page {
                Some(p) => p.to_owned(),
                None => 1 as i32,
            };

            let _page_size = match page_size {
                Some(ps) => ps.to_owned(),
                None => 10 as i32,
            };

            let _result = fetch_bills(&_page, &from_date, &to_date, &_page_size).await;
        }
    }

    Ok(())
}
