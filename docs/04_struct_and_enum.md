# 04. 데이터 묶기와 패턴 매칭: 구조체와 열거형 🧩

> *"현실 세계의 복잡한 데이터를 코드로 표현하는 가장 직관적이고 안전한 도구들입니다."*

---

## 4.1 구조체(Struct): 나만의 데이터 설계도

구조체(Struct)는 서로 연관된 여러 값들을 하나의 묶음으로 정의할 수 있는 커스텀 데이터 타입입니다. 객체지향 언어의 클래스(Class)에서 상태(데이터)를 정의하는 부분과 비슷합니다.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("ferris@rust-lang.org"),
        username: String::from("ferris_the_crab"),
        active: true,
        sign_in_count: 1,
    };

    // 필드 접근 및 변경 (인스턴스가 mut이어야 함)
    user1.email = String::from("new_ferris@rust-lang.org");
    println!("유저 이름: {}", user1.username);
}
```

### 구조체 업데이트 문법 (`..`)
기존 인스턴스의 대부분 필드를 유지하면서 일부만 바꾸어 새 인스턴스를 만들고 싶을 때 사용합니다.

```rust
let user2 = User {
    email: String::from("user2@rust-lang.org"),
    ..user1 // 나머지 필드는 user1의 값을 그대로 가져옴!
};
```

---

## 4.2 메서드 정의하기 (`impl`과 `self`)

Rust에서는 데이터 구조(`struct`)와 그 데이터가 하는 행동(함수/메서드)을 명확히 분리하여 **`impl` (Implementation) 블록**에 정의합니다.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 1. 인스턴스 메서드: 첫 번째 매개변수로 &self를 받음
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. 다른 사각형을 완전히 포함하는지 검사하는 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 3. 연관 함수 (Associated Function): &self가 없는 생성자 (마치 Rectangle::new 처럼 호출)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let sq = Rectangle::square(25);

    println!("rect1의 넓이: {}", rect1.area()); // 1500
    println!("rect1이 sq를 포함하나요? {}", rect1.can_hold(&sq)); // true
}
```

> [!NOTE]
> **`&self`, `&mut self`, `self`의 차이**
> - `&self`: 데이터를 읽기만 할 때 사용 (가장 흔함)
> - `&mut self`: 메서드 안에서 자신의 필드 값을 바꿀 때 사용
> - `self`: 호출한 인스턴스의 소유권을 완전히 가져와서 소비(소멸)시킬 때 사용

---

## 4.3 열거형(Enum): 상태와 값을 하나로!

많은 언어에서 열거형(Enum)은 단순히 0, 1, 2 같은 정수 상수의 나열에 불과합니다.  
하지만 **Rust의 열거형은 각 선택지(배리언트, Variant)마다 서로 다른 종류와 크기의 데이터를 직접 담을 수 있는 슈퍼 파워**를 가지고 있습니다!

```rust
enum Message {
    Quit,                       // 담고 있는 데이터 없음
    Move { x: i32, y: i32 },    // 익명 구조체 데이터를 담음
    Write(String),              // 문자열을 담음
    ChangeColor(i32, i32, i32), // RGB 세 개의 정수를 담음
}
```

네 가지 완전히 다른 형태의 메시지를 `Message`라는 단 하나의 타입으로 안전하게 묶어 처리할 수 있습니다!

---

## 4.4 10억 달러짜리 실수를 없앤 `Option<T>`

컴퓨터 과학의 거장 토니 호어(Tony Hoare)는 1965년 자신이 고안한 `null` 참조를 가리켜 **"10억 달러짜리 실수(Billion-Dollar Mistake)"**였다고 회고했습니다.  
프로그램이 실행되다가 예상치 못한 `NullPointerException`으로 뻗어버리는 문제가 끊이지 않았기 때문입니다.

Rust는 이를 해결하기 위해 아예 언어 차원에서 **`null`이라는 개념 자체를 없애버렸습니다!**  
대신 값이 "있을 수도 있고 없을 수도 있는 상태"를 표준 라이브러리의 **`Option<T>` 열거형**으로 명시하도록 강제합니다.

```rust
enum Option<T> {
    None,     // 값이 없음 (다른 언어의 null에 해당)
    Some(T),  // 유효한 타입 T의 값이 존재함!
}
```

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

> [!IMPORTANT]
> `i32`와 `Option<i32>`는 **완전히 다른 타입**입니다!  
> 일반 `i32`는 100% 무조건 유효한 숫자가 들어있음이 보장됩니다.  
> 만약 `Option<i32>`라면, 개발자가 컴파일 단계에서 "값이 없을 때(None)는 어떻게 할 것인가?"를 명시적으로 처리하지 않으면 코드 빌드 자체가 되지 않습니다!

---

## 4.5 강력한 무기: `match`와 `if let` 패턴 매칭

`Option`이나 `Enum` 안에 든 알맹이 값을 꺼내거나 상태에 따라 분기할 때 가장 멋진 무기가 바로 **`match`**입니다.

### 1. `match`의 모든 경우 검사 (Exhaustiveness)
Rust의 `match`는 **빠진 케이스가 하나라도 있으면 컴파일을 거부**합니다. 실수로 에러 처리를 빼먹는 휴먼 에러를 100% 막아줍니다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six); // Some(6)
    println!("none: {:?}", none); // None
}
```

```rust
// 코인 자판기 예제
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 2. 깔끔한 단일 처리: `if let`
여러 가지 경우 중 **오직 한 가지 경우에만 관심이 있고 나머지는 무시하고 싶을 때**, `match` 대신 간결하게 쓸 수 있는 문법 설탕(Syntactic Sugar)입니다.

```rust
let config_max = Some(3u8);

// match를 썼다면:
// match config_max {
//     Some(max) => println!("최대값은 {max}"),
//     _ => (),
// }

// if let으로 쓰면 한눈에 쏙!
if let Some(max) = config_max {
    println!("최대값은 {max}입니다!");
}
```

---

구조체로 데이터를 깔끔하게 설계하고, 열거형과 패턴 매칭으로 누락 없는 분기 처리를 완성했습니다!  
다음 장에서는 Rust에서 코드 재사용성과 다형성을 책임지는 **제네릭(Generics)과 트레이트(Trait)**를 정복해 보겠습니다! 🦀🚀
