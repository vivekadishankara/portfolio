pub mod db;
pub mod auth;

pub use runner::run;

mod runner {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;
    use tower_http::cors::CorsLayer;

    use crate::app::{App, shell};
    use super::db::Database;

    pub async fn run() {
        tracing_subscriber::fmt::init();
        dotenvy::dotenv().ok();

        let db = Database::new().await.expect("Failed to connect to database");
        db.run_migrations().await.expect("Failed to run migrations");

        let conf = get_configuration(None).unwrap();
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options.clone();
        let routes = generate_route_list(App);
        let db_for_context = db.clone();

        let app = Router::new()
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                move || {
                    provide_context(db_for_context.clone());
                },
                {
                    let opts = leptos_options.clone();
                    move || shell(opts.clone())
                },
            )
            .fallback(leptos_axum::file_and_error_handler({
                let opts = leptos_options.clone();
                move |_| shell(opts.clone())
            }))
            .nest_service("/assets", ServeDir::new("assets"))
            .layer(CorsLayer::permissive())
            .with_state(leptos_options);

        tracing::info!("Listening on http://{}", addr);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service()).await.unwrap();
    }
}