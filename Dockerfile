# ==============================================================================
# [1단계: 빌더 Stage] - 무거운 Rust 컴파일러 도구로 릴리즈 최적화 빌드
# ==============================================================================
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/app

# 빌드 캐싱 최적화를 위해 프로젝트 전체 복사
COPY . .

# 프로덕션용 최적화 릴리즈 빌드 (--release)
# (예시: sample-02-rest-crud 패키지 빌드)
ARG PACKAGE_NAME=sample-02-rest-crud
RUN cargo build --release -p ${PACKAGE_NAME} && \
    cp /usr/src/app/target/release/${PACKAGE_NAME} /usr/src/app/server_bin

# ==============================================================================
# [2단계: 러너 Stage] - 단 하나의 바이너리만 탑재하는 초경량(15~25MB) 컨테이너
# ==============================================================================
FROM debian:bookworm-slim AS runner

# 보안 강화를 위한 비루트(non-root) 시스템 유저 생성
RUN useradd -u 10001 -m -s /bin/false appuser

WORKDIR /app

# 빌더 단계에서 컴파일된 순수 기계어 바이너리만 쏙 복사!
COPY --from=builder /usr/src/app/server_bin /app/server

# 파일 소유권 변경 및 비루트 유저로 전환
RUN chown -R appuser:appuser /app
USER appuser

# 서비스 포트 개방 (sample-02 기본 포트: 3001)
EXPOSE 3001

# 컨테이너 시작 명령
CMD ["/app/server"]
