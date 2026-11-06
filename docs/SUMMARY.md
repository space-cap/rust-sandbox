# 🦀 페리스와 함께 떠나는 러스트(Rust) 완벽 입문 가이드

## 목차 (Table of Contents)

- [00. 시작하며: 왜 Rust인가?](00_intro.md)
  - [0.1 왜 전 세계 개발자들은 Rust에 열광할까요?](00_intro.md#01-왜-전-세계-개발자들은-rust에-열광할까요)
  - [0.2 Rust는 누가, 왜 만들었을까요?](00_intro.md#02-rust는-누가-왜-만들었을까요-21층-계단과-그레이든-호어)
  - [0.3 마스코트 페리스(Ferris) 이야기](00_intro.md#03-마스코트-페리스ferris-이야기)
  - [0.4 이 책을 효과적으로 공부하는 방법](00_intro.md#04-이-책을-효과적으로-공부하는-방법)

- [01. 개발 환경 설정과 첫 발걸음](01_setup_and_first_step.md)
  - [1.1 러스트 설치 (rustup)](01_setup_and_first_step.md#11-러스트-설치-rustup)
  - [1.2 최고의 조수: Cargo 도구 다루기](01_setup_and_first_step.md#12-최고의-조수-cargo-도구-다루기)
  - [1.3 세상에서 가장 안전한 "Hello, World!"](01_setup_and_first_step.md#13-세상에서-가장-안전한-hello-world)
  - [1.4 추천 개발 환경 (VS Code & Rust Analyzer)](01_setup_and_first_step.md#14-추천-개발-환경-vs-code--rust-analyzer)

- [02. 기본 문법: 변수, 타입, 제어 흐름](02_basic_syntax.md)
  - [2.1 왜 기본이 불변(Immutable)일까? (`let`과 `mut`)](02_basic_syntax.md#21-왜-기본이-불변immutable일까-let과-mut)
  - [2.2 변수 가리기(Shadowing)와 상수(const)](02_basic_syntax.md#22-변수-가리기shadowing와-상수const)
  - [2.3 핵심 데이터 타입 (스칼라 & 복합 타입)](02_basic_syntax.md#23-핵심-데이터-타입-스칼라--복합-타입)
  - [2.4 함수와 표현식(Expression) vs 구문(Statement)](02_basic_syntax.md#24-함수와-표현식expression-vs-구문statement)
  - [2.5 흐름 제어 (`if`, `loop`, `while`, `for`)](02_basic_syntax.md#25-흐름-제어-if-loop-while-for)

- [03. Rust의 심장: 소유권(Ownership)과 빌림(Borrowing)](03_ownership_and_borrow.md)
  - [3.1 가비지 컬렉터 없이 메모리를 비운다고? (스택 vs 힙)](03_ownership_and_borrow.md#31-가비지-컬렉터-없이-메모리를-비운다고-스택-vs-힙)
  - [3.2 소유권의 3대 황금률](03_ownership_and_borrow.md#32-소유권의-3대-황금률)
  - [3.3 이동(Move)과 깊은 복사(Clone), 스택 복사(Copy)](03_ownership_and_borrow.md#33-이동move과-깊은-복사clone-스택-복사copy)
  - [3.4 빌림(Borrowing): 빌려 쓰되 망가뜨리지 않기](03_ownership_and_borrow.md#34-빌림borrowing-빌려-쓰되-망가뜨리지-않기)
  - [3.5 슬라이스(Slice): 안전한 부분 참조](03_ownership_and_borrow.md#35-슬라이스slice-안전한-부분-참조)

- [04. 데이터 묶기와 패턴 매칭: 구조체와 열거형](04_struct_and_enum.md)
  - [4.1 구조체(Struct): 나만의 데이터 설계도](04_struct_and_enum.md#41-구조체struct-나만의-데이터-설계도)
  - [4.2 메서드 정의하기 (`impl`과 `self`)](04_struct_and_enum.md#42-메서드-정의하기-impl과-self)
  - [4.3 열거형(Enum): 상태와 값을 하나로!](04_struct_and_enum.md#43-열거형enum-상태와-값을-하나로)
  - [4.4 10억 달러짜리 실수를 없앤 `Option<T>`](04_struct_and_enum.md#44-10억-달러짜리-실수를-없앤-optiont)
  - [4.5 강력한 무기: `match`와 `if let` 패턴 매칭](04_struct_and_enum.md#45-강력한-무기-match와-if-let-패턴-매칭)

- [05. 제네릭, 트레이트, 그리고 라이프타임 맛보기](05_generics_and_traits.md)
  - [5.1 제네릭(Generics): 코드 중복 없이 유연하게](05_generics_and_traits.md#51-제네릭generics-코드-중복-없이-유연하게)
  - [5.2 트레이트(Trait): 공통의 약속과 인터페이스](05_generics_and_traits.md#52-트레이트trait-공통의-약속과-인터페이스)
  - [5.3 트레이트 바운드와 `#[derive]`](05_generics_and_traits.md#53-트레이트-바운드와-derive)
  - [5.4 라이프타임(Lifetime) 기초: 참조의 유효기간 이해하기](05_generics_and_traits.md#54-라이프타임lifetime-기초-참조의-유효기간-이해하기)

- [06. 에러 처리와 안전한 코딩](06_error_handling.md)
  - [6.1 패닉! 회복 불가능한 에러 (`panic!`)](06_error_handling.md#61-패닉-회복-불가능한-에러-panic)
  - [6.2 회복 가능한 에러: `Result<T, E>`](06_error_handling.md#62-회복-가능한-에러-resultt-e)
  - [6.3 에러 전파의 마법 열쇠: `?` 연산자](06_error_handling.md#63-에러-전파의-마법-열쇠--연산자)
  - [6.4 `unwrap`은 언제 쓰고 언제 피해야 할까?](06_error_handling.md#64-unwrap은-언제-쓰고-언제-피해야-할까)

- [07. 모듈과 패키지 생태계](07_modules_and_crates.md)
  - [7.1 패키지, 크레이트, 모듈 한눈에 정리](07_modules_and_crates.md#71-패키지-크레이트-모듈-한눈에-정리)
  - [7.2 모듈 분리와 공개 범위 (`mod`, `use`, `pub`)](07_modules_and_crates.md#72-모듈-분리와-공개-범위-mod-use-pub)
  - [7.3 crates.io에서 외부 라이브러리 가져다 쓰기](07_modules_and_crates.md#73-cratesio에서-외부-라이브러리-가져다-쓰기)

- [08. 실전 미니 프로젝트: 나만의 CLI Todo 매니저](08_mini_project.md)
  - [8.1 프로젝트 기획 및 요구사항](08_mini_project.md#81-프로젝트-기획-및-요구사항)
  - [8.2 데이터 모델링 및 파일 입출력 설계](08_mini_project.md#82-데이터-모델링-및-파일-입출력-설계)
  - [8.3 명령줄 인자 파싱 및 기능 구현](08_mini_project.md#83-명령줄-인자-파싱-및-기능-구현)
  - [8.4 완성된 Todo 앱 실행해보기](08_mini_project.md#84-완성된-todo-앱-실행해보기)

- [09. 부록: Rust 치트시트 & 추천 로드맵](09_appendix_cheatsheet.md)
  - [9.1 꼭 기억해야 할 Cargo 필수 명령어 치트시트](09_appendix_cheatsheet.md#91-꼭-기억해야-할-cargo-필수-명령어-치트시트)
  - [9.2 초보자가 가장 많이 만나는 컴파일 에러 Top 5와 해결법](09_appendix_cheatsheet.md#92-초보자가-가장-많이-만나는-컴파일-에러-top-5와-해결법)
  - [9.3 다음 걸음을 위한 추천 학습 로드맵](09_appendix_cheatsheet.md#93-다음-걸음을-위한-추천-학습-로드맵)



---

## 🚀 실전 분야별 심화 마스터 시리즈

### 🖥️ 데스크톱 GUI 앱 개발 마스터
- [로드맵: Rust 데스크톱 GUI 앱 개발 마스터 플랜](desktop/desktop_learning_plan.md)
  - [01장. Rust 데스크톱 생태계 지도 & 4대 패러다임 심층 분석](desktop/01_desktop_gui_landscape.md)
  - [02장. 데스크톱 앱 개발 필수 메커니즘과 동작 원리](desktop/02_gui_core_mechanisms.md)
  - [03장. 실무 1순위: Tauri 2.0 아키텍처 & IPC 심층 분석](desktop/03_tauri_deep_dive.md)
  - [04장. 순수 Rust GUI의 세계: egui (즉시 모드) & Iced (보유 모드) 실전 패턴](desktop/04_pure_rust_gui_egui_iced.md)
  - [05장. 배포 및 프로덕션 완벽 가이드 (설치 파일, 자동 업데이트, 코드 서명)](desktop/05_packaging_and_distribution.md)

### 🎨 프론트엔드 & WebAssembly 마스터
- [로드맵: Rust Wasm & 프론트엔드 마스터 플랜](frontend/frontend_learning_plan.md)
  - [01장. Wasm 기초 & Trunk 번들러 환경 세팅](frontend/01_wasm_and_trunk_intro.md)
  - [02장. Yew 컴포넌트 모델 & use_state 상태 관리](frontend/02_yew_component_and_state.md)
  - [03장. 실전 Yew Todo 웹앱 & LocalStorage 영구 보존](frontend/03_yew_interactive_todo.md)
  - [04장. Rust 백엔드(Axum)와의 풀스택 연동 (End-to-End)](frontend/04_fullstack_axum_integration.md)
  - [05장. JavaScript 대비 20배 빠른 고성능 그래픽 연산 (Canvas 2D)](frontend/05_high_perf_canvas.md)
  - [06장. 차세대 초고속 프레임워크 Leptos & 정적 배포](frontend/06_leptos_and_deployment.md)

### ⚙️ 백엔드 & 비동기 런타임 마스터
- [로드맵: Rust 고성능 백엔드 마스터 플랜](backend/backend_learning_plan.md)
  - [01장. 비동기 Rust의 원리와 Axum 시작하기](backend/01_tokio_and_axum_intro.md)
  - [02장. 견고한 RESTful API 아키텍처와 상태 공유](backend/02_rest_api_architecture.md)
  - [03장. SQLx와 PostgreSQL로 완전 비동기 데이터베이스 다루기](backend/03_sqlx_database_guide.md)
  - [04장. 엔터프라이즈급 인증(JWT), 보안, 그리고 미들웨어](backend/04_auth_and_middleware.md)
  - [05장. 실시간 양방향 통신: WebSocket & Tokio Broadcast](backend/05_realtime_websocket.md)
  - [06장. 벤치마크, 프로파일링, 그리고 초경량 Docker 배포](backend/06_benchmark_and_docker.md)

### 🎮 차세대 게임 엔진 Bevy 마스터 (ECS & 2D 인터랙티브)
- [로드맵: Rust Bevy 게임 개발 마스터 플랜](game/game_learning_plan.md)
  - [01장. Bevy 아키텍처 & 왜 객체지향(OOP)을 버리고 ECS로 가는가?](game/01_bevy_and_ecs_philosophy.md)
  - [02장. 2D 스프라이트 렌더링, 텍스처 아틀라스 & 애니메이션](game/02_2d_rendering_and_assets.md)
  - [03장. 탄막 슈팅 & AABB 충돌 감지 물리](game/03_physics_and_collision.md)
  - [04장. 게임 라이프사이클(States), BGM & UI HUD](game/04_states_sound_and_ui.md)
  - [05장. 파티클 폭발 이펙트 & 카메라 쉐이크 (Juice & Polish)](game/05_game_juice_and_effects.md)
  - [06장. 종합 실전: 2D 서바이벌 아레나 (Vampire Survivor 류)](game/06_full_survival_game.md)
