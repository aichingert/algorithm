use leptos::*;

use crate::components::Circle;

#[component]
pub fn Dijkstra(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1> "D" </h1>
        <h1> "i" </h1>
        <h1> "j" </h1>
        <h1> "k" </h1>
        <h1> "s" </h1>
        <h1> "t" </h1>
        <h1> "r" </h1>
        <h1> "a" </h1>
        <Circle radius=30.0 />
    }
}

