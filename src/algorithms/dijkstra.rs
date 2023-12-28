use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Dijkstra(cx: Scope) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>(cx);


    canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {
        let canvas_cln: HtmlElement<html::Canvas>= canvas.clone();
        let mut down = false;

        canvas.set_height(800);
        canvas.set_width(800);

        canvas.on(ev::mousedown, move |e| {
            log!("{:?}", "what da hell");
            let js_value = canvas_cln.get_context("2d");
            let x: f64 = e.x().into();
            let y: f64 = e.y().into();

            if let Ok(object) = js_value {
                if let Some(obj) = object {
                    if let Ok(ctx) = obj.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                        ctx.begin_path();
                        _ = ctx.rect(x - 10.0, y - 10., 10., 10.);
                        ctx.stroke();
                    }
                }
            }
        });
    });

    view! { cx,
        <canvas _ref=canvas_ref />
    }
}

