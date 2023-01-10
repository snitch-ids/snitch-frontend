use function_router::services::backend;
use wasm_bindgen_test::*;
use web_sys::console::log_1;

#[wasm_bindgen_test]
fn pass() {
    backend::authenticate();
}
