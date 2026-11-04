# desktop-01-egui-basics 🦀⚡

> **Rust 초경량 즉시 모드(Immediate Mode) GUI 입문 실습 프로젝트**  
> `eframe` (egui framework) 기반으로 크로스플랫폼 네이티브 윈도우 창을 띄우고, 매 프레임 UI를 평가하는 즉시 모드 멘탈 모델과 필수 인터랙티브 위젯을 정복합니다.

---

## 🌟 주요 기능 & 레이아웃 구성

### 1. ⚡ 실시간 인터랙티브 카운터 (`SidePanel::left`)
- **즉시 모드 원리**: 버튼 클릭 시 이벤트 콜백 대신 `if ui.button(...).clicked()` 문법으로 그 자리에서 즉시 변수 변경
- 증감 단위 슬라이더 (1~20 조절)
- 최근 50개 조작 내역 실시간 스크롤 히스토리 로깅 (`egui::ScrollArea`)

### 2. 📋 인터랙티브 위젯 쇼케이스 (`CentralPanel`)
- **텍스트 입력**: `ui.text_edit_singleline(&mut self.user.name)`
- **콤보 박스**: `egui::ComboBox`를 통한 직무 선택 (개발자, 디자이너, PM, DevOps)
- **정수 슬라이더**: `egui::Slider` 기반 경력 연차 조절
- **체크박스**: 불리언 값 직결 토글
- **볼륨 슬라이더 & 프로그레스 바**: `0.0..=100.0%` 슬라이더와 시각적 진행 게이지 연동
- **실시간 파생 상태 카드**: 이름과 직무에 따른 동적 환영 문구 및 연차별 숙련도 뱃지 실시간 표시

### 3. 🎨 상단 & 하단 패널
- **Top Panel**: 다크/라이트 테마 원클릭 스위치 (`ctx.set_visuals`), 정보 모달 토글
- **Bottom Panel**: 상태 표시줄, 누적 렌더 프레임 수 실시간 갱신 카운터
- **Modal Dialog (`egui::Window`)**: 팝업 창 열기/닫기

---

## 🛠️ 실행 및 테스트 방법

### 1. 네이티브 윈도우 창 실행
```powershell
cd H:\lee\rust-sandbox\projects\desktop\desktop-01-egui-basics
cargo run
```
실행 즉시 윈도우 바탕화면에 880x620 크기의 네이티브 GPU 가속 윈도우 창이 뜹니다.

### 2. 단위 테스트 검증
```powershell
cargo test
```
5개 단위 테스트 (프로필 기본값, 인사말 포맷팅, 숙련도 계산, 이름 유효성 검증, 카운터 히스토리 50개 제한) 자동 통과

