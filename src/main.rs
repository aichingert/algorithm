use leptos::*;
use leptos_router::*;

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
pub fn Algorithms(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1> "All algorithms" </h1>
        <A href="/algorithms/dijkstra"> "Dijkstra" </A>
    }
}

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
