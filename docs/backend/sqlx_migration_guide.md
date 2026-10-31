# 🗄️ SQLx 마이그레이션 완전 정복 가이드 (Deep Dive)

> *"서버가 구동되는 0.001초 만에 일어나는 DB 스키마 자동 동기화의 원리와 실무 팁을 정리한 핵심 가이드입니다."*

---

## 1. 코드 레벨 자동 마이그레이션이란?

웹 개발(Node.js, Python, Java)을 할 때 데이터베이스 스키마가 변경되면 다음과 같은 번거로운 과정을 거쳐야 했습니다:

- **기존 방식의 불편함**:
  - `npx prisma migrate dev`, `python manage.py migrate` 등 별도의 터미널 명령어를 손수 입력해야 함.
  - 새 팀원이 프로젝트를 클론받았거나 Docker 컨테이너로 배포할 때, 깜빡하고 마이그레이션을 안 돌리면 서버가 즉시 다운됨(`Column/Table not found`).
  - CI/CD 파이프라인이나 `entrypoint.sh`에 마이그레이션 스크립트를 억지로 끼워 넣어야 함.

### 🦀 Rust SQLx의 해답: 서버 내장 마이그레이션
Rust의 SQLx는 **마이그레이션 엔진이 서버 프로그램 바이너리 자체에 내장(Built-in)**되어 있습니다.

```rust
// 서버 기동 시 단 한 줄로 migrations/ 폴더의 모든 변경 사항을 즉시 동기화!
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

서버 실행 파일(`.exe` 또는 Linux 바이너리) 하나만 배포하여 실행(`cargo run`)하면, **외부 CLI 도구나 수동 작업 없이 DB 스키마 생성 및 업데이트까지 100% 자동으로 완료**됩니다.

---

## 2. 동작 원리: 어떻게 변경을 알고 자동으로 반영할까?

서버가 켜지는 첫 **0.001초** 동안 백그라운드에서 다음 과정이 자동으로 진행됩니다:

```mermaid
sequenceDiagram
    participant S as Axum 서버 (cargo run)
    participant M as migrations/ 폴더
    participant DB as 데이터베이스 (_sqlx_migrations)

    S->>DB: 1. "_sqlx_migrations 장부 테이블 있어?" (없으면 자동 생성)
    S->>M: 2. "migrations 폴더에 무슨 SQL 파일들이 있지?"
    M-->>S: 파일 목록 반환 (예: 20261101_init.sql)
    S->>DB: 3. "이 파일들 이미 실행한 적 있어?"
    alt 4-A. 아직 실행 안 된 새 파일인 경우
        S->>DB: 🚀 SQL 파일 즉시 실행! (테이블/컬럼 추가)
        S->>DB: 📝 장부에 [버전, 설명, 실행시간, 체크섬] 기록
    alt 4-B. 이미 지난번에 실행 완료된 파일인 경우
        S->>DB: ⏭️ 0.0001초 만에 스킵!
    end
    S->>S: 5. 🚀 웹 서버 포트 바인딩 및 서비스 시작
```

---

## 3. 마이그레이션 실행 이력은 어디에 저장될까?

마이그레이션 실행 이력은 파일 시스템이나 외부에 저장되지 않고, **"데이터베이스(DB) 자체 내부"**에 특별한 장부 테이블 형태로 보관됩니다.

- **테이블 이름**: **`_sqlx_migrations`**
  *(참고: Prisma는 `_prisma_migrations`, TypeORM은 `migrations`, Django는 `django_migrations`)*

### 실제 DB 내부의 레코드 구조 (실제 확인 데이터)

| 컬럼명 | 실제 저장된 예시 값 | 역할 및 의미 |
| :--- | :--- | :--- |
| **`version`** | `20261101` | 파일명 앞부분의 고유 식별 번호 (실행 순서 기준) |
| **`description`** | `init` | 파일명 뒷부분의 작업 설명 문자열 |
| **`installed_on`** | `2026-10-31 19:16:37` | 해당 마이그레이션이 실제 적용된 정확한 시각 |
| **`success`** | `1` (True) | 성공적으로 완료되었는지 여부 (1=성공, 0=실패) |
| **`checksum`** | `03f9a8...` (SHA 해시) | 파일 내용의 디지털 지문 (위변조 방지) |
| **`execution_time`** | `12` (ms) | 쿼리 실행에 소요된 시간 (밀리초) |

### 🛡️ 체크섬(Checksum)의 위변조 방지 기능
`_sqlx_migrations` 테이블은 각 파일 내용의 해시값(지문)을 보관합니다.  
만약 누군가 **이미 배포되어 실행이 끝난 SQL 파일의 내용을 나중에 몰래 수정**하면, 서버가 켜질 때 SQLx가 지문을 대조해 보고:
> *"⚠️ 경고: 이미 적용된 `20261101_init.sql` 파일의 내용이 지난번 실행 당시의 체크섬과 일치하지 않습니다!"*

라며 즉시 서버 기동을 중단하여 **DB 스키마가 꼬이는 휴먼 에러를 원천 차단**합니다.

---

## 4. 마이그레이션 파일 명명 규칙 (Naming Convention)

SQLx는 파일 이름을 기준으로 **실행 순서**와 **내용**을 파악하므로 아래 형식을 반드시 지켜야 합니다.

### 📌 공식 명명 규칙
```text
{숫자버전}_{설명}.sql
```

```
migrations/
├── 20261101_init.sql
├── 20261102_add_users_table.sql
└── 20261103_add_priority_to_todos.sql
```

1. **`{숫자버전}` (Version)**:
   - 반드시 **숫자(정수, Integer)**여야 합니다.
   - SQLx는 이 **숫자가 작은 것부터 큰 순서대로(오름차순)** 쿼리를 실행합니다.
   - **순번 스타일 (`01`, `02`, `03`)**: 개인 프로젝트나 간단한 프로토타입에 직관적.
   - **타임스탬프 스타일 (`YYYYMMDDHHMMSS`)**: **실무 및 팀 협업 시 강력 권장!** 여러 개발자가 동시에 마이그레이션을 만들어도 번호 충돌이 발생하지 않음.
2. **`_` (언더스코어)**:
   - 버전 번호와 설명을 구분하는 필수 구분자입니다.
3. **`{설명}` (Description)**:
   - 어떤 변경인지 알 수 있도록 스네이크 케이스(`snake_case`)로 작성합니다. (장부 테이블의 `description` 컬럼에 저장됨)

---

## 5. 롤백(Rollback)을 지원하는 Up / Down 방식

단순 마이그레이션은 `.sql` 파일 하나면 충분하지만, 변경 사항을 되돌리는 **롤백(Rollback)** 기능까지 완벽하게 지원하고 싶다면 `.up.sql`과 `.down.sql` 쌍으로 구성할 수 있습니다:

```
migrations/
├── 20261101_init.up.sql    # 적용할 때 실행 (CREATE TABLE todos ...)
└── 20261101_init.down.sql  # 되돌릴 때 실행 (DROP TABLE todos ...)
```

---

## 6. 실무 꿀팁: `sqlx-cli`를 활용한 자동 파일 생성

파일명을 직접 손으로 타이핑하기 번거롭다면, 공식 도구인 `sqlx-cli`를 설치하여 명령어 한 줄로 생성할 수 있습니다:

```bash
# 1. 도구 설치 (최초 1회)
cargo install sqlx-cli

# 2. 마이그레이션 파일 자동 생성
cargo sqlx migrate add add_priority_column
```

실행하면 현재 시각 타임스탬프가 붙은 새 파일이 `migrations/` 폴더에 즉시 자동 생성됩니다:
```text
Creating migrations/20261101043512_add_priority_column.sql
```
이 파일 안에 적용할 DDL SQL문만 적어주면 끝납니다! 🚀

