#![allow(unused_variables)]

use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

fn main() {

    #[wasm_bindgen(start)]
    fn run() -> Result<(), JsValue> {
        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        // One of the first interesting things we can do with closures is simply
        // access stack data in Rust!
        let array = Array::new();
        array.push(&"Hello".into());
        array.push(&1.into());
        let mut first_item = None;
        array.for_each(&mut |obj, idx, _arr| match idx {
            0 => {
                assert_eq!(obj, "Hello");
                first_item = obj.as_string();
            }
            1 => assert_eq!(obj, 1),
            _ => panic!("unknown index: {}", idx),
        });
        assert_eq!(first_item, Some("Hello".to_string()));

        // Below are some more advanced usages of the `Closure` type for closures
        // that need to live beyond our function call.

        setup_clock(&window, &document)?;
        setup_clicker(&document);

        // And now that our demo is ready to go let's switch things up so
        // everything is displayed and our loading prompt is hidden.
        document
            .get_element_by_id("loading")
            .expect("should have #loading on the page")
            .dyn_ref::<HtmlElement>()
            .expect("#loading should be an `HtmlElement`")
            .style()
            .set_property("display", "none")?;
        document
            .get_element_by_id("script")
            .expect("should have #script on the page")
            .dyn_ref::<HtmlElement>()
            .expect("#script should be an `HtmlElement`")
            .style()
            .set_property("display", "block")?;

        Ok(())
    }


    fn setup_clock(window: &Window, document: &Document) -> Result<(), JsValue> {
        let current_time = document
            .get_element_by_id("current-time")
            .expect("should have #current-time on the page");
        update_time(&current_time);
        let a = Closure::<dyn Fn()>::new(move || update_time(&current_time));
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
        fn update_time(current_time: &Element) {
            current_time.set_inner_html(&String::from(
                Date::new_0().to_locale_string("en-GB", &JsValue::undefined()),
            ));
        }

        a.forget();

        Ok(())
    }


    fn setup_clicker(document: &Document) {
        let num_clicks = document
            .get_element_by_id("num-clicks")
            .expect("should have #num-clicks on the page");
        let mut clicks = 0;
        let a = Closure::<dyn FnMut()>::new(move || {
            clicks += 1;
            num_clicks.set_inner_html(&clicks.to_string());
        });
        document
            .get_element_by_id("green-square")
            .expect("should have #green-square on the page")
            .dyn_ref::<HtmlElement>()
            .expect("#green-square be an `HtmlElement`")
            .set_onclick(Some(a.as_ref().unchecked_ref()));

        a.forget();
    }

}