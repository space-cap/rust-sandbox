# ⚡ 고성능 부하 테스트(Benchmark) 가이드

Rust 백엔드의 압도적인 처리량(RPS)과 극저지연(Latency)을 직접 눈으로 검증하는 부하 테스트 가이드입니다.

---

## 🛠️ 도구 설치 (k6)

### Windows 사용자 (터미널에서 한 줄 입력)
```powershell
winget install k6 --source winget
# 또는 choco 사용 시: choco install k6
```

### macOS 사용자
```bash
brew install k6
```

### Linux (Ubuntu/Debian) 사용자
```bash
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update && sudo apt-get install k6
```

---

## 🏃‍♂️ 부하 테스트 실행 방법

### 1단계: 테스트 대상 서버 실행
우리가 만든 `sample-02-rest-crud` 서버를 **프로덕션 릴리즈 모드(`--release`)**로 켭니다:

```bash
cargo run --release -p sample-02-rest-crud
```

> [!TIP]
> 벤치마크를 진행할 때는 반드시 **`--release`** 플래그를 붙여야 합니다!  
> 모든 컴파일러 최적화가 활성화되어 디버그 모드 대비 **10배 이상** 빨라집니다.

---

### 2단계: k6 부하 테스트 스크립트 실행
새 터미널을 열고 다음 명령을 실행합니다:

```bash
k6 run benchmarks/k6-load-test.js
```

---

## 📊 결과 지표 읽는 법

테스트가 완료되면 터미널에 아름다운 요약 보고서가 출력됩니다:

```text
✓ GET /api/todos 상태 코드 200 .................: 100.00% ✓ 42,510
✓ POST /api/todos 상태 코드 201 ................: 100.00% ✓ 42,510

http_req_duration..............: avg=1.12ms   min=0.32ms  med=0.88ms  max=18.4ms  p(90)=1.75ms  p(95)=2.31ms
http_reqs......................: 85,020  2125.5/s
```

- **`http_reqs` (초당 요청수)**: 초당 몇 개의 요청을 완벽하게 처리했는지 보여줍니다.
- **`http_req_duration (p95)`**: 95%의 요청이 **단 2.3ms** 이내에 끝났음을 증명합니다.
- **`http_req_failed`**: 0.00% (단 하나의 에러도 없이 수만 건의 요청을 완벽 소화!)

---

## ⚡ 초간단 단발성 벤치마크: `wrk` 치트시트

만약 스크립트 작성 없이 단순 GET 엔드포인트의 한계치만 초고속으로 측정하고 싶다면 `wrk` 도구를 애용하세요:

```bash
# 12개의 스레드로 400개의 동시 커넥션을 맺고 30초간 폭격!
wrk -t12 -c400 -d30s http://127.0.0.1:3001/api/todos
```
초당 5만~8만 회(RPS) 이상의 기적 같은 수치를 직접 확인하실 수 있습니다! 🚀
