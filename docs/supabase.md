# Supabase 설정하기

1. [Supabase](https://supabase.com/)에 가입합니다.
2. `Organization`과 `Project`를 생성합니다.
3. 데이터를 저장할 `Table`을 생성합니다.
    1. 좌측 메뉴 `SQL Editor`에 들어갑니다.
    2. `+ New query` 버튼을 클릭합니다.
    3. 아래 쿼리를 복사하여 붙여넣고 실행합니다.
      ```sql
      CREATE TABLE bills (
        registration_proc_number varchar(11) PRIMARY KEY,
        registration_number varchar(11),
        group_id varchar(11),
        request_date varchar(11),
        notice_date varchar(11),
        open_status varchar(11),
        open_status_code varchar(11),
        open_date varchar(11),
        open_date_reason varchar(11),
        request_subject varchar(11),
        request_description varchar(11),
        result_description varchar(11),
        open_type varchar(11),
        proc_person_class varchar(11),
        proc_person_name varchar(11),
        proc_org_name varchar(11),
        proc_org_dept_name varchar(11),
        proc_org_dept_code varchar(11),
        proc_org_dept_phone varchar(11),
        transfered_org_name varchar(11),
        sanction_drafter_name varchar(11),
        sanction_drafter_class varchar(11),
        sanction_dcaner_name varchar(11),
        sanction_dcrber_class varchar(11),
        sanction_dcrber_name varchar(11),
        sanction_dcaner_class varchar(11),
        sanction_checker_name varchar(11),
        sanction_checker_class varchar(11),
        open_file_method varchar(11),
        dept_sn varchar(11),
        user_id varchar(11)
      );
      ```

4. (⚠️ 중요) 생성한 테이블에 대하여 비공개 설정을 합니다.
   1. 좌측 메뉴 `Table Editor`에 들어갑니다.
   2. 방금 생성한 테이블 `bills`를 클릭합니다.
   3. 테이블 이름 옆에 클릭하고 나서 생긴 화살표를 클릭하여 `Edit Table`을 클릭합니다.
   4. `Enable Row Level Security` 체크박스에 체크표시를 한 뒤 저장합니다.
5. Supabase `Host` 와 `API Key`를 복사하여 설정에 적용합니다.
      1. 좌측 메뉴 `Settings`에 들어갑니다.
      2. 하위 메뉴 `Project settings` - `API`에 들어갑니다.
      3. `Configuration` - `URL`이 `Host`이고, `Project API Keys` - `service role, secret`이 `API Key` 입니다.
