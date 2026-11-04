use desktop_02_egui_dashboard::DashboardApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1020.0, 720.0])
            .with_min_inner_size([720.0, 500.0])
            .with_title("Rust Desktop 02: 실시간 시스템 모니터링 & 비동기 워커 대시보드 📊"),
        ..Default::default()
    };

    println!("🚀 [desktop-02-egui-dashboard] 실시간 모니터링 대시보드 윈도우 창을 실행합니다...");

    eframe::run_native(
        "desktop-02-egui-dashboard",
        options,
        Box::new(|cc| Ok(Box::new(DashboardApp::new(cc)))),
    )
}

