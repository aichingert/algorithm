use leptos::*;
use leptos_router::*;

pub mod components;

pub mod algorithms;
pub use algorithms::{Algorithms, Dijkstra};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    log::debug!("rendering routes");

    view! { cx,
        <Router>
            <nav>
                <A href="/"> "Home" </A>
                <A href="/algorithms"> "Algorithms" </A>
            </nav>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=Home
                    />
                    <Route
                        path="algorithms"
                        view=Algorithms
                    />
                    <Route
                        path="algorithms/dijkstra"
                        view=Dijkstra
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    log::debug!("rendering about");
    view! { cx,
        <h1> "Algorithm" </h1>

        <p> "This project is done to make it easier to understand algorithms" </p>
        <p> "the ones I will visualize are the ones I find to be interesting" </p>
        <p> "and maybe some of my own from the advent of code problems.     " </p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|cx| view! { cx, <App/> })
}
