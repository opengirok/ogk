# ogk - cli for [open.go.kr <대한민국 정보공개플랫폼>](https://open.go.kr)
[정보공개플랫폼](https://open.go.kr) 계정이 있고 를랫폼을 자주 사용하는 사용자라면 공식웹사이트보다 편리하게 데이터 및 파일을 관리할 수 있도록 돕기 위한 프로젝트입니다.

### 설정하기

```bash
# 1. 계정 설정
# 정보공개플랫폼 계정 설정(open.go.kr에 등록된 계정이어야 합니다.)
ogk auth login --username <username> --password <password>


# 2. 파일 관리 설정
# 내컴퓨터에 저장할 파일 위치를 지정합니다.
# 기본값: ~/.ogk/.data
ogk config --local-repository /Path/to/local/repository

# 3.에서 생성한 원격저장소 주소를 지정합니다.
# 현재 버전(v0.1)에서는 원격저장소가 반드시 지정되어야 파일 관리가 가능합니다.
# 예: ogk config repository hoonyland/data
ogk config --remote-repository user_or_org-name/repository_name

# 3. supabase (데이터베이스) 설정
# supabase에서 발급받은 host, api key를 환경변수로 등록합니다.
export OGK_SUPABASE_API_KEY={SUPABASE_API_KEY}
export OGK_SUPABASE_HOST={SUPABASE_HOST}

```

##### 파일 관리를 위한 요구사항

1. `git` 설치 및 계정 설정
2. github ssh 인증 설정 - [공식문서](https://docs.github.com/en/authentication/connecting-to-github-with-ssh), [참조 블로그](https://devocean.sk.com/blog/techBoardDetail.do?ID=163311)
3. [원격 저장소 생성](https://github.com/new)


### 사용하기
##### 1. 조회하기

- 단건 조회하기
```bash
ogk fetch bill --id <bill-id>
```

- 복수건 조회하기(날짜 & 페이지 단위 조회)
```bash
ogk fetch bills --from 2021-01-01 --to 2020-12-31 --page 1
```

##### 2. 파일 다운로드
: [설정하기](#설정하기)에서 파일관리를 위한 설정이 선행되어야 합니다.

```bash
ogk download bill --id <bill-id>
ogk download bills --from 2021-01-01 --to 2021-12-31
```

##### 3. 데이터 조회 및 저장하기
: [설정하기](#설정하기)에서 supabase 등록 및 설정이 선행되어야 합니다.

```bash
ogk sync --from 2021-01-01 --to 2021-12-31
```

### Roadmap

##### 1. 계정 보안
- [ ] open.go.kr 계정 관리 보안 강화

### 외부 라이브러리
- [clap](https://docs.rs/clap/3.0.0-beta.2/clap): rust cli builder
- [reqwest](https://docs.rs/reqwest/0.10.10/reqwest): http client
- [supabase](https://supabase.io): database as a service

### 기여자
 
- [pretty00butt](hoony.land)
 
### License

[MIT License](LICENSE)