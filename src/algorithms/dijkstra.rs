use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::Circle;

#[component]
pub fn Dijkstra(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>(cx);

    canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {
        let js_value = canvas.get_context("2d");

        if let Ok(object) = js_value {
            if let Some(obj) = object {
                if let Ok(ctx) = obj.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                    ctx.fill_rect(100.0, 100.0, 200.0, 200.0);
                }

            }
        }
    });

    view! { cx,
        <canvas _ref=canvas_ref />
        <Circle radius=30.0 />
    }
}

