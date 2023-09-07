use leptos::*;

#[component]
pub fn Circle(cx: Scope, radius: f32) -> impl IntoView {
    view! { cx,
        <p> {radius} </p>
    }
}

