# Learn Rust

러스트를 배워볼까요

## Getting Started
### convenient settings
1. vscode `settings.json` 에서 다음을 저장
   ``` "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer", // Makes the magic
    "editor.formatOnSave": true, // Optional
  },```

### cargo

1. cargo build : 말 그대로 빌드
   1. 빌드된 파일들은 ./target에 저장
   2. `Cargo.toml->package.name`.EXE 파일이 생성, 실행가능
2. cargo run : 빌드 후 실행
   1. 위 exe 파일을 실행
3. cargo check : 컴파일을 실행해주지만 실행 가능한 파일을 생성하지 않음
4. cargo build --release : 배포 전용으로 빌드를 수행함. (Rust code run faster)

## Common Programming Concept

### Variables and mutability
1. Immutability
   1. 변수가 불변할 때, 이름에 할당된 값을 바꿀 수 없음.
   2. `mut`을 사용하면 변경이 가능
2. Constants
   1. `const`는 `mut` 을 사용할 수 없음.
   2. `const`는 런타임에서 계산된 값을 포함할 수 없음 (const는 const끼리만 계산 가능)
3. Shadowing
   1. 불변한 변수도 덮어씌우기가 가능하다는 것. (scope 레벨에서 상이)
   2. `mut`은 타입까지 바꿀 수는 없지만, shadowing은 바꿀 수 있음..? (variable be immutable after those transformations have been completed. )
   3. 근데 좀 이상해서 찾아보니, [이런 글](https://stackoverflow.com/questions/59860476/what-is-the-rationale-behind-allowing-variable-shadowing-in-rust)이.. (대충 그냥 안쓴다는 말)