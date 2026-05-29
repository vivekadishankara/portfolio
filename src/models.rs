use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub title: String,
    pub bio: String,
    pub summary: String,
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub twitter: String,
    pub avatar_url: String,
    pub location: String,
    pub resume_url: String,
    pub theme: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: "profile".to_string(),
            name: "Your Name".to_string(),
            title: "Software Engineer".to_string(),
            bio: "Passionate developer building elegant solutions to complex problems.".to_string(),
            summary: String::new(),
            email: "you@example.com".to_string(),
            github: "https://github.com/yourname".to_string(),
            linkedin: "https://linkedin.com/in/yourname".to_string(),
            twitter: "https://twitter.com/yourname".to_string(),
            avatar_url: "".to_string(),
            location: "Your City, Country".to_string(),
            resume_url: "".to_string(),
            theme: "dark-emerald".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub id: String,
    pub company: String,
    pub role: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub current: bool,
    pub description: Vec<String>,
    pub technologies: Vec<String>,
    pub order_index: i32,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company: String::new(),
            role: String::new(),
            start_date: String::new(),
            end_date: None,
            current: false,
            description: vec![],
            technologies: vec![],
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Education {
    pub id: String,
    pub institution: String,
    pub degree: String,
    pub field: String,
    pub start_year: String,
    pub end_year: Option<String>,
    pub current: bool,
    pub description: String,
    pub gpa: Option<String>,
    pub order_index: i32,
}

impl Default for Education {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            institution: String::new(),
            degree: String::new(),
            field: String::new(),
            start_year: String::new(),
            end_year: None,
            current: false,
            description: String::new(),
            gpa: None,
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub long_description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub image_url: Option<String>,
    pub featured: bool,
    pub order_index: i32,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            description: String::new(),
            long_description: String::new(),
            technologies: vec![],
            github_url: None,
            live_url: None,
            image_url: None,
            featured: false,
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub category: String,
    pub level: i32, // 1-5
    pub order_index: i32,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            category: "Languages".to_string(),
            level: 3,
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Certification {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub date: String,
    pub expiry_date: Option<String>,
    pub credential_url: Option<String>,
    pub order_index: i32,
}

impl Default for Certification {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            issuer: String::new(),
            date: String::new(),
            expiry_date: None,
            credential_url: None,
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AdminCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PortfolioData {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub educations: Vec<Education>,
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
    pub certifications: Vec<Certification>,
}

impl Default for PortfolioData {
    fn default() -> Self {
        Self {
            profile: Profile::default(),
            experiences: vec![],
            educations: vec![],
            projects: vec![],
            skills: vec![],
            certifications: vec![],
        }
    }
}
