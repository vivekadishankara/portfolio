//! Server functions — compiled on BOTH ssr and hydrate targets.
//! The `#[server]` macro generates a client-side fetch stub for WASM
//! and the real implementation for SSR.

use leptos::prelude::*;
use crate::models::*;

// ── SSR-only helpers (not exposed to WASM) ────────────────────────────────────

#[cfg(feature = "ssr")]
fn db() -> Result<crate::server::db::Database, ServerFnError> {
    use_context::<crate::server::db::Database>()
        .ok_or_else(|| ServerFnError::new("No database context"))
}

#[cfg(feature = "ssr")]
fn require_auth(token: &str) -> Result<(), ServerFnError> {
    if crate::server::auth::is_valid_token(token) {
        Ok(())
    } else {
        Err(ServerFnError::new("Unauthorized"))
    }
}

// ── Public data ───────────────────────────────────────────────────────────────

#[server(GetPortfolioData, "/api")]
pub async fn get_portfolio_data() -> Result<PortfolioData, ServerFnError> {
    use crate::error::IntoServerError;
    let db = db()?;
    Ok(PortfolioData {
        profile:        db.get_profile().await.server_err()?,
        experiences:    db.get_experiences().await.server_err()?,
        educations:     db.get_educations().await.server_err()?,
        projects:       db.get_projects().await.server_err()?,
        skills:         db.get_skills().await.server_err()?,
        certifications: db.get_certifications().await.server_err()?,
    })
}

#[server(GetTheme, "/api")]
pub async fn get_theme() -> Result<String, ServerFnError> {
    use crate::error::IntoServerError;
    Ok(db()?.get_profile().await.server_err()?.theme)
}

// ── Auth ──────────────────────────────────────────────────────────────────────

#[server(AdminLogin, "/api")]
pub async fn admin_login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    use crate::{error::IntoServerError, server::auth};
    let valid = db()?.verify_admin(&username, &password).await.server_err()?;
    if valid {
        let token = auth::generate_token(&username).server_err()?;
        Ok(LoginResponse { success: true, token: Some(token), message: "Login successful".into() })
    } else {
        Ok(LoginResponse { success: false, token: None, message: "Invalid credentials".into() })
    }
}

// ── Profile ───────────────────────────────────────────────────────────────────

#[server(UpdateProfile, "/api")]
pub async fn update_profile(token: String, profile: Profile) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.update_profile(&profile).await.server_err()
}

#[server(SaveTheme, "/api")]
pub async fn save_theme(token: String, theme: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.save_theme(&theme).await.server_err()
}

// ── Experience ────────────────────────────────────────────────────────────────

#[server(UpsertExperience, "/api")]
pub async fn upsert_experience(token: String, experience: Experience) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.upsert_experience(&experience).await.server_err()
}

#[server(DeleteExperience, "/api")]
pub async fn delete_experience(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.delete_experience(&id).await.server_err()
}

// ── Education ─────────────────────────────────────────────────────────────────

#[server(UpsertEducation, "/api")]
pub async fn upsert_education(token: String, education: Education) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.upsert_education(&education).await.server_err()
}

#[server(DeleteEducation, "/api")]
pub async fn delete_education(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.delete_education(&id).await.server_err()
}

// ── Projects ──────────────────────────────────────────────────────────────────

#[server(UpsertProject, "/api")]
pub async fn upsert_project(token: String, project: Project) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.upsert_project(&project).await.server_err()
}

#[server(DeleteProject, "/api")]
pub async fn delete_project(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.delete_project(&id).await.server_err()
}

// ── Skills ────────────────────────────────────────────────────────────────────

#[server(UpsertSkill, "/api")]
pub async fn upsert_skill(token: String, skill: Skill) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.upsert_skill(&skill).await.server_err()
}

#[server(DeleteSkill, "/api")]
pub async fn delete_skill(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.delete_skill(&id).await.server_err()
}

// ── Certifications ────────────────────────────────────────────────────────────

#[server(UpsertCertification, "/api")]
pub async fn upsert_certification(token: String, certification: Certification) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.upsert_certification(&certification).await.server_err()
}

#[server(DeleteCertification, "/api")]
pub async fn delete_certification(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.delete_certification(&id).await.server_err()
}

// ── Settings ──────────────────────────────────────────────────────────────────

#[server(ChangePassword, "/api")]
pub async fn change_password(token: String, username: String, new_password: String) -> Result<(), ServerFnError> {
    use crate::error::IntoServerError;
    require_auth(&token)?;
    db()?.change_password(&username, &new_password).await.server_err()
}
