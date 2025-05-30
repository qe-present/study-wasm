use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
#[wasm_bindgen(start)]
fn main()->Result<(),JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;
    Ok(())
}
#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
