# ogk - cli for [open.go.kr <대한민국 정보공개플랫폼>](https://open.go.kr)
[정보공개플랫폼](https://open.go.kr) 계정이 있고 를랫폼을 자주 사용하는 사용자라면 공식웹사이트보다 편리하게 데이터 및 파일을 관리할 수 있도록 돕기 위한 프로젝트입니다.

## How to Use

### 0. 설정

#### 0-1. 개인 계정 인증하기
```bash
ogk auth login --username <username> --password <password>
```

#### 0-2. 원격 저장소 설정하기
: 
```bash
# Github에 연동된 ssh 파일 위치를 저장합니다.
ogk config repository-pub-key </path/to/ssh/private/key>

# 파일 저장 및 관리를 위한 github repository를 설정합니다.
# example: hoonyland/data
ogk config repository <remote url>
```

### 1. 조회하기

- 단건 조회하기
```bash
ogk fetch bill --id <bill-id>
```

- 복수건 조회하기(날짜 & 페이지 단위 조회)
```bash
ogk fetch bills --from 2021-01-01 --to 2020-12-31 --page 1
```

### 2. 파일 다운로드

```bash
ogk download bill --id <bill-id>
ogk download bills --from 2021-01-01 --to 2021-12-31
```

### 3. 데이터 관리하기(WIP)
- [ ] sync bill
- [ ] sync bills

```bash
ogk sync bill --id <bill-id>
ogk sync bills --from 2021-01-01 --to 2021-12-31
```

## Roadmap

### 1. 계정 보안
- [ ] open.go.kr 계정 관리 보안 강화

### 2. 파일 관리
- [ ] 로컬 파일 관리 기능(원격 저장소와 분리)

### 3. 스케쥴러
- [ ] 스케쥴러 관리 명령어 개발

## 외부 라이브러리
- [clap](https://docs.rs/clap/3.0.0-beta.2/clap): rust cli builder
- [reqwest](https://docs.rs/reqwest/0.10.10/reqwest): http client
- [supabase](https://supabase.io): database as a service