pub mod db;
pub mod auth;
pub mod pdf;

pub use runner::run;

mod runner {
    use axum::{
        Router,
        routing::get,
        extract::State,
        response::{IntoResponse, Response},
        http::{StatusCode, header},
    };
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeDir;
    use tower_http::cors::CorsLayer;

    use crate::app::{App, shell};
    use crate::models::PortfolioData;
    use super::db::Database;
    use super::pdf;

    // ── PDF handler ──────────────────────────────────────────────────────────

    async fn resume_pdf_handler(State(db): State<Database>) -> Response {
        // Fetch all portfolio data from the DB
        let data = match async {
            let profile       = db.get_profile().await?;
            let experiences   = db.get_experiences().await?;
            let educations    = db.get_educations().await?;
            let projects      = db.get_projects().await?;
            let skills        = db.get_skills().await?;
            let certifications = db.get_certifications().await?;
            Ok::<PortfolioData, sqlx::Error>(PortfolioData {
                profile, experiences, educations, projects, skills, certifications,
            })
        }.await {
            Ok(d)  => d,
            Err(e) => {
                tracing::error!("DB error building PDF: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {e}"),
                ).into_response();
            }
        };

        match pdf::generate_pdf(&data).await {
            Ok(bytes) => {
                let filename = format!(
                    "attachment; filename=\"{}_resume.pdf\"",
                    data.profile.name
                        .to_lowercase()
                        .replace(' ', "_")
                );
                (
                    [
                        (header::CONTENT_TYPE,        "application/pdf".to_string()),
                        (header::CONTENT_DISPOSITION, filename),
                    ],
                    bytes,
                ).into_response()
            }
            Err(e) => {
                tracing::error!("PDF generation failed: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("PDF generation failed: {e}"),
                ).into_response()
            }
        }
    }

    // ── Server entry point ───────────────────────────────────────────────────

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
            // ── PDF download endpoint (plain Axum, not a Leptos server fn) ──
            .route("/api/resume.pdf", get(resume_pdf_handler))
            .with_state(db.clone())
            // ── Leptos routes ───────────────────────────────────────────────
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