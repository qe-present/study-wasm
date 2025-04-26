use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
/// 标记为start 导入就会允许
/// 在js中调用
/// import { run } from './pkg/02_console_log.js';
#[wasm_bindgen(start)]
fn run(){
    println!("use bare bone");
    bare_bones();
    println!("use macro");
    use_macro();
    println!("use web sys");
    use_web_sys();
}
fn bare_bones() {
    log("Hello from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
}
fn  use_macro(){
    console_log!("Hello from macro!");
    console_log!("Hello from macro with number: {}", 42);
    console_log!("Hello from macro with multiple args: {} {}", "arg1", "arg2");
}
fn use_web_sys(){
    use web_sys::console;
    console::log_1(&"Hello from web_sys!".into());
    let js:JsValue=4.into();
    console::log_2(&js,&"Hello from 02_console.log!".into());
}