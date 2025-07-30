use crate::client;
use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;
use std::marker::Send;

pub mod models;
pub mod supabase;

#[async_trait]
pub trait DatabaseClient {
    async fn get(
        &self,
        table_name: &str,
        query: Option<&str>,
    ) -> Result<reqwest::Response, reqwest::Error>;

    async fn post<T: Debug + Serialize + Send>(
        &self,
        table_name: &str,
        items: Vec<T>,
    ) -> Result<reqwest::Response, reqwest::Error>;
}

// // TODO:
// pub enum Database {
//   Supabase(supabase::Supabase),
// }

pub async fn create_bills<C: DatabaseClient>(
    database_client: &C,
    bills_from_api: &Vec<client::DtlVo>,
) -> Result<Vec<models::BillRow>, reqwest::Error> {
    let bills = bills_from_api
        .iter()
        .map(|b| models::BillRow::new(b))
        .collect();

    let response = database_client
        .post("information_disclosure_request", bills)
        .await;
    match response {
        Ok(result) => {
            let status_code = result.status().as_u16();
            if status_code != 200 && status_code != 201 {
                panic!(
                    "Supabase 업로드에 실패하였습니다\n상태코드: {}\n에러: {}",
                    result.status(),
                    result.text().await.unwrap_or_default(),
                )
            }

            result.json::<Vec<models::BillRow>>().await
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}

pub async fn find_bills<C: DatabaseClient>(
    database_client: &C,
    query: &str,
) -> Result<Vec<models::BillRow>, reqwest::Error> {
    let response = database_client.get("bills", Some(query)).await;
    match response {
        Ok(result) => result.json::<Vec<models::BillRow>>().await,
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}
