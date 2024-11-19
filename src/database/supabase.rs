use crate::{database::DatabaseClient, utils::config};

use std::fmt::Debug;
use std::marker::Send;

use async_trait::async_trait;
use reqwest::{self, header};
use serde::Serialize;

#[derive(Debug)]
pub struct Supabase {
    client: reqwest::Client,
    host: String,
}

impl Supabase {
    pub fn new() -> Self {
        let _config = config::Config::load_or_new().unwrap();
        let supabase_host = _config
            .supabase_host
            .expect("supabase host 를 먼저 설정해주세요.");
        let supabase_api_key = _config
            .supabase_api_key
            .expect("supabase api key 를 먼저 설정해주세요.");

        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("{} {}", "Bearer", supabase_api_key)
                .parse()
                .unwrap(),
        );

        headers.insert("apiKey", supabase_api_key.parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();

        Supabase {
            client,
            host: supabase_host.to_owned(),
        }
    }
}

#[async_trait]
impl DatabaseClient for Supabase {
    async fn get(
        &self,
        table_name: &str,
        query_string: Option<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let builder = self.client.get(format!(
            "{}/rest/v1/{}?{}",
            &self.host,
            table_name,
            query_string.unwrap_or_default()
        ));

        builder.send().await
    }

    async fn post<T: Debug + Serialize + Send>(
        &self,
        table_name: &str,
        items: Vec<T>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let builder = self
            .client
            .post(format!("{}/rest/v1/{}", &self.host, table_name))
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates")
            .json(&items);

        builder.send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::DtlVo;
    use crate::database::models::BillRow;

    #[tokio::test]
    async fn test_get() {
        let supabase = Supabase::new();
        let query = "open_status_code=in.%28\"121\",\"131\"%29";
        let response = supabase.get("bills", Some(query)).await;
        match response {
            Ok(r) => {
                let bills = r.json::<Vec<BillRow>>().await.unwrap();
                println!("{:?}", bills.len());
                assert_eq!(bills.len(), 7);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }

    #[ignore]
    async fn test_post() {
        let supabase = Supabase::new();
        let bill = BillRow::new(&DtlVo {
            clsdrResnCn: format!("test"),
            clsdrResnNm: format!("test"),
            chckerFnm: format!("test"),
            chckerClsfNm: format!("test"),
            dcrberFnm: format!("test"),
            dcrberClsfNm: format!("test"),
            dcanerFnm: format!("test"),
            dcanerClsfNm: format!("test"),
            drafterFnm: format!("test"),
            drafterClsfNm: format!("test"),
            othinstSmtmProcessYn: format!("N"),
            sanctnDocNo: format!("test"),
            // pub sanctnerClsfNm: String,   // 결재정보 - 기안자 직위/직급
            // pub sanctnerFnm: String,      // 결재권자 이름
            // pub sanctnerDt: String,       // 결재일자 이름
            // pub sanctnerRequstDt: String, // 결재 요청 일자
            deptSn: format!("2"),
            decsnCn: format!("test"),
            trnsfInsttNmCn: format!("test"),
            opetrId: format!("test"),
            opetrFnm: format!("test"),
            opetrDeptCd: format!("test"),
            opetrDeptNm: format!("test"),
            opetrClsfCd: format!("test"),
            opetrClsfNm: format!("test"),
            opetrCbleTelno: format!("test"),
            othbcDtApnResnNm: format!("test"),
            othbcOprtnDt: format!("test"),
            // pub othbcInfoCnfirmDt: String, // *공개자료 열람 일시
            othbcPrearngeDt: format!("test"),
            othbcSeNm: format!("test"),
            othbcStleSeNm: format!("test"),
            recptMthSeNm: format!("test"),
            recptnServerId: format!("test"),
            // nticeCnfirmDt: String, // *결정통지 열람일시
            nticeDt: format!("test"),
            insttAddr: format!("test"),
            insttRqestProcStCd: format!("test"),
            insttRqestProcStNm: format!("test"),
            mberId: format!("test"),
            // procCd: String,             // [empty]
            prcsInsttCd: format!("test"),
            prcsInsttNm: format!("test"),
            prcsFullInsttNm: format!("test"),
            // prcsFullInsttNm: String,    // [empty]
            procCn: format!("test"),
            procDt: format!("test"),
            procRegstrNo: format!("test"),
            procDeptCbleTelno: format!("test"),
            procUserEmailAdres: format!("test"),
            rceptDt: format!("test"),
            rqestCn: format!("test"),
            rqestDt: format!("test"),
            rqestFullInsttNm: format!("test"),
            rqestInsttCd: format!("test"),
            rqestInsttNm: format!("test"),

            rqestProcRegstrNo: format!("test"),
            rqestRceptNo: format!("test"),
            rqestSj: format!("test"),
        });

        let bills = vec![bill];
        let response = supabase.post("bills", bills).await;

        match response {
            Ok(r) => {
                let bills = r.json::<Vec<BillRow>>().await.unwrap();
                assert_eq!(bills.len(), 1);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    }
}
