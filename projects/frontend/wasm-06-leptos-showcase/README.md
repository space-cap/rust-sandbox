# wasm-06-leptos-showcase ⚡

> **차세대 초고속 프레임워크 Leptos 0.7 & 정적 배포 마스터 쇼케이스**  
> SolidJS 스타일의 세분화된 반응성(Fine-Grained Reactivity, Signal)과 무가상 DOM(Zero Virtual DOM) 아키텍처를 직접 체험하고, GitHub Pages / Vercel / Netlify 프로덕션 배포 파이프라인을 구축합니다.

---

## 🌟 주요 기능 및 3대 쇼케이스 탭

### 1. ⚡ 반응성 & 메모 실험실 (`CounterTab`)
- **Signal**: 상태 변경 시 컴포넌트 재실행 없이 바인딩된 텍스트 노드만 핀포인트 갱신
- **Memo**: 의존성 Signal이 바뀔 때만 재연산되는 스마트 파생 상태 (`count * 2`, `count²`, `홀짝 판별`)
- **고빈도 벤치마크**: 1,000회 연속 틱 상태 갱신 속도(ms) 측정 (초당 수십만 회 처리)
- **아키텍처 비교 뷰**: Yew (Virtual DOM Tree Diffing) vs Leptos (Direct DOM Binding)

### 2. 📊 고성능 컬렉션 렌더링 & 실시간 필터 (`FilterTab`)
- **`<For>` 컴포넌트**: 키(Key) 기반 DOM 노드 정밀 재활용
- **실시간 검색 & 카테고리 필터**: Rust Wasm, Fullstack, JavaScript, Tooling 칩 필터링
- **정렬 & 동적 제어**: 추천순(별점) / 이름순 정렬, 실시간 별점 올리기 및 아이템 추가/삭제

### 3. 🚀 정적 배포 완벽 가이드 & 시뮬레이터 (`DeploymentTab`)
- **인터랙티브 준비도 게이지**: 배포 체크리스트 토글에 반응하는 실시간 백분율 게이지
- **배포 5대 필수 항목**: Trunk release 최적화, `wasm-opt -O4`, SPA `404.html` 트릭, MIME 헤더, `.nojekyll`
- **플랫폼별 CI/CD 템플릿**:
  - GitHub Actions (`.github/workflows/deploy.yml`)
  - Vercel (`vercel.json`)
  - Netlify (`netlify.toml`)
  - Docker / Nginx 멀티스테이지 컨테이너

---

## 🛠️ 실행 및 테스트 방법

### 1. 로컬 개발 서버 실행 (Trunk)
```bash
# wasm-06-leptos-showcase 디렉터리에서
trunk serve --port 9095
```
브라우저에서 `http://localhost:9095` 접속

### 2. 순수 Rust 단위 테스트 검증
```bash
cargo test
```
5개 단위 테스트 (파생 연산, 필터링, 정렬, 배포 점수 계산) 자동 통과

### 3. 프로덕션 배포 빌드
```bash
trunk build --release
```
`dist/` 디렉터리에 압축 및 최적화된 정적 웹 에셋 생성
