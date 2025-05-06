mod pages;
use pages::HomePage;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-ssr.css" />

        <Title formatter=|title| format!("{title} - leptos ssr") />
        <Router>
            <main>
                <header>
                    <nav>
                        <button class="bg-red-500 rounded text-white px-2 py-2 hover:bg-blue-500">GOOD HEADER</button>
                    </nav>
                </header>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}
