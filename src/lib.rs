#![feature(lazy_cell)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fallback;
mod routes;
use routes::demo::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/hackernews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="description" content="Leptos implementation of a HackerNews demo."/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Page ssr=SsrMode::InOrder/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
