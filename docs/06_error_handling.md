# 06. 에러 처리와 안전한 코딩 🛡️

> *"자바나 파이썬의 try-catch 예외(Exception)는 어디로 갔을까요? 러스트는 에러를 '숨겨진 폭탄'이 아닌 '명시적인 반환값'으로 다룹니다."*

---

## 6.1 패닉! 회복 불가능한 에러 (`panic!`)

Rust는 에러를 크게 두 가지 범주로 나눕니다:

1. **회복 불가능한 에러 (Unrecoverable Error)**: 시스템 메모리가 고갈되었거나, 배열 범위를 벗어난 인덱스에 접근하는 등 프로그램이 더 이상 정상 동작할 수 없을 때 발생합니다.
2. **회복 가능한 에러 (Recoverable Error)**: 파일을 열려고 했는데 파일이 없거나, 네트워크 요청이 일시적으로 타임아웃된 경우처럼 개발자가 재시도하거나 사용자에게 다른 옵션을 제시할 수 있는 상황입니다.

### `panic!` 매크로
회복 불가능한 상태가 감지되면 Rust는 **`panic!`**을 일으키며 프로그램을 즉시 중단(Abort 또는 Stack Unwinding)시키고 메모리를 안전하게 정리합니다.

```rust
fn main() {
    // 의도적인 패닉 발생
    // panic!("치명적인 문제가 발생하여 프로그램을 즉시 종료합니다! 💥");

    let v = vec![1, 2, 3];
    let element = v[99]; // ❌ 런타임에 panic 발생! (index out of bounds)
}
```

```text
thread 'main' panicked at src/main.rs:7:19:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

> [!TIP]
> 디버깅 시 어디서 패닉이 터졌는지 스택 호출 트레이스를 보고 싶다면 환경 변수 `RUST_BACKTRACE=1`을 켜고 실행하세요.  
> `RUST_BACKTRACE=1 cargo run`

---

## 6.2 회복 가능한 에러: `Result<T, E>`

대부분의 일상적인 에러는 회복 가능합니다.  
Rust에는 다른 언어의 `try-catch` 같은 숨겨진 제어 흐름이 없습니다. 대신 결과를 나타내는 표준 열거형 **`Result<T, E>`**를 반환합니다.

```rust
enum Result<T, E> {
    Ok(T),  // 작업이 성공하여 타입 T의 결과값을 담고 있음!
    Err(E), // 작업이 실패하여 타입 E의 에러 정보를 담고 있음!
}
```

### 파일을 열어보는 실전 예제
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일 생성 실패: {:?}", e),
            },
            other_error => {
                panic!("알 수 없는 파일 열기 에러: {:?}", other_error);
            }
        },
    };
}
```

`Result`를 반환받으면 개발자는 성공했을 때와 실패했을 때의 동작을 명시적으로 적어야 합니다. 에러 처리를 깜빡해서 생기는 런타임 참사를 사전에 차단할 수 있습니다!

---

## 6.3 에러 전파의 마법 열쇠: `?` 연산자 🔑

위의 `match` 중첩 코드를 보면 "에러 처리를 꼼꼼히 하는 건 좋은데, 코드가 너무 길어지고 지저분해진다"는 생각이 들 수 있습니다.  
이때 사용하는 Rust의 가장 우아한 문법이 바로 **물음표 연산자(`?`)**입니다!

### 물음표(`?`)가 하는 일
함수 안에서 `Result` 값을 돌려주는 식 뒤에 `?`를 붙이면:
- 만약 **`Ok(값)`**이면: 알맹이 `값`을 껍질에서 쏙 꺼내서 그대로 변수에 넘겨줍니다.
- 만약 **`Err(에러)`**이면: **현재 함수를 즉시 중단하고, 그 에러를 호출자에게 밖으로 바로 반환(전파)**해 버립니다!

```rust
use std::fs::File;
use std::io::{self, Read};

// 파일에서 사용자 이름을 읽어오는 함수
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?; // 실패 시 함수 즉시 탈출 후 Err 반환!
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;      // 실패 시 함수 즉시 탈출 후 Err 반환!
    Ok(username)                                       // 성공 시 Ok로 감싸서 반환
}
```

### 더 짧게 체이닝(Chaining)하기!
```rust
fn read_username_concise() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

놀랍도록 간결해졌죠? 가독성은 일반 동기 코드처럼 깔끔하면서도, 모든 단계의 에러가 안전하게 전파됩니다.

---

## 6.4 `unwrap`은 언제 쓰고 언제 피해야 할까?

코드 예제나 프로토타입을 작성할 때 자주 보이는 두 메서드가 있습니다:

```rust
// 1. unwrap(): 성공하면 값을 꺼내고, 에러면 무조건 panic!
let f = File::open("hello.txt").unwrap();

// 2. expect(): 에러로 패닉 시 내가 지정한 친절한 메시지를 출력!
let f = File::open("hello.txt").expect("hello.txt 파일은 반드시 존재해야 합니다!");
```

### 권장 사용 가이드

| 상황 | 사용 권장 여부 | 이유 |
| :--- | :--- | :--- |
| **빠른 프로토타이핑 / 학습 중** | ⭕ 권장 | 당장 문법을 테스트하고 개념을 증명하는 데 유용함 |
| **단위 테스트 코드 (`#[test]`)** | ⭕ 적극 권장 | 테스트는 에러가 나면 즉시 패닉으로 실패해야 하므로 |
| **절대 실패하지 않음을 개발자가 확신할 때** | ⭕ 사용 가능 | 하드코딩된 올바른 IP 주소를 파싱할 때 등 |
| **실제 운영(Production) 서버 코드** | ❌ **지양** | `unwrap()`으로 인해 서버 프로세스 전체가 갑자기 다운될 위험이 있음 |

> [!TIP]
> 실제 제품 코드에서는 `unwrap()` 대신 **`?` 연산자를 사용하여 상위 레이어로 에러를 넘기거나**, `match` / `unwrap_or_else()`로 기본값 대체 처리를 하는 것이 가장 안전합니다.

---

에러 처리의 기본과 물음표(`?`) 연산자의 마법을 익혔습니다!  
다음 07장에서는 코드를 모듈 단위로 깔끔하게 정리하고 전 세계 러스테이시안들의 라이브러리를 가져와 쓰는 **모듈과 패키지 생태계**를 배워보겠습니다! 🦀🚀

