use wasm_06_leptos_showcase::models::{
    calculate_progress, compute_doubled, compute_squared, filter_tech_items, is_even, TechCategory,
    TechItem,
};

#[test]
fn test_derived_calculations() {
    assert_eq!(compute_doubled(0), 0);
    assert_eq!(compute_doubled(15), 30);
    assert_eq!(compute_doubled(-8), -16);

    assert_eq!(compute_squared(0), 0);
    assert_eq!(compute_squared(10), 100);
    assert_eq!(compute_squared(-5), 25);

    assert!(is_even(0));
    assert!(is_even(42));
    assert!(!is_even(17));
    assert!(is_even(-4));
}

#[test]
fn test_filter_tech_items_query() {
    let items = TechItem::sample_data();

    // 1. 이름 및 설명 검색 (대소문자 무시: Leptos 본체 + 설명에 Leptos가 언급된 SolidJS)
    let leptos_matches = filter_tech_items(&items, "leptos", None);
    assert_eq!(leptos_matches.len(), 2);
    assert!(leptos_matches.iter().any(|it| it.name == "Leptos"));

    // 2. 태그 검색
    let vdom_matches = filter_tech_items(&items, "vdom", None);
    assert!(vdom_matches.iter().any(|it| it.name == "Yew"));

    // 3. 빈 검색어 -> 전체 반환
    let all_matches = filter_tech_items(&items, "", None);
    assert_eq!(all_matches.len(), items.len());

    // 4. 없는 키워드 -> 0건
    let none_matches = filter_tech_items(&items, "NonExistentKeywordXYZ", None);
    assert_eq!(none_matches.len(), 0);
}

#[test]
fn test_filter_tech_items_category() {
    let items = TechItem::sample_data();

    let rust_wasm = filter_tech_items(&items, "", Some(TechCategory::RustWasm));
    assert!(rust_wasm.iter().all(|it| it.category == TechCategory::RustWasm));
    assert!(rust_wasm.len() >= 2);

    let js = filter_tech_items(&items, "", Some(TechCategory::JavaScript));
    assert_eq!(js.len(), 1);
    assert_eq!(js[0].name, "SolidJS");
}

#[test]
fn test_calculate_progress() {
    assert_eq!(calculate_progress(0, 5), (0, 5, 0));
    assert_eq!(calculate_progress(3, 5), (3, 5, 60));
    assert_eq!(calculate_progress(5, 5), (5, 5, 100));
    assert_eq!(calculate_progress(0, 0), (0, 0, 0));
}

#[test]
fn test_tech_item_creation_and_sample_data() {
    let item = TechItem::new(
        99,
        "TestFramework",
        TechCategory::Tooling,
        "테스트 설명",
        500,
        vec!["Test", "Wasm"],
    );
    assert_eq!(item.id, 99);
    assert_eq!(item.name, "TestFramework");
    assert_eq!(item.stars, 500);
    assert_eq!(item.tags.len(), 2);

    let samples = TechItem::sample_data();
    assert!(!samples.is_empty());
    assert!(samples.iter().any(|s| s.name == "Leptos"));
}

