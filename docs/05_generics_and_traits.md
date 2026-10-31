# 05. 제네릭, 트레이트, 그리고 라이프타임 맛보기 🧬

> *"추상화의 자유로움과 제로 코스트 성능의 만남. 러스트의 유연함을 책임지는 핵심 기둥들입니다."*

---

## 5.1 제네릭(Generics): 코드 중복 없이 유연하게

만약 두 숫자의 좌표를 담는 `Point` 구조체를 만든다고 가정해 봅시다.  
정수(`i32`) 좌표도 필요하고 실수(`f64`) 좌표도 필요하다면, 구조체를 두 개 따로 만들어야 할까요?  
아닙니다! **제네릭(Generics)**을 사용하면 하나의 정의로 모든 타입을 포용할 수 있습니다.

```rust
// T는 어떤 구체적인 타입(Type)이든 될 수 있는 자리표시자(Placeholder)입니다!
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };       // Point<i32>로 자동 추론!
    let float_point = Point { x: 1.0, y: 4.0 };     // Point<f64>로 자동 추론!

    println!("정수 점: ({}, {})", integer_point.x, integer_point.y);
    println!("실수 점: ({}, {})", float_point.x, float_point.y);
}
```

### 제네릭의 성능 비밀: 단형성화 (Monomorphization)
"제네릭을 쓰면 런타임에 타입을 확인하느라 느려지지 않나요?"  
전혀 그렇지 않습니다! Rust 컴파일러는 빌드 시점에 코드를 분석하여 실제로 사용된 타입(예: `i32`, `f64`)별로 별도의 전용 기계어 코드를 컴파일 시점에 미리 복제해서 생성합니다.  
이를 **단형성화(Monomorphization)**라고 부르며, **런타임 오버헤드가 제로(0)**입니다!

---

## 5.2 트레이트(Trait): 공통의 약속과 인터페이스

**트레이트(Trait)**는 특정 타입이 공통으로 가져야 하는 '행동(메서드)'을 정의하는 일종의 규약(약속)입니다.  
다른 언어(Java, C#, Go)의 **인터페이스(Interface)**와 유사하지만 훨씬 유연합니다.

```rust
// 요약할 수 있는 능력을 정의하는 트레이트
pub trait Summary {
    fn summarize(&self) -> String;

    // 기본 구현(Default Implementation)을 둘 수도 있습니다!
    fn default_badge(&self) -> String {
        String::from("[공식 뱃지]")
    }
}

// 뉴스 기사 구조체
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

// NewsArticle에 Summary 트레이트를 구현!
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - by {} ({})", self.headline, self.author, self.location)
    }
}

// SNS 트윗 구조체
pub struct Tweet {
    pub username: String,
    pub content: String,
}

// Tweet에도 Summary 트레이트를 구현!
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("ferris"),
        content: String::from("러스트 배울수록 너무 재밌다! 🦀"),
    };

    println!("새 트윗: {}", tweet.summarize());
    println!("뱃지: {}", tweet.default_badge());
}
```

---

## 5.3 트레이트 바운드와 `#[derive]`

트레이트는 함수의 매개변수 타입을 제한할 때 강력한 힘을 발휘합니다.  
"아무 타입이나 다 받는 게 아니라, **`Summary` 트레이트를 구현한 녀석만** 받겠다!"라고 조건을 거는 것을 **트레이트 바운드(Trait Bound)**라고 합니다.

```rust
// Summary 트레이트를 구현한 타입이라면 무엇이든 출력 가능!
pub fn notify(item: &impl Summary) {
    println!("속보 알림! {}", item.summarize());
}
```

### 마법 같은 매크로: `#[derive]`
Rust를 코딩하다 보면 구조체 위에 `#[derive(Debug, Clone, PartialEq)]` 같은 주석 같은 문장을 자주 보게 됩니다.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Player {
    name: String,
    score: u32,
}
```

이것은 컴파일러에게 **"이 구조체에 필요한 자주 쓰는 표준 트레이트들을 알아서 자동으로 구현해줘!"**라고 지시하는 문법입니다.
- **`Debug`**: `println!("{:?}", player);` 형태로 디버그용 출력을 가능하게 해줍니다.
- **`Clone`**: `.clone()`으로 메모리를 쉽게 복제할 수 있게 해줍니다.
- **`PartialEq`**: `player1 == player2` 처럼 `==` 비교 연산자를 쓸 수 있게 해줍니다.

---

## 5.4 라이프타임(Lifetime) 기초: 참조의 유효기간 이해하기

Rust를 공부하다 보면 가장 두려워하는 외계어(?) 같은 문법이 바로 작은따옴표 하나가 붙은 **라이프타임(예: `'a`)**입니다.  
하지만 원리만 알면 전혀 어렵지 않습니다!

### 왜 라이프타임이 필요할까요?
모든 참조(포인터)는 **"가리키고 있는 원본 데이터가 메모리에서 사라지기 전까지만"** 살아있어야 안전합니다. 만약 원본이 이미 소멸했는데 엉뚱한 빈 주소를 가리키고 있다면 치명적인 버그(Dangling Reference)가 됩니다.

```rust
// ❌ 컴파일되지 않는 위험한 코드의 예:
fn dangle() -> &String {
    let s = String::from("hello"); // s는 이 함수 안에서 생성됨
    &s // s의 참조를 반환하려고 시도!
} // ⚠️ 함수가 끝나는 순간 s는 메모리에서 소멸(drop)! 참조는 허공을 가리키게 됨!
```
Rust 컴파일러의 **빌림 체커(Borrow Checker)**는 이런 코드를 발견하면 즉시 빌드를 중단시킵니다.

### 라이프타임 명시 문법 (`'a`)
보통의 경우 컴파일러가 알아서 참조의 수명을 계산해 줍니다(라이프타임 생략 규칙).  
하지만 두 개 이상의 참조를 함수 매개변수로 받고 그중 하나를 반환할 때, 컴파일러가 어느 참조가 반환되는지 확신할 수 없으면 **"이 참조들의 수명이 서로 얼마나 연결되어 있는지 힌트를 달라"**고 요청합니다.

```rust
// 'a는 "x와 y 중 더 짧게 살아남는 녀석의 수명만큼 반환값의 참조도 유효하다"는 약속입니다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("긴 문자열입니다");
    let string2 = "짧음";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {result}");
}
```

> [!TIP]
> 라이프타임 주석(`'a`)은 **어떤 변수의 수명을 억지로 늘려주는 것이 아닙니다.**  
> 단지 컴파일러에게 **"이 참조들의 유효 기간 관계가 이러하니 안심하고 검사해 줘!"**라고 설명해 주는 메타데이터일 뿐입니다.

---

제네릭, 트레이트, 그리고 라이프타임의 핵심 원리까지 정복했습니다!  
이제 06장에서는 예외(Exception) 없이 우아하고 안전하게 오류를 다루는 **에러 처리(Error Handling)** 기법을 배워보겠습니다! 🦀🚀
