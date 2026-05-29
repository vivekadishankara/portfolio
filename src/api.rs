//! Server functions — compiled on BOTH ssr and hydrate targets.
//! The #[server] macro generates client-side fetch stubs automatically for WASM.

use leptos::prelude::*;
use crate::models::*;

// ─── Public ──────────────────────────────────────────────────────────────────

#[server(GetPortfolioData, "/api")]
pub async fn get_portfolio_data() -> Result<PortfolioData, ServerFnError> {
    use crate::server::db::Database;
    let db = use_context::<Database>()
        .ok_or_else(|| ServerFnError::new("No DB context"))?;

    let profile       = db.get_profile().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let experiences   = db.get_experiences().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let educations    = db.get_educations().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let projects      = db.get_projects().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let skills        = db.get_skills().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let certifications = db.get_certifications().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(PortfolioData { profile, experiences, educations, projects, skills, certifications })
}

// ─── Auth ────────────────────────────────────────────────────────────────────

#[server(AdminLogin, "/api")]
pub async fn admin_login(username: String, password: String) -> Result<LoginResponse, ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>()
        .ok_or_else(|| ServerFnError::new("No DB context"))?;

    let valid = db.verify_admin(&username, &password)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if valid {
        let token = auth::generate_token(&username)
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(LoginResponse { success: true, token: Some(token), message: "Login successful".to_string() })
    } else {
        Ok(LoginResponse { success: false, token: None, message: "Invalid credentials".to_string() })
    }
}

// ─── Profile ─────────────────────────────────────────────────────────────────

#[server(UpdateProfile, "/api")]
pub async fn update_profile(token: String, profile: Profile) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.update_profile(&profile).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Experience ──────────────────────────────────────────────────────────────

#[server(UpsertExperience, "/api")]
pub async fn upsert_experience(token: String, experience: Experience) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.upsert_experience(&experience).await.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteExperience, "/api")]
pub async fn delete_experience(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.delete_experience(&id).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Education ───────────────────────────────────────────────────────────────

#[server(UpsertEducation, "/api")]
pub async fn upsert_education(token: String, education: Education) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.upsert_education(&education).await.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteEducation, "/api")]
pub async fn delete_education(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.delete_education(&id).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Projects ────────────────────────────────────────────────────────────────

#[server(UpsertProject, "/api")]
pub async fn upsert_project(token: String, project: Project) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.upsert_project(&project).await.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteProject, "/api")]
pub async fn delete_project(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.delete_project(&id).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Skills ──────────────────────────────────────────────────────────────────

#[server(UpsertSkill, "/api")]
pub async fn upsert_skill(token: String, skill: Skill) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.upsert_skill(&skill).await.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteSkill, "/api")]
pub async fn delete_skill(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.delete_skill(&id).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Certifications ──────────────────────────────────────────────────────────

#[server(UpsertCertification, "/api")]
pub async fn upsert_certification(token: String, certification: Certification) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.upsert_certification(&certification).await.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(DeleteCertification, "/api")]
pub async fn delete_certification(token: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.delete_certification(&id).await.map_err(|e| ServerFnError::new(e.to_string()))
}

// ─── Change Password ─────────────────────────────────────────────────────────

#[server(ChangePassword, "/api")]
pub async fn change_password(token: String, username: String, new_password: String) -> Result<(), ServerFnError> {
    use crate::server::{db::Database, auth};
    let db = use_context::<Database>().ok_or_else(|| ServerFnError::new("No DB context"))?;
    if !auth::is_valid_token(&token) { return Err(ServerFnError::new("Unauthorized")); }
    db.change_password(&username, &new_password).await.map_err(|e| ServerFnError::new(e.to_string()))
}
