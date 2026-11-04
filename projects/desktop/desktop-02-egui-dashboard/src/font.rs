use eframe::egui;

/// 시스템에 설치된 한글 폰트 및 유니코드 이모지 폰트를 자동으로 탐색하여 egui에 등록합니다.
pub fn setup_korean_and_emoji_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 1. 한국어 폰트 후보군 (OS별 우선순위)
    let mut font_candidates = Vec::new();
    if let Ok(windir) = std::env::var("WINDIR") {
        font_candidates.push(format!("{windir}\\Fonts\\malgun.ttf"));
        font_candidates.push(format!("{windir}\\Fonts\\gulim.ttc"));
    }
    font_candidates.push("C:\\Windows\\Fonts\\malgun.ttf".to_string());
    font_candidates.push("C:\\Windows\\Fonts\\gulim.ttc".to_string());
    font_candidates.push("/System/Library/Fonts/AppleSDGothicNeo.ttc".to_string());
    font_candidates.push("/Library/Fonts/Arial Unicode.ttf".to_string());
    font_candidates.push("/usr/share/fonts/truetype/nanum/NanumGothic.ttf".to_string());
    font_candidates.push("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc".to_string());
    font_candidates.push("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc".to_string());

    let mut loaded_korean = false;
    for path in font_candidates {
        if let Ok(font_bytes) = std::fs::read(&path) {
            fonts.font_data.insert(
                "korean_font".to_owned(),
                egui::FontData::from_owned(font_bytes).into(),
            );
            // 비례 폰트 우선순위에 추가
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "korean_font".to_owned());
            // 고정폭 폰트 fallback에도 추가
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("korean_font".to_owned());
            loaded_korean = true;
            break;
        }
    }

    // 2. Windows Segoe UI Emoji 폰트 탐색
    let mut emoji_candidates = Vec::new();
    if let Ok(windir) = std::env::var("WINDIR") {
        emoji_candidates.push(format!("{windir}\\Fonts\\seguiemj.ttf"));
    }
    emoji_candidates.push("C:\\Windows\\Fonts\\seguiemj.ttf".to_string());

    for path in emoji_candidates {
        if let Ok(emoji_bytes) = std::fs::read(&path) {
            fonts.font_data.insert(
                "emoji_font".to_owned(),
                egui::FontData::from_owned(emoji_bytes).into(),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .push("emoji_font".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("emoji_font".to_owned());
            break;
        }
    }

    if loaded_korean {
        ctx.set_fonts(fonts);
    }
}

