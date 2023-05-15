use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::cmd_logger::CmdLogger;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/robots-web.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <body class="bg-slate-800">
            <Router>
                <main>
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    </Routes>
                </main>
            </Router>
        </body>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1 class="text-white text-4xl my-4">"Welcome to Leptos!"</h1>
        <CmdLogger />
    }
}
