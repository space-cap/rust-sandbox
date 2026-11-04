use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, TouchEvent};
use yew::prelude::*;

use crate::particle_system::ParticleUniverse;

#[derive(Properties, PartialEq)]
pub struct CanvasViewProps {
    pub universe: Rc<RefCell<ParticleUniverse>>,
    pub on_fps_update: Callback<u32>,
}

#[function_component(CanvasView)]
pub fn canvas_view(props: &CanvasViewProps) -> Html {
    let canvas_ref = use_node_ref();

    // 60FPS 렌더링 루프 실행
    {
        let canvas_ref = canvas_ref.clone();
        let universe = props.universe.clone();
        let on_fps_update = props.on_fps_update.clone();

        use_effect_with((), move |_| {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("Canvas 요소를 찾을 수 없음");

            // 고해상도 DPI 보정
            let window = web_sys::window().expect("Window 객체 없음");
            let dpr = window.device_pixel_ratio();
            let rect = canvas.get_bounding_client_rect();

            let width = if rect.width() > 0.0 { rect.width() } else { 850.0 };
            let height = if rect.height() > 0.0 { rect.height() } else { 580.0 };

            canvas.set_width((width * dpr) as u32);
            canvas.set_height((height * dpr) as u32);

            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            let _ = ctx.scale(dpr, dpr);
            universe.borrow_mut().resize(width, height);

            // FPS 계산용 상태
            let mut frame_count = 0usize;
            let mut fps_timer = 0.0f64;

            let anim_handle: Rc<RefCell<Option<gloo_render::AnimationFrame>>> =
                Rc::new(RefCell::new(None));
            let anim_handle_clone = anim_handle.clone();

            let render_frame = Rc::new(RefCell::new(None::<Box<dyn FnMut(f64)>>));
            let render_frame_self = render_frame.clone();

            *render_frame.borrow_mut() = Some(Box::new(move |timestamp: f64| {
                frame_count += 1;
                if fps_timer == 0.0 {
                    fps_timer = timestamp;
                }

                if timestamp - fps_timer >= 500.0 {
                    let fps = ((frame_count as f64) * 1000.0 / (timestamp - fps_timer)) as u32;
                    on_fps_update.emit(fps);
                    frame_count = 0;
                    fps_timer = timestamp;
                }

                // 물리 연산 & 화면 그리기
                {
                    let mut u = universe.borrow_mut();
                    u.step();
                    u.render(&ctx);
                }

                // 다음 프레임 예약
                let next_fn = render_frame_self.clone();
                *anim_handle_clone.borrow_mut() = Some(gloo_render::request_animation_frame(
                    move |ts| {
                        if let Some(ref mut f) = *next_fn.borrow_mut() {
                            f(ts);
                        }
                    },
                ));
            }));

            // 첫 프레임 즉시 시작
            let start_handle = render_frame.clone();
            if let Some(ref mut f) = *start_handle.borrow_mut() {
                f(0.0);
            }

            // 컴포넌트 언마운트 시 애니메이션 루프 해제
            move || {
                *anim_handle.borrow_mut() = None;
                *render_frame.borrow_mut() = None;
            }
        });
    }

    // 마우스 이동 핸들러
    let on_mouse_move = {
        let universe = props.universe.clone();
        let canvas_ref = canvas_ref.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let rect = canvas.get_bounding_client_rect();
                let x = (e.client_x() as f64) - rect.left();
                let y = (e.client_y() as f64) - rect.top();
                universe.borrow_mut().set_cursor(x, y, true);
            }
        })
    };

    let on_mouse_enter = {
        let universe = props.universe.clone();
        Callback::from(move |_| {
            let mut u = universe.borrow_mut();
            u.is_hovering = true;
        })
    };

    let on_mouse_leave = {
        let universe = props.universe.clone();
        Callback::from(move |_| {
            let mut u = universe.borrow_mut();
            u.is_hovering = false;
        })
    };

    // 마우스 클릭 시 해당 지점 폭발 효과(Burst)
    let on_click = {
        let universe = props.universe.clone();
        let canvas_ref = canvas_ref.clone();

        Callback::from(move |e: MouseEvent| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let rect = canvas.get_bounding_client_rect();
                let x = (e.client_x() as f64) - rect.left();
                let y = (e.client_y() as f64) - rect.top();
                universe.borrow_mut().burst(x, y, 70);
            }
        })
    };

    // 모바일 터치 이벤트 핸들러
    let on_touch_move = {
        let universe = props.universe.clone();
        let canvas_ref = canvas_ref.clone();

        Callback::from(move |e: TouchEvent| {
            if let Some(touch) = e.touches().get(0) {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let rect = canvas.get_bounding_client_rect();
                    let x = (touch.client_x() as f64) - rect.left();
                    let y = (touch.client_y() as f64) - rect.top();
                    universe.borrow_mut().set_cursor(x, y, true);
                }
            }
        })
    };

    let on_touch_end = {
        let universe = props.universe.clone();
        Callback::from(move |_| {
            let mut u = universe.borrow_mut();
            u.is_hovering = false;
        })
    };

    html! {
        <div class="canvas-wrapper card">
            <canvas
                id="perf-canvas"
                ref={canvas_ref}
                onmousemove={on_mouse_move}
                onmouseenter={on_mouse_enter}
                onmouseleave={on_mouse_leave}
                onclick={on_click}
                ontouchmove={on_touch_move}
                ontouchend={on_touch_end}
            />
            <div class="canvas-hint">
                <span>{ "🖱️ 캔버스 위를 클릭하면 파티클이 폭발(Burst)합니다" }</span>
            </div>
        </div>
    }
}
