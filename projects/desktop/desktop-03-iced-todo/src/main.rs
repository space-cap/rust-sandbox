use desktop_03_iced_todo::TodoApp;

fn main() -> iced::Result {
    println!("🚀 [desktop-03-iced-todo] Iced Elm 아키텍처 Todo 앱을 실행합니다...");

    iced::application(TodoApp::title, TodoApp::update, TodoApp::view)
        .theme(TodoApp::theme)
        .default_font(iced::Font::with_name("Malgun Gothic"))
        .window(iced::window::Settings {
            size: iced::Size::new(760.0, 700.0),
            min_size: Some(iced::Size::new(500.0, 450.0)),
            ..Default::default()
        })
        .run_with(TodoApp::new)
}
