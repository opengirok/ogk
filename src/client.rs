#![allow(non_snake_case)]

use crate::files::{Downloadable, FileManager};
use crate::utils::auth::AuthConfig;

use bytes::Bytes;
use regex::Regex;
use reqwest::{self, header, Error};
use std::str;

const LIST_HOST: &str = "https://www.open.go.kr/rqestMlrd/rqestDtls/reqstDocSrchList.ajax";
const LOGIN_HOST: &str = "https://www.open.go.kr/com/login/memberLogin.ajax";
const DETAIL_HOST_FOR_NOT_OPENED: &str =
  "https://www.open.go.kr/rqestMlrd/rqestDtls/reqstDocDetail.do";
const DETAIL_HOST_FOR_OPENED: &str =
  "https://www.open.go.kr/rqestMlrd/rqestDtls/reqstDocDecsnNotie.do";
const DOWNLOAD_HOST: &str = "https://www.open.go.kr/util/FileDownload.do";

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModelResultRtnV0 {
  pub accesType: String,
  pub addr1: String,
  pub addr2: String,
  pub age: i32,
  pub agent: String,
  pub agentInfo: String,
  pub apoloId: String,
  pub birth: String,
  pub birthDe: String,
  pub bizrNo: String,
  pub bizrNo1: String,
  pub bizrNo2: String,
  pub bizrNo3: String,
  pub changePwdYn: String,
  pub crt: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModelResult {
  pub error_code: String,
  pub error_msg: String,
  pub mberSeCd: String,
  pub sysdate: String,
  pub today: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndViewModel {
  pub result: AuthResponseModelAndViewModelResult,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponseModelAndView {
  pub empty: bool,
  pub model: AuthResponseModelAndViewModel,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthResponse {
  pub modelAndView: AuthResponseModelAndView,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ListVo {
  pub totalPage: i32, // 아이템 전체 개수
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct DtlVo {
  pub deptSn: String, // 파일 여부

  pub clsdrResnCn: String, // *비공개내용
  pub clsdrResnNm: String, // *비공개제목

  pub chckerClsfNm: String,  // 결재정보 - 검토자 이름
  pub chckerFnm: String,     // 결재정보 - 검토자 이름
  pub dcrberFnm: String,     // 결재정보 - 전결자 이름
  pub dcrberClsfNm: String,  // 결재정보 - 전결자 직급/직위
  pub dcanerFnm: String,     // 결재정보 - 대결자 이름
  pub dcanerClsfNm: String,  // 결재정보 - 대결자 직급/직위
  pub drafterFnm: String,    // 결재정보 - 기안자 이름
  pub drafterClsfNm: String, // 결재정보 - 기안자 직급/직위
  pub sanctnDocNo: String,   // 결재정보 - 문서 번호

  // pub sanctnerClsfNm: String,   // 결재정보 - 기안자 직위/직급
  // pub sanctnerFnm: String,      // 결재권자 이름
  // pub sanctnerDt: String,       // 결재일자 이름
  // pub sanctnerRequstDt: String, // 결재 요청 일자
  pub decsnCn: String,        // 공개내용/이송사유 ex)
  pub trnsfInsttNmCn: String, // 이송 기관

  // pub feeRdcxptResnCd: String, // 수수료 감면 코드?
  // pub feeRdcxptResnNm: String, // 수수료 감면 사유?
  // pub feeRdcxptYn: String,     // 수수료 감면 여부?
  // pub feeSumAmt: String,       // 수수료
  pub opetrId: String,        // *처리기관 내 ID
  pub opetrFnm: String,       // *처리기관 처리자 이름
  pub opetrDeptCd: String,    // *처리기관 처리과 코드
  pub opetrDeptNm: String,    // *처리기관 처리과 이름
  pub opetrClsfCd: String,    // *처리기관 처리자 직위/직급 코드
  pub opetrClsfNm: String,    // *처리기관 처리자 직위/직급
  pub opetrCbleTelno: String, // *처리기관 처리

  pub othinstSmtmProcessYn: String,
  pub othbcDtApnResnNm: String, // *공개일시 지정 사유 ex) 수수료납부 완료후 바로 공개
  pub othbcOprtnDt: String,     // *공개 일시
  // pub othbcInfoCnfirmDt: String, // *공개자료 열람 일시
  pub othbcSeNm: String,       // 공개여부 ex) 공개
  pub othbcStleSeNm: String,   // 공개방법 - 교부형태 ex) 전자파일
  pub othbcPrearngeDt: String, // *공개 일시

  pub recptMthSeNm: String,   // 공개방법 - 교부방법 ex) 정보통신망
  pub recptnServerId: String, //

  // pub nticeCnfirmDt: String, // *결정통지 열람일시
  pub nticeDt: String, // *공개일시

  pub insttAddr: String,          // 처리기관 주소
  pub insttRqestProcStCd: String, // 처리상태 코드 ex) 143
  pub insttRqestProcStNm: String, // 처리상태명 ex) 공개완료

  pub mberId: String, // 사용자이름 // ex) opengirok

  // pub procCd: String,             // [empty]
  pub prcsInsttCd: String, // *처리기관 코드 ex) 6110000
  pub prcsInsttNm: String, // *처리기관 이름 short ver ex) 서울특별시
  // pub prcsFullInsttNm: String,    // [empty]
  pub procCn: String,             // 통지 결과 상태
  pub procDt: String,             // 통지 일자
  pub procRegstrNo: String,       // *세부 페이지 요청에 필요한 번호
  pub procDeptCbleTelno: String,  // *처리기관 전화번호
  pub procUserEmailAdres: String, // 처리자 전자우편

  pub rceptDt: String,          // 접수일자 ex)  2020.09.12
  pub rqestCn: String,          // 청구내용 ex)
  pub rqestDt: String,          // 청구내용 ex)
  pub rqestFullInsttNm: String, // ex) 요청기관 이름 full ver. - 고용노동부 최저임금위원회
  pub rqestInsttCd: String,     // 요청기관 코드 ex) 1492865
  pub rqestInsttNm: String,     // ex) 요청기관 이름 short ver. - 최저임금위원회

  pub rqestProcRegstrNo: String, // 처리번호 * 세부 페이지 요청에 필요한 번호
  pub rqestRceptNo: String,      // 접수번호 * 세부 페이지 요청에 필요한 번호
  pub rqestSj: String,           // 요청 제목 ex) 최저임금 위원회 회의록 및 속기록 (JE)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DntcFile {
  pub atchmnflByteCo: String,     // '100081',
  pub atchmnflPrsrvNm: String,    // '202007171546284220000.zip',
  pub csdCnvrStCd: String,        // '020',
  pub fileAbsltCoursNm: String,   // '/pidfiles/uploads/pb/dlsrinfo/',
  pub fileSn: String,             // '1',
  pub fileUploadNo: String,       // 'VVdXZnJWYWI5Mm5GTzlsN1dWdno0QT09',
  pub frstRegisterId: String,     // 'MIG',
  pub uploadFileOrginlNm: String, // ex) '서범수 의원 요구자료 일체.zip',
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct BillWithFiles {
  pub atchFileList: Option<Vec<DntcFile>>,
  pub dntcFileList: Option<Vec<DntcFile>>,
  pub dtlVo: DtlVo,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RedirectedBillWithFiles {
  pub redirectUrl: String,
}

#[derive(Debug)]
pub enum BillReturnType {
  BillWithFiles(BillWithFiles),
  RedirectedBillWithFiles(RedirectedBillWithFiles),
  None,
}

impl Downloadable for BillWithFiles {
  fn get_filename(&self, orig_file_name: &str) -> String {
    FileManager::make_filename(
      &self.dtlVo.rqestProcRegstrNo.trim(),
      &self.dtlVo.prcsInsttNm.trim(),
      &self.dtlVo.insttRqestProcStNm.trim(),
      orig_file_name.trim(),
    )
  }

  fn get_dirname(&self) -> String {
    FileManager::make_dirname(&self.dtlVo.rceptDt.trim(), &self.dtlVo.rqestSj.trim())
  }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Bills {
  pub list: Vec<DtlVo>,
  pub vo: ListVo,
}

#[derive(Debug)]
pub struct Client {
  client: reqwest::Client,
  scui: String,
}

impl Client {
  pub fn new() -> Self {
    let mut headers = header::HeaderMap::new();
    headers.insert(
      "Accept",
      "application/json, text/javascript, */*; q=0.01"
        .parse()
        .unwrap(),
    );
    headers.insert(
      "Content-Type",
      "application/x-www-form-urlencoded; charset=UTF-8"
        .parse()
        .unwrap(),
    );
    headers.insert("Host", "www.open.go.kr".parse().unwrap());
    headers.insert("Origin", "https://www.open.go.kr".parse().unwrap());
    headers.insert(
      "User-Agent",
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:80.0) Gecko/20100101 Firefox/80.0"
        .parse()
        .unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
      .default_headers(headers)
      .cookie_store(true)
      .build()
      .unwrap();

    let scui = "";

    Client {
      client,
      scui: scui.to_owned(),
    }
  }

  pub async fn auth(
    &mut self,
    username: &str,
    password: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let auth: [(&str, &str); 3] = [("mberId", username), ("pwd", password), ("agent", "PC")];
    let response = self.client.post(LOGIN_HOST).form(&auth).send().await?;
    match response.json::<AuthResponse>().await {
      Ok(response_json) => {
        if response_json.modelAndView.model.result.error_msg == "로그인 완료" {
          let response_scui = self
            .client
            .post("https://www.open.go.kr/com/main/mainView.do")
            .send()
            .await?;
          let response_scui_text = response_scui.text().await?;

          let regex = Regex::new(r"const scui = '(.+)';").unwrap();
          for cap in regex.captures_iter(&response_scui_text) {
            self.scui = cap[0].to_owned();
          }

          return Ok(());
        }

        if response_json.modelAndView.model.result.error_msg
          == "비밀번호를 마지막으로 변경한지 180일이 지났습니다."
        {
          let set_password: [(&str, &str); 2] = [("hash", "true"), ("scui", &self.scui)];

          self
            .client
            .post("https://www.open.go.kr/com/main/mainView.do")
            .form(&set_password)
            .send()
            .await?;

          return Ok(());
        }

        panic!("사용자이름과 비밀번호를 확인해주세요.");
      }
      Err(_) => {
        panic!("사용자이름과 비밀번호를 확인해주세요.");
      }
    }
  }

  pub async fn auth_from_storage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    let config = AuthConfig::load()?;
    let decoded_password = base64::decode(&config.default.password.as_bytes())?;
    let password = str::from_utf8(&decoded_password)?;
    self.auth(&config.default.username, password).await?;
    Ok(())
  }

  pub async fn download_file(
    &self,
    rqest_proc_regstr_no: &str,
    file_upload_no: &str,
  ) -> Result<Bytes, Error> {
    let pre_params = &[("rqestProcRegstrNo", rqest_proc_regstr_no)];
    self
      .client
      .post("https://www.open.go.kr/rqestMlrd/rqestDtls/updateReqstInfoCnfirmDt.ajax")
      .form(pre_params)
      .send()
      .await?;

    let params = &[("fileUploadNo", &file_upload_no), ("fileSn", &"1")];
    self
      .client
      .post(DOWNLOAD_HOST)
      .form(params)
      .send()
      .await?
      .bytes()
      .await
  }

  pub async fn fetch_a_bill(
    &self,
    dtlVo: &DtlVo,
  ) -> Result<BillReturnType, Box<dyn std::error::Error>> {
    let mut deptSn = dtlVo.deptSn.clone();

    let host = match dtlVo.insttRqestProcStCd.as_str() {
      "141" | "143" | "1411" | "1413" | "1415" | "1421" | "163" | "165" | "1861" => {
        DETAIL_HOST_FOR_OPENED
      }
      "171" | "172" => {
        deptSn = format!("");

        if &dtlVo.othinstSmtmProcessYn == "N" {
          DETAIL_HOST_FOR_OPENED
        } else {
          DETAIL_HOST_FOR_NOT_OPENED
        }
      }
      _ => DETAIL_HOST_FOR_NOT_OPENED,
    };

    let params: [(&str, &str); 8] = [
      ("rqestRceptNo", &dtlVo.rqestRceptNo),
      ("rqestProcRegstrNo", &dtlVo.rqestProcRegstrNo),
      ("procRegstrNo", &dtlVo.procRegstrNo),
      ("insttRqestProcStCd", &dtlVo.insttRqestProcStCd),
      ("deptSn", &deptSn),
      ("hash", "true"),
      ("multiDeptProcYn", "N"),
      ("scui", &self.scui),
    ];

    let response = self.post(host, &params).await?;
    let text_response = response.text().await?;

    let regex = Regex::new(r"var result(\s+)=(\s+)(.+);").unwrap();
    let mut stringified_json_result = String::from("");
    for cap in regex.captures_iter(&text_response) {
      stringified_json_result = String::from(&cap[3]);
    }

    if stringified_json_result != "" {
      match serde_json::from_str(&stringified_json_result) {
        Ok(result) => {
          return Ok(BillReturnType::BillWithFiles(result));
        }
        Err(_) => {
          let wrong_result: RedirectedBillWithFiles =
            serde_json::from_str(&stringified_json_result).unwrap();
          return Ok(BillReturnType::RedirectedBillWithFiles(wrong_result));
        }
      };
    } else {
      return Ok(BillReturnType::None);
    }
  }

  pub async fn fetch_a_bill_by_registration_proc_number(
    &self,
    registration_proc_number: &str,
  ) -> Result<BillReturnType, Box<dyn std::error::Error>> {
    let params: [(&str, &str); 8] = [
      ("rqestRceptNo", ""),
      ("rqestProcRegstrNo", registration_proc_number),
      ("procRegstrNo", registration_proc_number),
      ("insttRqestProcStCd", ""),
      ("deptSn", "2"),
      ("hash", "true"),
      ("multiDeptProcYn", "N"),
      ("scui", &self.scui),
    ];

    let response = self.post(DETAIL_HOST_FOR_OPENED, &params).await?;
    let text_response = response.text().await?;

    let regex = Regex::new(r"var result(\s+)=(\s+)(.+);").unwrap();
    let mut stringified_json_result = String::from("");
    for cap in regex.captures_iter(&text_response) {
      stringified_json_result = String::from(&cap[3]);
    }

    if stringified_json_result != "" {
      match serde_json::from_str(&stringified_json_result) {
        Ok(result) => {
          return Ok(BillReturnType::BillWithFiles(result));
        }
        Err(_) => {
          let wrong_result: RedirectedBillWithFiles =
            serde_json::from_str(&stringified_json_result).unwrap();
          return Ok(BillReturnType::RedirectedBillWithFiles(wrong_result));
        }
      };
    } else {
      return Ok(BillReturnType::None);
    }
  }

  pub async fn fetch_bills(
    &self,
    page: &i32,
    from_date: &str,
    to_date: &str,
    page_count: &i32,
  ) -> Result<Bills, Error> {
    let params: [(&str, &str); 9] = [
      ("stRceptDt", from_date),
      ("edRceptDt", to_date),
      ("searchYn", "Y"),
      ("selRowPage", &page_count.to_string()),
      ("moveStatus", "L"),
      ("viewPage", &page.to_string()),
      ("rowPage", &page_count.to_string()),
      ("chkDate", "nonClass"),
      ("scui", &self.scui),
    ];

    let response = self.client.post(LIST_HOST).form(&params).send().await?;
    response.json::<Bills>().await
  }

  pub async fn post(&self, url: &str, form: &[(&str, &str)]) -> Result<reqwest::Response, Error> {
    self.client.post(url).form(form).send().await
  }
}
