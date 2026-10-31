# 01. 개발 환경 설정과 첫 발걸음 🛠️

> *"모든 위대한 여정은 환경 설정과 'Hello, World!' 한 줄에서 시작됩니다."*

---

## 1.1 러스트 설치 (rustup)

Rust의 공식 도구 체인 관리자는 **`rustup`**입니다.  
Node.js의 `nvm`이나 파이썬의 `pyenv`처럼 컴파일러 버전 업데이트, 타깃 플랫폼 크로스 컴파일 환경 등을 손쉽게 관리해 줍니다.

### 운영체제별 설치 방법

#### Windows 사용자
1. 공식 웹사이트 [rustup.rs](https://rustup.rs/)에 접속하여 `rustup-init.exe` 설치 파일을 다운로드합니다.
2. 다운로드한 파일을 실행하면 콘솔 창이 뜹니다.
3. 기본 옵션인 `1) Proceed with installation (default)`를 선택하고 Enter를 누르면 자동으로 설치가 진행됩니다.
   *(참고: C++ 빌드 도구가 필요하다는 안내가 나오면 Visual Studio C++ 빌드 도구를 함께 설치해 주시면 됩니다.)*

#### macOS 및 Linux 사용자
터미널을 열고 다음 명령어를 입력합니다:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 설치 확인하기
설치가 완료된 후 터미널(또는 명령 프롬프트 / PowerShell)을 새로 열고 다음 명령어를 입력해 버전을 확인해 봅니다.

```bash
rustc --version
cargo --version
```

버전 번호가 잘 출력된다면 설치가 완벽하게 끝난 것입니다! 🎉

---

## 1.2 최고의 조수: Cargo 도구 다루기

Rust를 배울 때 개발자들이 가장 감탄하는 도구 중 하나가 바로 **Cargo(카고)**입니다.  
Cargo는 **빌드 시스템(Build System)**이자 **패키지 매니저(Package Manager)**로, 코드 컴파일, 외부 라이브러리(Crate) 다운로드, 테스트 실행, 문서 생성까지 모든 것을 도맡아 처리합니다.

### 자주 쓰는 Cargo 4대 핵심 명령어

| 명령어 | 설명 | 언제 쓸까요? |
| :--- | :--- | :--- |
| `cargo new <이름>` | 새로운 프로젝트 생성 | 새 프로젝트를 시작할 때 |
| `cargo check` | 실행 파일 생성 없이 문법 오류만 빠르게 검사 | 코딩 중 문법/타입 오류를 실시간으로 확인할 때 (초고속!) |
| `cargo build` | 프로젝트 컴파일 및 실행 파일 생성 | 최종 결과물을 빌드할 때 |
| `cargo run` | 컴파일 후 바로 실행까지 한 번에 수행 | 코드를 수정하고 바로 결과를 확인할 때 |

> [!TIP]
> **개발 중에는 `cargo check`를 애용하세요!**  
> `cargo build`는 최종 실행 파일(기계어 바이너리)을 디스크에 기록하므로 약간의 시간이 걸리지만, `cargo check`는 바이너리를 만들지 않고 타입과 문법 에러만 번개처럼 빠르게 체크합니다.

---

## 1.3 세상에서 가장 안전한 "Hello, World!"

이제 직접 손으로 첫 프로젝트를 만들어 보겠습니다. 터미널에서 다음 명령을 실행해 보세요.

```bash
cargo new hello_rust
cd hello_rust
```

생성된 폴더 안을 살펴보면 다음과 같은 구조로 파일들이 놓여 있습니다:

```
hello_rust/
├── Cargo.toml      # 프로젝트 설정 및 외부 라이브러리 명세서
└── src/
    └── main.rs     # 프로그램의 시작점이 되는 소스 코드
```

### `Cargo.toml` 살펴보기
TOML(Tom's Obvious, Minimal Language) 형식으로 작성된 프로젝트 설정 파일입니다.

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# 나중에 필요한 외부 라이브러리들을 여기에 적습니다.
```

### `src/main.rs` 코드 살펴보기
파일을 열어보면 이미 기본 코드가 작성되어 있습니다.

```rust
fn main() {
    println!("Hello, World!");
}
```

한 줄씩 뜯어볼까요?

1. **`fn main() { ... }`**:
   - `fn`은 함수(Function)를 정의할 때 쓰는 키워드입니다.
   - `main` 함수는 모든 실행 파일의 **시작점(진입점, Entry Point)**입니다.
2. **`println!("Hello, World!");`**:
   - 화면에 글자를 출력하고 줄을 바꿉니다.
   - **어? 왜 `println` 뒤에 느낌표(`!`)가 붙어 있을까요?**  
     Rust에서는 이름 끝에 느낌표(`!`)가 붙은 것을 **매크로(Macro)**라고 부릅니다. 일반 함수가 아니라, 컴파일 타임에 코드를 확장해주는 특별한 기능입니다. 포맷팅 안전성 검사를 컴파일 시점에 수행하기 위해 매크로 형태로 제공됩니다.
3. **세미콜론 (`;`)**:
   - Rust에서는 대부분의 식의 끝에 세미콜론을 붙여 구문(Statement)의 끝을 알립니다. (다음 장에서 세미콜론의 비밀이 밝혀집니다!)

### 실행해보기!
프로젝트 루트 디렉토리에서 터미널에 아래 명령을 쳐보세요:

```bash
cargo run
```

```text
   Compiling hello_rust v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_rust`
Hello, World!
```

축하합니다! 여러분은 방금 첫 번째 러스트 프로그램을 성공적으로 실행하셨습니다! 🦀🎉

---

## 1.4 추천 개발 환경 (VS Code & Rust Analyzer)

러스트 코딩을 가장 쾌적하게 하는 방법은 **Visual Studio Code(VS Code)**와 공식 플러그인을 활용하는 것입니다.

1. **VS Code** 설치
2. Extensions(확장 탭)에서 **`rust-analyzer`** 검색 후 설치  
   *(주의: 예전의 구버전 'Rust' 플러그인이 아닌 꼭 **rust-analyzer**를 설치하세요!)*
3. **`CodeLLDB`** 확장 설치 (브레이크포인트를 잡고 디버깅할 때 아주 유용합니다)
4. **`Even Better TOML`** 확장 설치 (`Cargo.toml` 파일의 문법 강조 및 자동완성 지원)

`rust-analyzer`를 설치하면 변수의 타입을 자동으로 추론해서 연한 회색 글씨로 미리 보여주고, 잘못된 문법에 즉시 친절한 경고를 띄워주므로 개발 속도가 10배 빨라집니다.

준비가 모두 끝났으니, 다음 장에서 러스트의 기본 문법을 차근차근 익혀보겠습니다! 🏃‍♂️💨

