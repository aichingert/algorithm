use leptos::*;

use crate::components::Circle;
use crate::common::canvas::get_canvas;

#[component]
pub fn Dijkstra(cx: Scope) -> impl IntoView {
    let (ctx, set_ctx) = create_signal(cx, get_canvas());

    view! { cx,
        <button on:click=move |_| set_ctx.update(|ctx| *ctx = get_canvas())>"draw"</button>

        <canvas id="canvas" />
        <Circle canvas_ctx=ctx radius=30.0 />
    }
}

