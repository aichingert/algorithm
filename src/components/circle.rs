use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn Circle(cx: Scope, canvas_ctx: ReadSignal<Option<web_sys::HtmlCanvasElement>>, radius: f32) -> impl IntoView {
    log::debug!("{:?}", canvas_ctx.get());
    if let Some(canvas) = canvas_ctx.get() {
        let canvas = canvas.get_context("2d")
            .unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        canvas.fill_rect(0.0, 0.0, 10.0, 10.0);
    }

    view! { cx,
        <p> {radius} </p>
    }
}

