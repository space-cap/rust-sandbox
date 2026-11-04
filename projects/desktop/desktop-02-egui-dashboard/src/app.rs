use eframe::egui;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};
use std::time::Duration;

use crate::font::setup_korean_and_emoji_fonts;
use crate::models::{DashboardTab, MetricHistory, WorkerCommand, WorkerEvent};
use crate::worker::WorkerHandle;

pub struct DashboardApp {
    pub worker: WorkerHandle,
    pub history: MetricHistory,
    pub current_tab: DashboardTab,
    pub is_monitoring_active: bool,
    pub sample_interval_ms: u64,
    pub auto_fit_plot: bool,

    // 비동기 무거운 연산 상태
    pub heavy_task_progress: f32,
    pub heavy_task_status: String,
    pub heavy_task_is_running: bool,
    pub heavy_target_iterations: u64,
    pub responsiveness_counter: i64,

    // CSV 관리 상태
    pub csv_status_message: Option<(String, bool)>,

    // 시스템 및 테마
    pub ui_frame_count: u64,
    pub is_dark_theme: bool,
}

impl DashboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 시스템 한글/이모지 폰트 로드
        setup_korean_and_emoji_fonts(&cc.egui_ctx);

        let worker = WorkerHandle::spawn();

        Self {
            worker,
            history: MetricHistory::default(),
            current_tab: DashboardTab::LiveChart,
            is_monitoring_active: true,
            sample_interval_ms: 500,
            auto_fit_plot: true,

            heavy_task_progress: 0.0,
            heavy_task_status: "대기 중: 연산 시작 버튼을 눌러보세요.".to_string(),
            heavy_task_is_running: false,
            heavy_target_iterations: 100_000,
            responsiveness_counter: 0,

            csv_status_message: None,

            ui_frame_count: 0,
            is_dark_theme: true,
        }
    }

    /// 백그라운드 워커 이벤트 수신 및 상태 갱신
    fn process_worker_events(&mut self) {
        while let Ok(event) = self.worker.event_rx.try_recv() {
            match event {
                WorkerEvent::MetricUpdate(point) => {
                    self.history.push(point);
                }
                WorkerEvent::HeavyTaskProgress {
                    progress,
                    current_step,
                } => {
                    self.heavy_task_progress = progress;
                    self.heavy_task_status = current_step;
                }
                WorkerEvent::HeavyTaskDone { summary, .. } => {
                    self.heavy_task_progress = 1.0;
                    self.heavy_task_status = summary;
                    self.heavy_task_is_running = false;
                }
                WorkerEvent::HeavyTaskCancelled => {
                    self.heavy_task_status = "연산이 사용자에 의해 즉시 취소되었습니다.".to_string();
                    self.heavy_task_is_running = false;
                }
            }
        }
    }

    /// CSV 파일로 현재 메트릭 데이터 내보내기 (rfd 네이티브 다이얼로그)
    pub fn export_csv(&mut self) {
        let file_path = rfd::FileDialog::new()
            .add_filter("CSV 스프레드시트", &["csv"])
            .set_file_name("system_metrics.csv")
            .save_file();

        if let Some(path) = file_path {
            match self.history.to_csv_string() {
                Ok(csv_data) => match std::fs::write(&path, csv_data) {
                    Ok(_) => {
                        self.csv_status_message = Some((
                            format!("성공: '{}' 경로에 CSV 파일이 저장되었습니다.", path.display()),
                            true,
                        ));
                    }
                    Err(e) => {
                        self.csv_status_message = Some((
                            format!("파일 저장 실패: {e}"),
                            false,
                        ));
                    }
                },
                Err(e) => {
                    self.csv_status_message = Some((
                        format!("CSV 변환 실패: {e}"),
                        false,
                    ));
                }
            }
        }
    }

    /// CSV 파일로부터 메트릭 데이터 불러오기 (rfd 네이티브 다이얼로그)
    pub fn import_csv(&mut self) {
        let file_path = rfd::FileDialog::new()
            .add_filter("CSV 스프레드시트", &["csv"])
            .pick_file();

        if let Some(path) = file_path {
            match std::fs::read_to_string(&path) {
                Ok(csv_data) => match MetricHistory::from_csv_str(&csv_data) {
                    Ok(records) => {
                        let count = records.len();
                        self.history.clear();
                        for r in records {
                            self.history.push(r);
                        }
                        self.csv_status_message = Some((
                            format!("성공: '{}'에서 총 {count}개의 메트릭 기록을 불러왔습니다.", path.display()),
                            true,
                        ));
                    }
                    Err(e) => {
                        self.csv_status_message = Some((
                            format!("CSV 파싱 실패: {e}"),
                            false,
                        ));
                    }
                },
                Err(e) => {
                    self.csv_status_message = Some((
                        format!("파일 읽기 실패: {e}"),
                        false,
                    ));
                }
            }
        }
    }
}

impl eframe::App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui_frame_count += 1;

        // 1. 백그라운드 워커 이벤트 처리 (논블로킹)
        self.process_worker_events();

        // 2. 모니터링 활성 시 주기적 자동 렌더링 요청 (UI 프리징 차단 + 부드러운 차트 갱신)
        if self.is_monitoring_active || self.heavy_task_is_running {
            ctx.request_repaint_after(Duration::from_millis(self.sample_interval_ms.min(200)));
        }

        // 테마 설정
        if self.is_dark_theme {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // 상단 메뉴 & 탭 네비게이션 바
        egui::TopBottomPanel::top("top_dashboard_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("🖥️ ");
                ui.strong("Rust Desktop 02: 실시간 시스템 모니터링 대시보드");
                ui.separator();

                // 탭 선택 버튼들
                for tab in DashboardTab::all() {
                    let is_selected = self.current_tab == tab;
                    if ui.selectable_label(is_selected, tab.title()).clicked() {
                        self.current_tab = tab;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.is_dark_theme { "☀️ 라이트" } else { "🌙 다크" }).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }
                });
            });
        });

        // 하단 상태 표시줄
        egui::TopBottomPanel::bottom("bottom_dashboard_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🚀 스택: eframe 0.31 + egui_plot + Tokio Worker");
                ui.separator();

                if let Some(latest) = self.history.latest() {
                    ui.label(format!("📊 현재 CPU: {:.1}%", latest.cpu_usage));
                    ui.separator();
                    ui.label(format!(
                        "🧠 RAM: {:.0} MB / {:.0} MB ({:.1}%)",
                        latest.memory_used_mb, latest.memory_total_mb, latest.memory_usage_percent
                    ));
                    ui.separator();
                }

                ui.label(format!("⏱️ UI 렌더 프레임: {} 회", self.ui_frame_count));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if self.is_monitoring_active {
                        ui.label("🟢 모니터링 실행 중");
                    } else {
                        ui.label("⏸️ 모니터링 일시 정지");
                    }
                });
            });
        });

        // 중앙 메인 탭 렌더링
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                DashboardTab::LiveChart => self.render_live_chart_tab(ui),
                DashboardTab::HeavyWorker => self.render_heavy_worker_tab(ui),
                DashboardTab::DataManager => self.render_data_manager_tab(ui),
            }
        });
    }
}

impl DashboardApp {
    /// 탭 1: 실시간 메트릭 차트 렌더링
    fn render_live_chart_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("📈 실시간 CPU & RAM 성능 모니터링");
        ui.label("백그라운드 워커가 sysinfo로 측정한 시스템 지표를 논블로킹 채널로 받아 렌더링합니다.");
        ui.separator();

        // 1. 상단 통계 카드 그리드
        ui.horizontal(|ui| {
            // CPU 요약 카드
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(260.0);
                ui.heading("⚡ CPU 사용률");
                let (curr_cpu, avg_cpu, max_cpu) = if let Some(latest) = self.history.latest() {
                    (latest.cpu_usage, self.history.average_cpu(), self.history.max_cpu())
                } else {
                    (0.0, 0.0, 0.0)
                };
                ui.label(
                    egui::RichText::new(format!("{curr_cpu:.1}%"))
                        .size(28.0)
                        .strong()
                        .color(egui::Color32::from_rgb(251, 146, 60)),
                );
                ui.label(format!("평균: {avg_cpu:.1}%  |  최고: {max_cpu:.1}%"));
            });

            // RAM 요약 카드
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(260.0);
                ui.heading("🧠 RAM 사용률");
                let (curr_ram, used_mb, total_mb) = if let Some(latest) = self.history.latest() {
                    (latest.memory_usage_percent, latest.memory_used_mb, latest.memory_total_mb)
                } else {
                    (0.0, 0.0, 0.0)
                };
                ui.label(
                    egui::RichText::new(format!("{curr_ram:.1}%"))
                        .size(28.0)
                        .strong()
                        .color(egui::Color32::from_rgb(52, 211, 153)),
                );
                ui.label(format!("{used_mb:.0} MB / {total_mb:.0} MB"));
            });

            // 데이터 수집 상태 카드
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_width(260.0);
                ui.heading("📊 수집 데이터 상태");
                ui.label(
                    egui::RichText::new(format!("{}/{} 개", self.history.points.len(), self.history.max_capacity))
                        .size(28.0)
                        .strong()
                        .color(egui::Color32::from_rgb(96, 165, 250)),
                );
                ui.label(format!("수집 주기: {} ms (초당 {:.1}회)", self.sample_interval_ms, 1000.0 / self.sample_interval_ms as f64));
            });
        });

        ui.add_space(8.0);

        // 2. 조작 제어부 (수집 토글, 주기 조절, 리셋)
        ui.horizontal(|ui| {
            if self.is_monitoring_active {
                if ui.button("⏸️ 수집 일시 정지").clicked() {
                    self.is_monitoring_active = false;
                    let _ = self.worker.cmd_tx.send(WorkerCommand::PauseMonitoring);
                }
            } else {
                if ui.button("▶️ 수집 재개").clicked() {
                    self.is_monitoring_active = true;
                    let _ = self.worker.cmd_tx.send(WorkerCommand::StartMonitoring);
                }
            }

            if ui.button("🗑️ 차트 기록 초기화").clicked() {
                self.history.clear();
            }

            ui.separator();
            let prev_interval = self.sample_interval_ms;
            ui.add(egui::Slider::new(&mut self.sample_interval_ms, 100..=2000).text("수집 주기 (ms)"));
            if prev_interval != self.sample_interval_ms {
                let _ = self.worker.cmd_tx.send(WorkerCommand::SetIntervalMs(self.sample_interval_ms));
            }
        });

        ui.add_space(8.0);

        // 3. egui_plot 동적 차트 렌더링
        let cpu_points: PlotPoints = self
            .history
            .points
            .iter()
            .map(|p| [p.index as f64, p.cpu_usage as f64])
            .collect();

        let memory_points: PlotPoints = self
            .history
            .points
            .iter()
            .map(|p| [p.index as f64, p.memory_usage_percent as f64])
            .collect();

        let cpu_line = Line::new(cpu_points)
            .name("CPU 점유율 (%)")
            .color(egui::Color32::from_rgb(251, 146, 60))
            .width(2.0_f32);

        let memory_line = Line::new(memory_points)
            .name("RAM 점유율 (%)")
            .color(egui::Color32::from_rgb(52, 211, 153))
            .width(2.0_f32);

        Plot::new("system_metrics_plot")
            .legend(Legend::default().position(Corner::RightBottom))
            .include_y(0.0)
            .include_y(100.0)
            .height(ui.available_height() - 15.0)
            .show(ui, |plot_ui| {
                plot_ui.line(cpu_line);
                plot_ui.line(memory_line);
            });
    }

    /// 탭 2: 비동기 워커 무거운 연산 & UI 프리징 차단 체험
    fn render_heavy_worker_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚡ 비동기 백그라운드 워커 & UI 무중단 체험");
        ui.label("메인 UI 스레드가 블로킹되지 않도록 Tokio/OS 백그라운드 스레드로 무거운 연산을 위임합니다.");
        ui.separator();

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("💡 왜 메인 스레드 분리가 필수인가요?");
            ui.label("• 버튼 클릭 안에서 5초 걸리는 작업을 직접 돌리면, 윈도우 OS가 '응답 없음(Not Responding)'을 띄웁니다.");
            ui.label("• 아래에서 대용량 소수 판별 연산을 시작한 후, 우측의 즉각 반응 카운터를 자유롭게 클릭해보세요!");
            ui.label("• 100% CPU 부하 상태에서도 UI 버튼과 화면 조작이 단 1밀리초도 끊기지 않습니다.");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("연산 대상 숫자 범위:");
            ui.add(egui::Slider::new(&mut self.heavy_target_iterations, 10_000..=500_000).text("개 탐색"));
        });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if self.heavy_task_is_running {
                if ui.button("🛑 연산 중지 (Cancel)").clicked() {
                    let _ = self.worker.cmd_tx.send(WorkerCommand::CancelHeavyTask);
                }
            } else {
                if ui.button("🚀 백그라운드 소수 연산 시작").clicked() {
                    self.heavy_task_is_running = true;
                    self.heavy_task_progress = 0.0;
                    self.heavy_task_status = "연산 시작 중...".to_string();
                    let _ = self.worker.cmd_tx.send(WorkerCommand::StartHeavyTask {
                        target_iterations: self.heavy_target_iterations,
                    });
                }
            }
        });

        ui.add_space(10.0);
        ui.label("진행률:");
        ui.add(egui::ProgressBar::new(self.heavy_task_progress).show_percentage());
        ui.label(format!("상태: {}", self.heavy_task_status));

        ui.separator();
        ui.heading("🎮 UI 반응성 실시간 검증 위젯");
        ui.label("무거운 연산이 백그라운드에서 돌아가는 동안에도 즉시 반응합니다:");

        ui.horizontal(|ui| {
            if ui.button("− 1").clicked() {
                self.responsiveness_counter -= 1;
            }
            if ui.button("+ 1").clicked() {
                self.responsiveness_counter += 1;
            }
            if ui.button("🔄 리셋").clicked() {
                self.responsiveness_counter = 0;
            }
            ui.label(
                egui::RichText::new(format!("반응성 카운터: {}", self.responsiveness_counter))
                    .size(18.0)
                    .strong()
                    .color(egui::Color32::from_rgb(56, 189, 248)),
            );
        });
    }

    /// 탭 3: 데이터 관리 (CSV 내보내기/불러오기 및 테이블)
    fn render_data_manager_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("💾 데이터 관리 및 CSV 입출력 (rfd 다이얼로그)");
        ui.label("OS 네이티브 파일 탐색기 창을 호출하여 수집된 시계열 메트릭을 CSV로 저장하거나 불러옵니다.");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("📁 CSV 파일로 내보내기 (Export)").clicked() {
                self.export_csv();
            }

            if ui.button("📂 CSV 파일 불러오기 (Import)").clicked() {
                self.import_csv();
            }

            if ui.button("🗑️ 수집 데이터 비우기").clicked() {
                self.history.clear();
                self.csv_status_message = Some(("데이터가 초기화되었습니다.".to_string(), true));
            }
        });

        if let Some((msg, is_success)) = &self.csv_status_message {
            ui.add_space(5.0);
            let color = if *is_success {
                egui::Color32::from_rgb(74, 222, 128)
            } else {
                egui::Color32::from_rgb(248, 113, 113)
            };
            ui.label(egui::RichText::new(msg).color(color).strong());
        }

        ui.separator();
        ui.heading(format!("📋 수집된 메트릭 목록 (총 {}건)", self.history.points.len()));

        egui::ScrollArea::both()
            .max_height(ui.available_height() - 20.0)
            .show(ui, |ui| {
                egui::Grid::new("metrics_table")
                    .striped(true)
                    .spacing([20.0, 6.0])
                    .show(ui, |ui| {
                        ui.strong("인덱스");
                        ui.strong("시각");
                        ui.strong("CPU 점유율");
                        ui.strong("메모리 사용량");
                        ui.strong("전체 메모리");
                        ui.strong("메모리 점유율");
                        ui.end_row();

                        for point in self.history.points.iter().rev() {
                            ui.label(format!("#{}", point.index));
                            ui.label(&point.timestamp);
                            ui.label(format!("{:.1}%", point.cpu_usage));
                            ui.label(format!("{:.1} MB", point.memory_used_mb));
                            ui.label(format!("{:.1} MB", point.memory_total_mb));
                            ui.label(format!("{:.1}%", point.memory_usage_percent));
                            ui.end_row();
                        }
                    });
            });
    }
}
