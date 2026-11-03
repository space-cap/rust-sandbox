use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlButtonElement, HtmlInputElement};

// ==============================================================================
// 1. 순수 Rust 비즈니스 로직 (단위 테스트 가능)
// ==============================================================================

/// 텍스트 분석 결과 DTO
#[derive(Debug, PartialEq, Eq)]
pub struct TextAnalysis {
    pub char_count: usize,
    pub word_count: usize,
    pub reversed: String,
    pub uppercase: String,
}

/// 문자열을 분석하여 통계 및 변환 데이터를 반환합니다.
pub fn analyze_text(input: &str) -> TextAnalysis {
    let char_count = input.chars().count();
    let word_count = input.split_whitespace().count();
    let reversed = input.chars().rev().collect();
    let uppercase = input.to_uppercase();

    TextAnalysis {
        char_count,
        word_count,
        reversed,
        uppercase,
    }
}

/// 피보나치 수열을 고속으로 계산합니다.
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a.saturating_add(b);
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// 환영 메시지 생성 함수
pub fn format_greeting(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        "게스트님, Rust WebAssembly 세계에 오신 것을 환영합니다! 🦀".to_string()
    } else {
        format!("{trimmed}님, Rust WebAssembly 세계에 오신 것을 환영합니다! 🦀")
    }
}

// ==============================================================================
// 2. JavaScript로 공개(Export)할 Wasm 함수들 (FFI)
// ==============================================================================

/// JavaScript 환경에서 직접 호출할 수 있는 exported Rust 함수
#[wasm_bindgen]
pub fn rust_greet(name: &str) -> String {
    format_greeting(name)
}

/// JavaScript 환경에서 호출 가능한 Rust 고속 피보나치 연산 함수
#[wasm_bindgen]
pub fn rust_fibonacci(n: u32) -> u64 {
    fibonacci(n)
}

// ==============================================================================
// 3. WebAssembly 애플리케이션 진입점 및 DOM 제어
// ==============================================================================

/// 브라우저에서 실행되는 Wasm 메인 루틴
pub fn run() -> Result<(), JsValue> {
    // 1) 패닉 발생 시 브라우저 콘솔에 친절한 스택 트레이스 출력
    console_error_panic_hook::set_once();

    // 2) 브라우저 개발자 도구 콘솔에 로깅
    console::log_1(&"🚀 [Rust Wasm] 브라우저 웹어셈블리 모듈이 성공적으로 로드되었습니다!".into());

    // 3) window와 document 객체 획득
    let window = web_sys::window().expect("전역 window 객체를 찾을 수 없습니다.");
    let document = window.document().expect("document 객체를 찾을 수 없습니다.");

    // ---------------------------------------------------------
    // 기능 1: 인터랙티브 카운터 (Rc<RefCell<i32>> 상태 관리)
    // ---------------------------------------------------------
    let counter_state = Rc::new(RefCell::new(0i32));

    let val_span = document
        .get_element_by_id("counter-val")
        .expect("#counter-val 요소를 찾을 수 없습니다.");

    let btn_inc = document
        .get_element_by_id("btn-inc")
        .expect("#btn-inc 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlButtonElement>()?;

    let btn_dec = document
        .get_element_by_id("btn-dec")
        .expect("#btn-dec 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlButtonElement>()?;

    let btn_reset = document
        .get_element_by_id("btn-reset")
        .expect("#btn-reset 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlButtonElement>()?;

    // +1 버튼 클릭 리스너
    {
        let count = counter_state.clone();
        let val_span = val_span.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            *count.borrow_mut() += 1;
            let current = *count.borrow();
            val_span.set_text_content(Some(&current.to_string()));
            console::log_1(&format!("카운트 증가: {current}").into());
        });
        btn_inc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // -1 버튼 클릭 리스너
    {
        let count = counter_state.clone();
        let val_span = val_span.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            *count.borrow_mut() -= 1;
            let current = *count.borrow();
            val_span.set_text_content(Some(&current.to_string()));
            console::log_1(&format!("카운트 감소: {current}").into());
        });
        btn_dec.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // 초기화 버튼 클릭 리스너
    {
        let count = counter_state.clone();
        let val_span = val_span.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            *count.borrow_mut() = 0;
            val_span.set_text_content(Some("0"));
            console::log_1(&"카운트 초기화".into());
        });
        btn_reset.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // ---------------------------------------------------------
    // 기능 2: 실시간 텍스트 분석기 (Input 이벤트)
    // ---------------------------------------------------------
    let text_input = document
        .get_element_by_id("text-input")
        .expect("#text-input 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlInputElement>()?;

    let out_char = document
        .get_element_by_id("out-char")
        .expect("#out-char 요소를 찾을 수 없습니다.");
    let out_word = document
        .get_element_by_id("out-word")
        .expect("#out-word 요소를 찾을 수 없습니다.");
    let out_reversed = document
        .get_element_by_id("out-reversed")
        .expect("#out-reversed 요소를 찾을 수 없습니다.");
    let out_uppercase = document
        .get_element_by_id("out-uppercase")
        .expect("#out-uppercase 요소를 찾을 수 없습니다.");

    {
        let input_elem = text_input.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let text = input_elem.value();
            let analysis = analyze_text(&text);

            out_char.set_text_content(Some(&analysis.char_count.to_string()));
            out_word.set_text_content(Some(&analysis.word_count.to_string()));
            out_reversed.set_text_content(Some(&analysis.reversed));
            out_uppercase.set_text_content(Some(&analysis.uppercase));
        });
        text_input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // ---------------------------------------------------------
    // 기능 3: 고속 피보나치 연산 데모
    // ---------------------------------------------------------
    let fibo_input = document
        .get_element_by_id("fibo-input")
        .expect("#fibo-input 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlInputElement>()?;

    let btn_fibo = document
        .get_element_by_id("btn-fibo")
        .expect("#btn-fibo 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlButtonElement>()?;

    let fibo_result = document
        .get_element_by_id("fibo-result")
        .expect("#fibo-result 요소를 찾을 수 없습니다.");

    {
        let fibo_input = fibo_input.clone();
        let fibo_result = fibo_result.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let val_str = fibo_input.value();
            let n: u32 = val_str.trim().parse().unwrap_or(0);
            let res = fibonacci(n);
            fibo_result.set_text_content(Some(&format!("{n}번째 피보나치 수 = {res}")));
        });
        btn_fibo.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // ---------------------------------------------------------
    // 기능 4: Rust -> JS alert() 호출 FFI
    // ---------------------------------------------------------
    let btn_alert = document
        .get_element_by_id("btn-alert")
        .expect("#btn-alert 요소를 찾을 수 없습니다.")
        .dyn_into::<HtmlButtonElement>()?;

    {
        let window_clone = window.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let msg = "🦀 안녕하세요! Rust WebAssembly 코드가 브라우저 window.alert()를 직접 호출했습니다!";
            let _ = window_clone.alert_with_message(msg);
        });
        btn_alert.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

// ==============================================================================
// 4. 순수 Rust 단위 테스트
// ==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text_basic() {
        let result = analyze_text("Rust Wasm 짱");
        assert_eq!(result.char_count, 11);
        assert_eq!(result.word_count, 3);
        assert_eq!(result.reversed, "짱 msaW tsuR");
        assert_eq!(result.uppercase, "RUST WASM 짱");
    }

    #[test]
    fn test_fibonacci_correctness() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_format_greeting() {
        assert_eq!(
            format_greeting("페리스"),
            "페리스님, Rust WebAssembly 세계에 오신 것을 환영합니다! 🦀"
        );
        assert_eq!(
            format_greeting("   "),
            "게스트님, Rust WebAssembly 세계에 오신 것을 환영합니다! 🦀"
        );
    }
}

