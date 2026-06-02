use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Default value helpers for serde ──────────────────────────────────────────

fn default_id()            -> String { "profile".to_string() }
fn default_title()         -> String { "Software Engineer".to_string() }
fn default_bio()           -> String { "Passionate developer building elegant solutions to complex problems.".to_string() }
fn default_email()         -> String { "you@example.com".to_string() }
fn default_github()        -> String { "https://github.com/yourname".to_string() }
fn default_linkedin()      -> String { "https://linkedin.com/in/yourname".to_string() }
fn default_twitter()       -> String { "https://twitter.com/yourname".to_string() }
fn default_location()      -> String { "Your City, Country".to_string() }
fn default_theme()         -> String { "dark-emerald".to_string() }
fn default_section_order() -> String { "experience,projects,skills,education,certifications".to_string() }
fn default_name()          -> String { "Your Name".to_string() }
fn default_skill_cat()     -> String { "Languages".to_string() }
fn default_skill_level()   -> i32    { 3 }
fn new_uuid()              -> String { Uuid::new_v4().to_string() }

// ── Models ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    #[serde(default = "default_id")]
    pub id: String,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_bio")]
    pub bio: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default = "default_email")]
    pub email: String,
    #[serde(default = "default_github")]
    pub github: String,
    #[serde(default = "default_linkedin")]
    pub linkedin: String,
    #[serde(default = "default_twitter")]
    pub twitter: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default = "default_location")]
    pub location: String,
    #[serde(default)]
    pub resume_url: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_section_order")]
    pub section_order: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id:            default_id(),
            name:          default_name(),
            title:         default_title(),
            bio:           default_bio(),
            summary:       String::new(),
            email:         default_email(),
            github:        default_github(),
            linkedin:      default_linkedin(),
            twitter:       default_twitter(),
            avatar_url:    String::new(),
            location:      default_location(),
            resume_url:    String::new(),
            theme:         default_theme(),
            section_order: default_section_order(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    #[serde(default = "new_uuid")]
    pub id: String,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(default)]
    pub order_index: i32,
}

impl Default for Experience {
    fn default() -> Self {
        Self {
            id:           new_uuid(),
            company:      String::new(),
            role:         String::new(),
            start_date:   String::new(),
            end_date:     None,
            current:      false,
            description:  vec![],
            technologies: vec![],
            order_index:  0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Education {
    #[serde(default = "new_uuid")]
    pub id: String,
    #[serde(default)]
    pub institution: String,
    #[serde(default)]
    pub degree: String,
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub start_year: String,
    #[serde(default)]
    pub end_year: Option<String>,
    #[serde(default)]
    pub current: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub gpa: Option<String>,
    #[serde(default)]
    pub order_index: i32,
}

impl Default for Education {
    fn default() -> Self {
        Self {
            id:          new_uuid(),
            institution: String::new(),
            degree:      String::new(),
            field:       String::new(),
            start_year:  String::new(),
            end_year:    None,
            current:     false,
            description: String::new(),
            gpa:         None,
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    #[serde(default = "new_uuid")]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub long_description: String,
    #[serde(default)]
    pub technologies: Vec<String>,
    #[serde(default)]
    pub github_url: Option<String>,
    #[serde(default)]
    pub live_url: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub order_index: i32,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id:               new_uuid(),
            title:            String::new(),
            description:      String::new(),
            long_description: String::new(),
            technologies:     vec![],
            github_url:       None,
            live_url:         None,
            image_url:        None,
            featured:         false,
            order_index:      0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    #[serde(default = "new_uuid")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_skill_cat")]
    pub category: String,
    #[serde(default = "default_skill_level")]
    pub level: i32,
    #[serde(default)]
    pub order_index: i32,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            id:          new_uuid(),
            name:        String::new(),
            category:    default_skill_cat(),
            level:       default_skill_level(),
            order_index: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Certification {
    #[serde(default = "new_uuid")]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub expiry_date: Option<String>,
    #[serde(default)]
    pub credential_url: Option<String>,
    #[serde(default)]
    pub order_index: i32,
}

impl Default for Certification {
    fn default() -> Self {
        Self {
            id:             new_uuid(),
            name:           String::new(),
            issuer:         String::new(),
            date:           String::new(),
            expiry_date:    None,
            credential_url: None,
            order_index:    0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub success: bool,
    pub token:   Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct PortfolioData {
    pub profile:          Profile,
    pub experiences:      Vec<Experience>,
    pub educations:       Vec<Education>,
    pub projects:         Vec<Project>,
    pub skills:           Vec<Skill>,
    pub certifications:   Vec<Certification>,
}
