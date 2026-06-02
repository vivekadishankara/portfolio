use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::pages::{home::HomePage, login::LoginPage, admin::AdminPage};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
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

    let theme = Resource::new(|| (), |_| async {
        crate::api::get_theme().await.unwrap_or_else(|_| "dark-emerald".to_string())
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Syne:wght@400;500;600;700;800&family=DM+Mono:ital,wght@0,300;0,400;0,500;1,300&display=swap"
            rel="stylesheet"
        />
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Suspense fallback=|| ()>
            {move || theme.get().map(|t| view! { <Html attr:data-theme=t/> })}
        </Suspense>

        <Router>
            <Routes fallback=|| view! {
                <div class="min-h-screen t-bg-primary flex items-center justify-center">
                    <p class="t-text-muted font-mono">"404 – Page not found"</p>
                </div>
            }>
                <Route path=path!("/")            view=HomePage/>
                <Route path=path!("/admin")       view=AdminPage/>
                <Route path=path!("/admin/login") view=LoginPage/>
            </Routes>
        </Router>
    }
}
