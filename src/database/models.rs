use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::client::DtlVo;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BillRow {
    pub registration_number: String,
    pub registration_proc_number: String,
    pub group_id: Option<String>,

    pub proc_org_dept_code: Option<String>,
    pub proc_org_dept_phone: Option<String>,
    pub proc_org_dept_name: Option<String>,
    pub proc_org_name: Option<String>,
    pub proc_person_class: Option<String>,
    pub proc_person_name: Option<String>,

    pub dept_sn: Option<String>,
    pub open_type: Option<String>,
    pub open_status: Option<String>,
    pub open_status_code: Option<String>,
    pub open_date: Option<String>,
    pub open_date_reason: Option<String>,
    pub open_file_method: Option<String>,
    pub notice_date: Option<String>,

    pub request_date: String,
    pub request_subject: String,
    pub request_description: String,
    pub result_description: Option<String>,
    pub transfered_org_name: Option<String>,

    pub sanction_checker_name: Option<String>,
    pub sanction_checker_class: Option<String>,
    pub sanction_drafter_name: Option<String>,
    pub sanction_drafter_class: Option<String>,
    pub sanction_dcaner_name: Option<String>,
    pub sanction_dcaner_class: Option<String>,
    pub sanction_dcrber_name: Option<String>,
    pub sanction_dcrber_class: Option<String>,

    pub user_id: String,
}

impl BillRow {
    pub fn new(bill: &DtlVo) -> Self {
        let group_id = BillRow::create_group_id(&bill.rqestSj, &bill.rqestCn);

        let mut result_description: Option<String> = None;
        if !bill.decsnCn.is_empty() {
            result_description = Some(bill.decsnCn.clone());
        } else if !bill.clsdrResnCn.is_empty() {
            result_description = Some(bill.clsdrResnCn.clone());
        }

        let open_date: String = if bill.othbcOprtnDt.is_empty() {
            if bill.othbcPrearngeDt.is_empty() {
                format!("")
            } else {
                bill.othbcPrearngeDt.clone()
            }
        } else {
            bill.othbcOprtnDt.clone()
        };

        BillRow {
            group_id: Some(group_id),
            notice_date: Some(bill.nticeDt.clone()),
            dept_sn: Some(bill.deptSn.clone()),
            open_date: Some(open_date.replace(".", "-")),
            open_date_reason: Some(bill.othbcDtApnResnNm.clone()),
            open_status: Some(bill.insttRqestProcStNm.clone()),
            open_status_code: Some(bill.insttRqestProcStCd.clone()),
            open_type: Some(bill.othbcSeNm.clone()),
            proc_org_dept_code: Some(bill.opetrDeptCd.clone()),
            proc_org_dept_name: Some(bill.opetrDeptNm.clone()),
            proc_org_dept_phone: Some(bill.opetrCbleTelno.clone()),
            proc_org_name: Some(bill.prcsInsttNm.clone()),
            proc_person_class: Some(bill.opetrClsfNm.clone()),
            proc_person_name: Some(bill.opetrFnm.clone()),
            registration_number: bill.rqestRceptNo.clone(),
            registration_proc_number: bill.rqestProcRegstrNo.clone(),
            request_date: bill.rqestDt.replace(".", "-").clone(),
            request_description: bill.rqestCn.clone(),
            request_subject: bill.rqestSj.clone(),

            result_description: result_description,
            open_file_method: Some(bill.othbcStleSeNm.clone()),

            sanction_checker_class: Some(bill.chckerClsfNm.clone()),
            sanction_checker_name: Some(bill.chckerFnm.clone()),
            sanction_drafter_name: Some(bill.drafterFnm.clone()),
            sanction_drafter_class: Some(bill.drafterClsfNm.clone()),
            sanction_dcaner_name: Some(bill.dcanerFnm.clone()),
            sanction_dcaner_class: Some(bill.dcanerClsfNm.clone()),
            sanction_dcrber_name: Some(bill.dcrberFnm.clone()),
            sanction_dcrber_class: Some(bill.dcrberClsfNm.clone()),

            transfered_org_name: Some(bill.trnsfInsttNmCn.clone()),

            user_id: bill.mberId.to_owned(),
        }
    }

    pub fn create_group_id(rqest_sj: &str, rqest_cn: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(format!("{}_{}", rqest_sj, rqest_cn).as_str());
        return hasher.result_str();
    }
}
