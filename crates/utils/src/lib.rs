use wasm_bindgen::prelude::*;
use log::{info, warn, error};  
use web_sys::window;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 暴露一个 greet 函数给 JS 调用
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 使用 web-sys 操作 DOM（可选）
#[wasm_bindgen(start)]
pub fn run() {
    console_log::init().unwrap();

    info!("Hello from Rust WASM!");
    warn!("This is a warning message.");
    error!("This is an error message.");

    // 使用 web-sys 打印到浏览器控制台
    web_sys::console::log_1(&"Hello from Rust WASM!".into());

    let body = window().unwrap().document().unwrap().body().unwrap();
    body.set_text_content(Some("Hello from Rust WASM!"));
}
