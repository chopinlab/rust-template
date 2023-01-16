# rust-template

## rust
- version : 1.66.1

## rust 설치
```shell
# 1. rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 2. brew 
brew install rust
```

## rust 업데이트
```shell
rustup update
```

## cargo 명령어
```shell
cargo new rust-template --bin
cargo run      # 카고 실행
cargo build    # 러스트 빌드
cargo check    # 컴파일 상태 체크
```

## 폴더 구조
- src
  - api
  - db
  - service
  - configs
  - lib.rs
  - main.rs
- target
- Cargo.toml
- Cargo.lock


## 사용 라이브러리
- 설정 라이브러리
  - https://github.com/mehcode/config-rs


- scheduler cron 라이브러리
  - (사용 방법) https://medium.com/@knoldus/schedule-the-program-in-rust-a368f710a17f
  - https://github.com/lholden/job_scheduler

## 문법 정리
```rust
let x: Result<u32, &str> = Ok(2);
println!("{}", x.unwrap());
println!("{}", x.ok().unwrap());

let x1: Result<u32, i32> = Err(2);
println!("{}", x1.err().unwrap());

let y: Option<i32> = Some(2);
println!("{}", y.unwrap());


```

## 1.TODO
- 해야 할 것
  - [x] 폴더 구조 정하기
  - [x] 설정 라이브러리
  - [ ] orm 테스트
  - [ ] REST Request
  - [ ] CI/CD 구축
  - [ ] cron
  - 웹 프레임워크
  - logger
  - swagger
  - test code
