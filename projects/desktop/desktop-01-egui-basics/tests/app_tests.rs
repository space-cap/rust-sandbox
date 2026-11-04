use desktop_01_egui_basics::models::{
    calculate_experience_level, format_greeting, validate_username, AppTheme, UserProfile,
    UserRole,
};
use desktop_01_egui_basics::DesktopBasicsApp;

#[test]
fn test_default_profile() {
    let profile = UserProfile::default();
    assert_eq!(profile.name, "페리스 (Ferris)");
    assert_eq!(profile.role, UserRole::Developer);
    assert_eq!(profile.experience_years, 3);
    assert!(profile.is_subscribed);
    assert_eq!(profile.volume, 75.0);
}

#[test]
fn test_format_greeting() {
    let greeting = format_greeting("홍길동", &UserRole::Developer);
    assert!(greeting.contains("홍길동"));
    assert!(greeting.contains("개발자"));

    let empty_greeting = format_greeting("   ", &UserRole::Designer);
    assert!(empty_greeting.contains("익명의 러스트 탐험가"));
}

#[test]
fn test_experience_level() {
    assert!(calculate_experience_level(0).contains("새싹"));
    assert!(calculate_experience_level(2).contains("주니어"));
    assert!(calculate_experience_level(5).contains("미드레벨"));
    assert!(calculate_experience_level(10).contains("시니어"));
    assert!(calculate_experience_level(25).contains("마스터"));
}

#[test]
fn test_username_validation() {
    assert!(validate_username("Ferris").is_ok());
    assert!(validate_username("").is_err());
    assert!(validate_username("    ").is_err());
    assert!(validate_username(&"A".repeat(50)).is_err());
}

#[test]
fn test_counter_and_history() {
    let mut app = DesktopBasicsApp::default();
    assert_eq!(app.counter, 0);
    assert_eq!(app.current_theme, AppTheme::Dark);

    app.counter += 5;
    app.record_click("테스트 5 증가");
    assert_eq!(app.counter, 5);
    assert_eq!(app.click_history.len(), 2);
    assert!(app.click_history[1].contains("테스트 5 증가"));
    assert!(app.click_history[1].contains("5"));

    // 50개 제한 테스트
    for i in 0..60 {
        app.record_click(&format!("액션 {i}"));
    }
    assert!(app.click_history.len() <= 50);
}
