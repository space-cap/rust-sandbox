use desktop_01_egui_basics::DesktopBasicsApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // 윈도우 창 설정 옵션
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([880.0, 620.0])
            .with_min_inner_size([600.0, 450.0])
            .with_title("Rust Desktop 01: egui 초경량 GUI 기초 🦀"),
        ..Default::default()
    };

    println!("🚀 [desktop-01-egui-basics] egui 윈도우 데스크톱 창을 실행합니다...");

    eframe::run_native(
        "desktop-01-egui-basics",
        options,
        Box::new(|cc| Ok(Box::new(DesktopBasicsApp::new(cc)))),
    )
}
