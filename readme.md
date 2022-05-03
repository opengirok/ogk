# ogk - cli tool for [open.go.kr](https://open.go.kr)
[정보공개포털](https://open.go.kr) 계정이 있고 플랫폼을 자주 사용하는 사용자라면 공식웹사이트보다 편리하게 데이터 및 파일을 관리할 수 있도록 돕기 위한 프로젝트입니다.


### 후원하기
- [정보공개센터 후원하기](https://www.opengirok.or.kr/)
- [ogk 기여자](https://hoony.land)에게 [커피 한 잔 사주기](https://www.buymeacoffee.com/pretty00butt)


### 설치하기
: 현재 `ogk`는 [Homebrew](https://brew.sh/) 혹은 [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)를 통해서 설치가 가능합니다.

##### `homebrew`

```bash
brew tap opengirok/ogk
brew install ogk
```

##### `cargo`

```bash
cargo install ogk
```

### 설정하기

```bash
# 1. 계정 설정
# 정보공개플랫폼 계정 설정(open.go.kr에 등록된 계정이어야 합니다.)
ogk auth login --username <username> --password <password>


# 2. 파일 관리 설정
# 내컴퓨터에 저장할 파일 위치를 지정합니다.
# 기본값: ~/.ogk/.data
ogk config files --local-repository /Path/to/local/repository

# 3.에서 생성한 원격저장소 주소를 지정합니다.
# 현재 버전(v0.1)에서는 원격저장소가 반드시 지정되어야 파일 관리가 가능합니다.
# 예: ogk config files --remote-repository hoonyland/data
ogk config files --remote-repository <user_or_org-name/repository_name>

# 3. supabase (데이터베이스) 설정
# [Supabase 설정하기](docs/supabase.md) 문서를 참고하여 Supabase 설정을 먼저 완료하시길 바랍니다.
# 위 설정 후 Supabase에서 발급받은 host, api key를 환경변수로 등록합니다.
export OGK_SUPABASE_API_KEY=
export OGK_SUPABASE_HOST=

# 4. SLACK 알림 설정
# Slack Webhook을 생성한 뒤 URL을 환경변수로 설정합니다.
# 각 명령 뒤에 `--with-slack-notification true` 를 붙이면 슬랙 알림이 함께 갑니다.
export OGK_SLACK_WEBHOOK_URL=

```

##### 파일 관리를 위한 요구사항

1. `git` 설치 및 계정 설정
2. github ssh 인증 설정 - [공식문서](https://docs.github.com/en/authentication/connecting-to-github-with-ssh), [참조 블로그](https://devocean.sk.com/blog/techBoardDetail.do?ID=163311)
3. [원격 저장소 생성](https://github.com/new)


### 사용하기

##### 1. 조회하기

- 날짜 & 페이지 단위 조회
```bash
ogk fetch bills --from 2021-01-01 --to 2020-12-31 --page 1
```

##### 2. 파일 다운로드
: [설정하기](#설정하기)에서 파일관리를 위한 설정이 선행되어야 합니다.

```bash
ogk download --from 2021-01-01 --to 2021-12-31
```

##### 3. 데이터 조회 및 저장하기
: [설정하기](#설정하기)에서 supabase 등록 및 설정이 선행되어야 합니다.

```bash

# 1. 기본 조회 및 저장
ogk sync --from 2021-01-01 --to 2021-12-31

# 2. 현재 설정된 Supabase 데이터베이스에서 통지완료되지 않은 건들만 새로 업데이트
# `--from`과 `--to` 옵션을 지정하지 않습니다.
ogk sync
```

### Roadmap

##### 1. 계정 보안
- [ ] open.go.kr 계정 관리 보안 강화

### 외부 라이브러리
- [clap](https://docs.rs/clap/3.0.0-beta.2/clap): rust cli builder
- [reqwest](https://docs.rs/reqwest/0.10.10/reqwest): http client
- [supabase](https://supabase.io): database as a service

### 기여자
 
[pretty00butt](hoony.land)

<a href="https://www.buymeacoffee.com/pretty00butt" target="_blank">
<img src="https://user-images.githubusercontent.com/1366161/156967861-490ed2cb-fa3c-4ef1-acee-6b7649ccdcf9.png" width="25%" />
</a>
 
### License

[MIT License](LICENSE)
