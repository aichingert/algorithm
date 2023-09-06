use leptos::*;
use wasm_bindgen::prelude::*;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(element) = document.get_element_by_id("canvas") {
        Some(element.dyn_into().unwrap())
    } else {
        None
    }
}
