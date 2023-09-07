use std::f64::consts::PI;

use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::Circle;

const R: f64 = 32.0;

#[component]
pub fn Dijkstra(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>(cx);

    canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {
        let canvas_cln: HtmlElement<html::Canvas>= canvas.clone();

        canvas.set_height(800);
        canvas.set_width(800);

        canvas.on(ev::click, move |e| {
            let js_value = canvas_cln.get_context("2d");
            let x: f64 = e.x().into();
            let y: f64 = e.y().into();

            if let Ok(object) = js_value {
                if let Some(obj) = object {
                    if let Ok(ctx) = obj.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                        ctx.begin_path();
                        _ = ctx.arc(x - 10.0, y - R, R, 0.0, 2. * PI);
                        ctx.stroke();
                    }
                }
            }
        });
    });

    view! { cx,
        <canvas _ref=canvas_ref />
        <Circle radius=30.0 />
    }
}

