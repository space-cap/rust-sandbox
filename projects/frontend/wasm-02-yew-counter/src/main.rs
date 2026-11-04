fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<wasm_02_yew_counter::App>::new().render();
}
