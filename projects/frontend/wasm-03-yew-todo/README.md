# 🦀 Sample 03: Yew Todo 애플리케이션 (wasm-03-yew-todo)

React 스타일의 컴포넌트 간 양방향 이벤트 통신, 제어 컴포넌트(Controlled Input) 폼 처리, 그리고 브라우저 **`LocalStorage`** 연동을 통한 상태 영구 보존을 학습하는 완성형 Todo 웹 애플리케이션입니다.

---

## 📌 주요 학습 내용

1. **양방향 컴포넌트 이벤트 통신**:
   - 부모(`App`) ➡️ 자식(`TodoItem`, `TodoHeader`, `TodoFilterBar`)으로의 단방향 데이터 흐름(Props)
   - 자식 ➡️ 부모로의 콜백 발송(`cb.emit(payload)`)을 통한 상태 갱신
2. **제어 컴포넌트 (Controlled Component) 폼 제어**:
   - `oninput`과 `onkeydown` 이벤트를 실시간으로 Rust 메모리에 동기화
   - Enter 키 입력 시 자동 제출 및 입력창 초기화
3. **인라인 더블클릭 수정 모드**:
   - 더블클릭 또는 수정 버튼 클릭 시 인라인 편집 모드로 전환
   - Enter/Blur로 저장, Escape로 취소
4. **브라우저 `LocalStorage` 연동 (`gloo-storage`)**:
   - `use_effect_with`를 활용하여 할 일 목록이 수정될 때마다 LocalStorage에 JSON 직렬화 자동 저장
   - 페이지 새로고침이나 브라우저 재실행 후에도 데이터가 안전하게 복원
5. **필터링 & 일괄 처리 로직**:
   - 전체(All) / 진행 중(Active) / 완료됨(Completed) 다이내믹 필터링
   - 전체 일괄 완료/해제 토글 및 완료 항목 일괄 삭제

---

## 🏃‍♂️ 실행 방법 (로컬 개발 서버)

본 디렉터리(`projects/frontend/wasm-03-yew-todo`)에서 다음 명령어를 실행합니다:

```bash
trunk serve --port 9092
```

- 웹 브라우저에서 `http://127.0.0.1:9092` 접속
- 소스 코드 수정 시 브라우저가 자동으로 핫 리로딩(Live Reload)됩니다.

---

## 🧪 비즈니스 로직 단위 테스트

UI와 분리된 순수 Rust TodoList 상태 전이 로직을 검증합니다:

```bash
cargo test -p wasm-03-yew-todo
```

---

## 🏗️ 프로덕션 배포용 빌드

```bash
trunk build --release
```
- `dist/` 폴더에 번들링된 `.wasm` 바이너리와 정적 파일들이 생성됩니다.

