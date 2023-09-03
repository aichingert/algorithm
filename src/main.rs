use leptos::*;
use leptos_router::*;

#[component]
pub fn Routes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <nav>
                <A exact=true href="/">Algorithms</A>
                <A href="about">About</A>
            </nav>
            <main>
                <AlgorithmRoutes/>
                <Route
                    path="about"
                    view=|cx| view! { cx, <About/> }
                />
                <About/>
            </main>
        </Router>
    }
}

#[component(transparent)]
pub fn AlgorithmRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route
            path=""
            view=|cx| view! { cx, <h1> "Hi" </h1> }
        />
    }
}

#[component]
pub fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1> "Algorithm" </h1>

        <p> "This project is done to make it easier to understand algorithms" </p>
        <p> "the ones I will visualize are the ones I find to be interesting" </p>
        <p> "and maybe some of my own from the advent of code problems.     " </p>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <Routes/> })
}
