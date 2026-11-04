use eframe::egui;
use crate::models::{
    calculate_experience_level, format_greeting, AppTheme, UserProfile, UserRole,
};

pub struct DesktopBasicsApp {
    pub user: UserProfile,
    pub counter: i32,
    pub counter_step: i32,
    pub click_history: Vec<String>,
    pub current_theme: AppTheme,
    pub show_about_dialog: bool,
    pub frame_count: u64,
}

impl Default for DesktopBasicsApp {
    fn default() -> Self {
        Self {
            user: UserProfile::default(),
            counter: 0,
            counter_step: 1,
            click_history: vec!["앱 초기화 완료 (기본 상태)".to_string()],
            current_theme: AppTheme::Dark,
            show_about_dialog: false,
            frame_count: 0,
        }
    }
}

impl DesktopBasicsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // 커스텀 폰트 또는 초기 스타일 설정 가능
        Self::default()
    }

    pub fn record_click(&mut self, action: &str) {
        let msg = format!("[#{}] {}: 현재 값 = {}", self.click_history.len(), action, self.counter);
        self.click_history.push(msg);
        if self.click_history.len() > 50 {
            self.click_history.remove(0);
        }
    }
}

impl eframe::App for DesktopBasicsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;

        // 테마 적용
        match self.current_theme {
            AppTheme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            AppTheme::Light => ctx.set_visuals(egui::Visuals::light()),
        }

        // 1. 상단 메뉴 패널 (TopBottomPanel::top)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("🦀 ");
                ui.strong("Rust Desktop 01: egui 초경량 GUI 기초");
                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("ℹ️ 정보").clicked() {
                        self.show_about_dialog = !self.show_about_dialog;
                    }

                    match self.current_theme {
                        AppTheme::Dark => {
                            if ui.button("☀️ 라이트 모드").clicked() {
                                self.current_theme = AppTheme::Light;
                            }
                        }
                        AppTheme::Light => {
                            if ui.button("🌙 다크 모드").clicked() {
                                self.current_theme = AppTheme::Dark;
                            }
                        }
                    }
                });
            });
        });

        // 2. 하단 상태 표시줄 (TopBottomPanel::bottom)
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🚀 프레임워크: egui 0.31 (eframe)");
                ui.separator();
                ui.label("🎨 렌더러: GPU 가속 (OpenGL/glow)");
                ui.separator();
                ui.label(format!("⏱️ 누적 렌더 프레임: {} 회", self.frame_count));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("🟢 상태: 정상 실행 중");
                });
            });
        });

        // 3. 좌측 사이드 패널 (SidePanel::left): 실시간 카운터 및 조작부
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("⚡ 실시간 카운터");
                ui.label("즉시 모드(Immediate Mode) 원리 체험");
                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("CURRENT COUNTER");
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{}", self.counter))
                            .size(36.0)
                            .strong()
                            .color(egui::Color32::from_rgb(56, 189, 248)),
                    ));
                    ui.add_space(10.0);
                });

                // 카운터 조작 버튼들
                ui.horizontal(|ui| {
                    if ui.button(format!("− {}", self.counter_step)).clicked() {
                        self.counter -= self.counter_step;
                        self.record_click("감소 클릭");
                    }
                    if ui.button(format!("+ {}", self.counter_step)).clicked() {
                        self.counter += self.counter_step;
                        self.record_click("증가 클릭");
                    }
                    if ui.button("🔄 리셋").clicked() {
                        self.counter = 0;
                        self.record_click("카운터 초기화");
                    }
                });

                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.counter_step, 1..=20).text("증감 단위"));

                ui.separator();
                ui.heading("📜 조작 히스토리");
                ui.label("최근 50개 이벤트 로깅:");

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for log in self.click_history.iter().rev() {
                            ui.label(egui::RichText::new(log).size(11.0).monospace());
                        }
                    });
            });

        // 4. 중앙 메인 패널 (CentralPanel): 사용자 프로필 폼 & 위젯 쇼케이스
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 인터랙티브 위젯 쇼케이스");
            ui.label("변수의 포인터를 넘기면 UI가 매 프레임 변수와 동기화됩니다.");
            ui.separator();

            egui::Grid::new("profile_grid")
                .num_columns(2)
                .spacing([15.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("👤 이름:");
                    ui.text_edit_singleline(&mut self.user.name);
                    ui.end_row();

                    ui.label("💼 직무:");
                    egui::ComboBox::from_id_salt("role_combo")
                        .selected_text(self.user.role.as_str())
                        .show_ui(ui, |ui| {
                            for role in UserRole::all() {
                                ui.selectable_value(&mut self.user.role, role, role.as_str());
                            }
                        });
                    ui.end_row();

                    ui.label("📈 경력 (연차):");
                    ui.add(egui::Slider::new(&mut self.user.experience_years, 0..=30).text("년차"));
                    ui.end_row();

                    ui.label("🔔 구독 여부:");
                    ui.checkbox(&mut self.user.is_subscribed, "주간 Rust 뉴스레터 구독");
                    ui.end_row();

                    ui.label("🔊 시스템 볼륨:");
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut self.user.volume, 0.0..=100.0).text("%"));
                        let frac = self.user.volume / 100.0;
                        ui.add(egui::ProgressBar::new(frac).show_percentage());
                    });
                    ui.end_row();
                });

            ui.add_space(15.0);

            // 동적 인사말 카드 (Frame)
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.heading("✨ 실시간 파생 상태 미리보기");
                let greeting = format_greeting(&self.user.name, &self.user.role);
                ui.label(
                    egui::RichText::new(greeting)
                        .size(16.0)
                        .strong()
                        .color(egui::Color32::from_rgb(74, 222, 128)),
                );

                let level = calculate_experience_level(self.user.experience_years);
                ui.label(format!("🏅 숙련도 등급: {level}"));

                if self.user.is_subscribed {
                    ui.label("📬 뉴스레터 발송 대상입니다.");
                } else {
                    ui.label("🔕 알림을 받지 않습니다.");
                }
            });

            ui.add_space(15.0);

            // 즉시 모드(Immediate Mode) 원리 설명 상자
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label(
                    egui::RichText::new("💡 즉시 모드(Immediate Mode)의 작동 원리")
                        .strong()
                        .color(egui::Color32::from_rgb(251, 146, 60)),
                );
                ui.label("1. 버튼, 슬라이더 객체를 힙 메모리에 영구 저장하지 않습니다.");
                ui.label("2. 매 프레임 update()가 호출될 때 Rust 변수 값을 읽어 즉시 GPU에 그립니다.");
                ui.label("3. 따라서 '화면의 값과 백엔드 변수 값이 따로 노는 버그'가 원천적으로 불가능합니다!");
            });
        });

        // 5. 모달 팝업 창 (egui::Window)
        if self.show_about_dialog {
            let mut is_open = self.show_about_dialog;
            let mut close_clicked = false;
            egui::Window::new("ℹ️ egui 프레임워크 정보")
                .collapsible(false)
                .resizable(false)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    ui.heading("desktop-01-egui-basics");
                    ui.label("Rust 순수 즉시 모드 GUI 실습 예제");
                    ui.separator();
                    ui.label("• 버전: eframe 0.31");
                    ui.label("• 플랫폼: Windows x86_64 네이티브");
                    ui.label("• Electron 대비 메모리 사용량: 약 1/10 (30MB 내외)");
                    ui.separator();
                    if ui.button("닫기").clicked() {
                        close_clicked = true;
                    }
                });
            self.show_about_dialog = is_open && !close_clicked;
        }
    }
}
