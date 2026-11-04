use desktop_02_egui_dashboard::font::setup_korean_and_emoji_fonts;
use desktop_02_egui_dashboard::models::{
    DashboardTab, MetricHistory, SystemMetricPoint, WorkerCommand,
};

#[test]
fn test_metric_point_creation() {
    let point = SystemMetricPoint::new(0, 45.5, 8192.0, 16384.0);
    assert_eq!(point.index, 0);
    assert!((point.cpu_usage - 45.5).abs() < f32::EPSILON);
    assert!((point.memory_usage_percent - 50.0).abs() < f32::EPSILON);
    assert_eq!(point.memory_used_mb, 8192.0);
    assert_eq!(point.memory_total_mb, 16384.0);
    assert!(!point.timestamp.is_empty());

    // 클램핑 검증 (비정상 값 150% -> 100% 제한)
    let overflow_point = SystemMetricPoint::new(1, 150.0, 20000.0, 10000.0);
    assert_eq!(overflow_point.cpu_usage, 100.0);
    assert_eq!(overflow_point.memory_usage_percent, 100.0);
}

#[test]
fn test_metric_history_ring_buffer() {
    let mut history = MetricHistory::new(5);
    assert_eq!(history.points.len(), 0);

    for i in 0..10 {
        let p = SystemMetricPoint::new(i, i as f32 * 10.0, 1000.0, 4000.0);
        history.push(p);
    }

    // 최대 5개 유지 확인
    assert_eq!(history.points.len(), 5);
    // 가장 최근 인덱스는 9번이어야 함
    assert_eq!(history.latest().unwrap().index, 9);
    // 첫 번째 인덱스는 5번이어야 함 (0~4 밀려남)
    assert_eq!(history.points[0].index, 5);

    // 초기화 테스트
    history.clear();
    assert_eq!(history.points.len(), 0);
    assert_eq!(history.next_index, 0);
}

#[test]
fn test_metric_statistics() {
    let mut history = MetricHistory::new(10);
    history.push(SystemMetricPoint::new(0, 10.0, 2000.0, 10000.0)); // mem: 20%
    history.push(SystemMetricPoint::new(1, 30.0, 4000.0, 10000.0)); // mem: 40%
    history.push(SystemMetricPoint::new(2, 50.0, 6000.0, 10000.0)); // mem: 60%

    // CPU 통계
    assert!((history.average_cpu() - 30.0).abs() < f32::EPSILON);
    assert_eq!(history.max_cpu(), 50.0);

    // RAM 통계
    assert!((history.average_memory() - 40.0).abs() < f32::EPSILON);
    assert_eq!(history.max_memory(), 60.0);
}

#[test]
fn test_csv_serialization_and_deserialization() {
    let mut history = MetricHistory::new(10);
    history.push(SystemMetricPoint::new(0, 12.5, 4096.0, 16384.0));
    history.push(SystemMetricPoint::new(1, 88.0, 8192.0, 16384.0));

    // CSV 문자열로 변환
    let csv_str = history.to_csv_string().expect("CSV 직렬화 실패");
    assert!(csv_str.contains("cpu_usage"));
    assert!(csv_str.contains("12.5"));
    assert!(csv_str.contains("88"));

    // 다시 역직렬화하여 복원
    let restored = MetricHistory::from_csv_str(&csv_str).expect("CSV 파싱 실패");
    assert_eq!(restored.len(), 2);
    assert_eq!(restored[0].index, 0);
    assert!((restored[0].cpu_usage - 12.5).abs() < f32::EPSILON);
    assert_eq!(restored[1].index, 1);
    assert!((restored[1].cpu_usage - 88.0).abs() < f32::EPSILON);
}

#[test]
fn test_worker_channel_messages() {
    let (tx, rx) = std::sync::mpsc::channel::<WorkerCommand>();
    tx.send(WorkerCommand::StartMonitoring).unwrap();
    tx.send(WorkerCommand::SetIntervalMs(250)).unwrap();
    tx.send(WorkerCommand::StartHeavyTask { target_iterations: 50_000 }).unwrap();

    match rx.recv().unwrap() {
        WorkerCommand::StartMonitoring => {}
        _ => panic!("예상치 못한 명령"),
    }

    match rx.recv().unwrap() {
        WorkerCommand::SetIntervalMs(ms) => assert_eq!(ms, 250),
        _ => panic!("예상치 못한 명령"),
    }

    match rx.recv().unwrap() {
        WorkerCommand::StartHeavyTask { target_iterations } => assert_eq!(target_iterations, 50_000),
        _ => panic!("예상치 못한 명령"),
    }
}

#[test]
fn test_dashboard_tabs_metadata() {
    let tabs = DashboardTab::all();
    assert_eq!(tabs.len(), 3);
    assert_eq!(tabs[0], DashboardTab::LiveChart);
    assert_eq!(tabs[1], DashboardTab::HeavyWorker);
    assert_eq!(tabs[2], DashboardTab::DataManager);

    for tab in tabs {
        assert!(!tab.title().is_empty());
    }
}

#[test]
fn test_font_setup_and_korean_rendering() {
    let ctx = eframe::egui::Context::default();
    setup_korean_and_emoji_fonts(&ctx);

    let sample_text = "Rust 실시간 모니터링 대시보드 📊 ⚡ 🚀 12345";
    let _ = ctx.run(eframe::egui::RawInput::default(), |ctx| {
        ctx.fonts(|fonts| {
            let galley = fonts.layout_no_wrap(
                sample_text.to_string(),
                eframe::egui::FontId::proportional(14.0),
                eframe::egui::Color32::WHITE,
            );
            assert!(galley.size().x > 0.0);
        });
    });
}
