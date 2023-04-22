# Learn Rust

러스트를 배워볼까요

## Getting Started

### cargo

1. cargo build : 말 그대로 빌드
   1. 빌드된 파일들은 ./target에 저장
   2. `Cargo.toml->package.name`.EXE 파일이 생성, 실행가능
2. cargo run : 빌드 후 실행
   1. 위 exe 파일을 실행
3. cargo check : 컴파일을 실행해주지만 실행 가능한 파일을 생성하지 않음
4. cargo build --release : 배포 전용으로 빌드를 수행함. (Rust code run faster)