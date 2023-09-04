use leptos::*;
use leptos_router::*;

pub mod dijkstra;
pub use dijkstra::Dijkstra;

#[component]
pub fn Algorithms(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1> "All algorithms" </h1>
        <A href="/algorithms/dijkstra"> "Dijkstra" </A>
    }
}
