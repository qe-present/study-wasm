use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/tt.js")]
extern "C"{
    fn name() -> String;
    type MyClass;
    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;
    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32)->MyClass;
    
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
    
}
#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}
#[wasm_bindgen(start)]
pub fn run(){
    log(&format!("Hello from {}!", name()));
    let x = MyClass::new();
    assert_eq!(x.number(), 42);
    x.set_number(10);
    log(&x.render());
}