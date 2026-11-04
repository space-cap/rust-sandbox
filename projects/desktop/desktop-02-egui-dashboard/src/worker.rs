use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::models::{SystemMetricPoint, WorkerCommand, WorkerEvent};

/// 백그라운드 워커 관리 핸들
pub struct WorkerHandle {
    pub cmd_tx: Sender<WorkerCommand>,
    pub event_rx: Receiver<WorkerEvent>,
}

impl WorkerHandle {
    /// 백그라운드 모니터링 및 연산 워커 스레드를 시작합니다.
    pub fn spawn() -> Self {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<WorkerCommand>();
        let (event_tx, event_rx) = std::sync::mpsc::channel::<WorkerEvent>();

        std::thread::Builder::new()
            .name("desktop-dashboard-worker".to_string())
            .spawn(move || {
                run_worker_loop(cmd_rx, event_tx);
            })
            .expect("Failed to spawn background worker thread");

        Self { cmd_tx, event_rx }
    }
}

fn run_worker_loop(cmd_rx: Receiver<WorkerCommand>, event_tx: Sender<WorkerEvent>) {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    // CPU 점유율 최초 샘플링 (첫 호출 시에는 0.0이 나올 수 있으므로 초기 refresh)
    sys.refresh_cpu_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();

    let mut is_monitoring = true;
    let mut interval = Duration::from_millis(500);
    let mut last_metric_time = Instant::now();
    let mut metric_index = 0u64;

    let heavy_task_running = Arc::new(AtomicBool::new(false));

    loop {
        // 1. UI로부터 들어온 명령 처리 (논블로킹 대기)
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                WorkerCommand::StartMonitoring => {
                    is_monitoring = true;
                }
                WorkerCommand::PauseMonitoring => {
                    is_monitoring = false;
                }
                WorkerCommand::SetIntervalMs(ms) => {
                    interval = Duration::from_millis(ms.max(100));
                }
                WorkerCommand::StartHeavyTask { target_iterations } => {
                    if !heavy_task_running.load(Ordering::SeqCst) {
                        heavy_task_running.store(true, Ordering::SeqCst);
                        let flag = Arc::clone(&heavy_task_running);
                        let tx = event_tx.clone();

                        // 무거운 연산은 별도 스레드로 격리하여 모니터링 주기와 독립적으로 실행
                        std::thread::spawn(move || {
                            run_heavy_computation(target_iterations, flag, tx);
                        });
                    }
                }
                WorkerCommand::CancelHeavyTask => {
                    heavy_task_running.store(false, Ordering::SeqCst);
                }
            }
        }

        // 2. 주기적 메트릭 수집 및 UI 전송
        if is_monitoring && last_metric_time.elapsed() >= interval {
            sys.refresh_cpu_all();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage();
            let total_mem = sys.total_memory() as f64 / (1024.0 * 1024.0); // bytes to MB
            let used_mem = sys.used_memory() as f64 / (1024.0 * 1024.0);

            let point = SystemMetricPoint::new(metric_index, cpu_usage, used_mem, total_mem);
            metric_index += 1;

            if event_tx.send(WorkerEvent::MetricUpdate(point)).is_err() {
                // UI가 종료되어 채널이 끊긴 경우 워커 스레드 종료
                break;
            }
            last_metric_time = Instant::now();
        }

        // 과도한 CPU 점유 방지 (10ms 슬립)
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// UI 프리징 없이 백그라운드에서 실행되는 CPU 연산 시뮬레이터 (진행률 실시간 보고)
fn run_heavy_computation(
    iterations: u64,
    running_flag: Arc<AtomicBool>,
    event_tx: Sender<WorkerEvent>,
) {
    let start_time = Instant::now();
    let report_chunk = (iterations / 100).max(1);
    let mut prime_count = 0u64;

    for i in 1..=iterations {
        // 취소 플래그 확인
        if !running_flag.load(Ordering::SeqCst) {
            let _ = event_tx.send(WorkerEvent::HeavyTaskCancelled);
            return;
        }

        // 시뮬레이션: 소수 판별 연산
        if is_prime(i) {
            prime_count += 1;
        }

        // 1% 단위 또는 일정 주기마다 진행률 전송
        if i % report_chunk == 0 || i == iterations {
            let progress = i as f32 / iterations as f32;
            let current_step = format!("{i} / {iterations} 연산 중 (발견된 소수: {prime_count}개)");
            let _ = event_tx.send(WorkerEvent::HeavyTaskProgress {
                progress,
                current_step,
            });
        }

        // 데스크톱 반응성 테스트용 아주 미세한 지연
        if i % 10_000 == 0 {
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    running_flag.store(false, Ordering::SeqCst);
    let elapsed_ms = start_time.elapsed().as_millis();
    let summary = format!(
        "연산 완료! 총 {iterations}개 숫자 탐색 중 {prime_count}개 소수 발견 (소요 시간: {elapsed_ms}ms)"
    );
    let _ = event_tx.send(WorkerEvent::HeavyTaskDone {
        elapsed_ms,
        summary,
    });
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

