# 08. 실전 미니 프로젝트: 나만의 CLI Todo 매니저 🦀🔨

> *"백문이 불여일타! 지금까지 배운 모든 지식을 총동원하여 터미널에서 작동하는 진짜 Todo 애플리케이션을 만들어 봅시다."*

---

## 8.1 프로젝트 기획 및 요구사항

우리가 만들 프로그램은 터미널 명령어로 할 일(Todo)을 추가하고, 확인하고, 완료 처리하고, 삭제할 수 있는 CLI(Command Line Interface) 도구입니다. 프로그램이 종료되어도 데이터가 사라지지 않도록 **JSON 파일로 로컬 디스크에 자동 저장**됩니다.

### 지원할 명령어 기능
- `cargo run -- add "할 일 내용"`: 새로운 할 일 추가
- `cargo run -- list`: 전체 할 일 목록 보기 (`[ ]` 미완료, `[x]` 완료)
- `cargo run -- done <번호>`: 해당 번호의 할 일을 완료 상태로 변경
- `cargo run -- remove <번호>`: 해당 번호의 할 일 삭제

---

## 8.2 단계별 구현 준비

새 프로젝트를 만들거나, 연습용 폴더에서 실습해 볼 수 있습니다.

### 1. 의존성 추가 (`Cargo.toml`)
데이터를 JSON으로 저장하고 불러오기 위해 가장 널리 쓰이는 `serde`와 `serde_json` 크레이트를 추가합니다.

```bash
cargo add serde --features derive
cargo add serde_json
```

`Cargo.toml`의 `[dependencies]` 부분은 다음과 같이 설정됩니다:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 8.3 데이터 모델 및 비즈니스 로직 설계

우리가 배운 **구조체(Struct)**와 **`#[derive]`**, **`Result<T, E>`**를 적극 활용합니다.

### 1. 할 일 아이템 (`TodoItem`) 구조체
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub text: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        Self {
            text,
            completed: false, // 처음 추가할 땐 미완료 상태
        }
    }
}
```

### 2. 할 일 목록 관리 (`TodoList`)
```rust
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}
```

---

## 8.4 완성된 전체 소스 코드 (`src/main.rs`)

아래 코드를 복사하여 `src/main.rs`에 붙여넣기만 하면 즉시 작동하는 완전한 프로그램입니다!

```rust
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const FILE_PATH: &str = "todos.json";

#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    text: String,
    completed: bool,
}

impl TodoItem {
    fn new(text: String) -> Self {
        Self {
            text,
            completed: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    // 파일에서 TodoList를 읽어옵니다. (파일이 없으면 빈 목록 생성)
    fn load() -> Self {
        if !Path::new(FILE_PATH).exists() {
            return Self::default();
        }

        let mut file = match File::open(FILE_PATH) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    // 현재 TodoList를 파일에 JSON 형태로 저장합니다.
    fn save(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(FILE_PATH)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    // 할 일 추가
    fn add(&mut self, text: String) {
        let item = TodoItem::new(text);
        println!("✨ 할 일이 추가되었습니다: '{}'", item.text);
        self.items.push(item);
    }

    // 할 일 목록 출력
    fn list(&self) {
        if self.items.is_empty() {
            println!("📭 등록된 할 일이 없습니다! 쉬는 시간인가요? ☕");
            return;
        }

        println!("\n📋 [나의 Todo 리스트]");
        for (index, item) in self.items.iter().enumerate() {
            let status = if item.completed { "[x]" } else { "[ ]" };
            println!("  {}. {} {}", index + 1, status, item.text);
        }
        println!();
    }

    // 할 일 완료 처리
    fn complete(&mut self, index: usize) {
        if index > 0 && index <= self.items.len() {
            self.items[index - 1].completed = true;
            println!("🎉 {}번 할 일을 완료했습니다! 멋져요!", index);
        } else {
            println!("⚠️ 유효하지 않은 번호입니다: {}", index);
        }
    }

    // 할 일 삭제
    fn remove(&mut self, index: usize) {
        if index > 0 && index <= self.items.len() {
            let removed = self.items.remove(index - 1);
            println!("🗑️ '{}' 항목이 삭제되었습니다.", removed.text);
        } else {
            println!("⚠️ 유효하지 않은 번호입니다: {}", index);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    let mut todo_list = TodoList::load();

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("⚠️ 추가할 할 일 내용을 입력해주세요!");
            } else {
                let text = args[2..].join(" ");
                todo_list.add(text);
                let _ = todo_list.save();
            }
        }
        "list" => {
            todo_list.list();
        }
        "done" => {
            if let Some(num_str) = args.get(2) {
                if let Ok(num) = num_str.parse::<usize>() {
                    todo_list.complete(num);
                    let _ = todo_list.save();
                } else {
                    println!("⚠️ 올바른 숫자를 입력해주세요.");
                }
            } else {
                println!("⚠️ 완료할 번호를 입력해주세요. 예: cargo run -- done 1");
            }
        }
        "remove" => {
            if let Some(num_str) = args.get(2) {
                if let Ok(num) = num_str.parse::<usize>() {
                    todo_list.remove(num);
                    let _ = todo_list.save();
                } else {
                    println!("⚠️ 올바른 숫자를 입력해주세요.");
                }
            } else {
                println!("⚠️ 삭제할 번호를 입력해주세요. 예: cargo run -- remove 1");
            }
        }
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("🦀 [Rust CLI Todo 사용법]");
    println!("  cargo run -- add <할일>     : 새로운 할 일 추가");
    println!("  cargo run -- list           : 전체 목록 조회");
    println!("  cargo run -- done <번호>    : 해당 번호 완료 처리");
    println!("  cargo run -- remove <번호>  : 해당 번호 삭제");
}
```

---

## 8.5 직접 실행해보기! (시나리오 실습)

터미널에서 아래 명령들을 순서대로 입력해보며 프로그램이 잘 동작하는지 확인해 보세요!

### 1. 할 일 2개 등록하기
```bash
cargo run -- add "러스트 E-Book 3장 소유권 복습하기"
cargo run -- add "맛있는 저녁 식사하기"
```

### 2. 목록 확인하기
```bash
cargo run -- list
```
**출력 결과:**
```text
📋 [나의 Todo 리스트]
  1. [ ] 러스트 E-Book 3장 소유권 복습하기
  2. [ ] 맛있는 저녁 식사하기
```

### 3. 첫 번째 할 일 완료하기
```bash
cargo run -- done 1
```
**출력 결과:**
```text
🎉 1번 할 일을 완료했습니다! 멋져요!
```

### 4. 다시 목록 확인해보기
```bash
cargo run -- list
```
**출력 결과:**
```text
📋 [나의 Todo 리스트]
  1. [x] 러스트 E-Book 3장 소유권 복습하기
  2. [ ] 맛있는 저녁 식사하기
```

### 5. `todos.json` 파일 열어보기
프로젝트 폴더에 새로 생긴 `todos.json` 파일을 열어보면, 데이터가 깔끔한 JSON 포맷으로 저장되어 있는 것을 확인할 수 있습니다!

---

직접 손으로 완성한 나만의 CLI 도구, 어떠셨나요?  
소유권, 가변 참조(`&mut self`), 에러 처리, 파일 I/O, 외부 크레이트 연동까지 모든 조각이 하나의 멋진 프로그램으로 완성되었습니다! 🦀🎉

마지막 부록인 **09장: 치트시트 & 추천 로드맵**에서 자주 쓰는 문법 요약과 다음 학습 길잡이를 확인해 보세요!

