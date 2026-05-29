use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::pages::{
    home::HomePage,
    admin::AdminPage,
    login::LoginPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <AutoReload options=options.clone()/>   // hot reload in dev
                <HydrationScripts options=options/>     // injects WASM scripts
                <MetaTags/>                             // where Title/Stylesheet inject
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        // <Link href="https://fonts.googleapis.com/css2?family=Syne:wght@400;500;600;700;800&family=DM+Mono:ital,wght@0,300;0,400;0,500;1,300&display=swap" rel="stylesheet"/>
        <Title text="Portfolio"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Router>
            <Routes fallback=|| view! { <p class="text-center p-8 text-red-400">"404 – Page not found"</p> }>
                <Route path=path!("/") view=HomePage/>
                <Route path=path!("/admin") view=AdminPage/>
                <Route path=path!("/admin/login") view=LoginPage/>
            </Routes>
        </Router>
    }
}
