use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row};
use crate::models::*;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            let mut path = std::env::current_dir().unwrap_or_default();
            path.push("portfolio.db");
            format!("sqlite://{}?mode=rwc", path.display())
        });

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS profile (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL DEFAULT '',
                title TEXT NOT NULL DEFAULT '',
                bio TEXT NOT NULL DEFAULT '',
                email TEXT NOT NULL DEFAULT '',
                github TEXT NOT NULL DEFAULT '',
                linkedin TEXT NOT NULL DEFAULT '',
                twitter TEXT NOT NULL DEFAULT '',
                avatar_url TEXT NOT NULL DEFAULT '',
                location TEXT NOT NULL DEFAULT '',
                resume_url TEXT NOT NULL DEFAULT '',
                summary TEXT NOT NULL DEFAULT '',
                theme TEXT NOT NULL DEFAULT 'dark-emerald',
                section_order TEXT NOT NULL DEFAULT 'experience,projects,skills,education,certifications'
            )"
        ).execute(&self.pool).await?;

        // Run schema migration for existing databases
        let _ = sqlx::query(
            "ALTER TABLE profile ADD COLUMN section_order TEXT NOT NULL DEFAULT 'experience,projects,skills,education,certifications'"
        ).execute(&self.pool).await;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS experiences (
                id TEXT PRIMARY KEY,
                company TEXT NOT NULL,
                role TEXT NOT NULL,
                start_date TEXT NOT NULL,
                end_date TEXT,
                current INTEGER NOT NULL DEFAULT 0,
                description TEXT NOT NULL DEFAULT '[]',
                technologies TEXT NOT NULL DEFAULT '[]',
                order_index INTEGER NOT NULL DEFAULT 0
            )"
        ).execute(&self.pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS educations (
                id TEXT PRIMARY KEY,
                institution TEXT NOT NULL,
                degree TEXT NOT NULL,
                field TEXT NOT NULL,
                start_year TEXT NOT NULL,
                end_year TEXT,
                current INTEGER NOT NULL DEFAULT 0,
                description TEXT NOT NULL DEFAULT '',
                gpa TEXT,
                order_index INTEGER NOT NULL DEFAULT 0
            )"
        ).execute(&self.pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                long_description TEXT NOT NULL DEFAULT '',
                technologies TEXT NOT NULL DEFAULT '[]',
                github_url TEXT,
                live_url TEXT,
                image_url TEXT,
                featured INTEGER NOT NULL DEFAULT 0,
                order_index INTEGER NOT NULL DEFAULT 0
            )"
        ).execute(&self.pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS skills (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL DEFAULT 'Languages',
                level INTEGER NOT NULL DEFAULT 3,
                order_index INTEGER NOT NULL DEFAULT 0
            )"
        ).execute(&self.pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS certifications (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                issuer TEXT NOT NULL,
                date TEXT NOT NULL,
                expiry_date TEXT,
                credential_url TEXT,
                order_index INTEGER NOT NULL DEFAULT 0
            )"
        ).execute(&self.pool).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            )"
        ).execute(&self.pool).await?;

        // Seed default admin
        let admin_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admin")
            .fetch_one(&self.pool).await?;

        if admin_count == 0 {
            let hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST).unwrap();
            sqlx::query("INSERT INTO admin (username, password_hash) VALUES (?, ?)")
                .bind("admin")
                .bind(&hash)
                .execute(&self.pool).await?;
        }

        // Seed default profile
        let profile_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM profile")
            .fetch_one(&self.pool).await?;

        if profile_count == 0 {
            sqlx::query(
                "INSERT INTO profile (id, name, title, bio, summary, email, github, linkedin, twitter, avatar_url, location, resume_url, theme, section_order)
                 VALUES ('profile', 'Your Name', 'Full-Stack Engineer',
                         'Passionate developer building elegant solutions to complex problems.',
                         '',
                         'you@example.com', 'https://github.com/yourname',
                         'https://linkedin.com/in/yourname', 'https://twitter.com/yourname',
                         '', 'Your City, Country', '', 'dark-emerald', 'experience,projects,skills,education,certifications')"
            ).execute(&self.pool).await?;
        }

        Ok(())
    }

    // ─── Profile ─────────────────────────────────────────────────────────────

    pub async fn get_profile(&self) -> Result<Profile, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, name, title, bio, summary, email, github, linkedin, twitter, avatar_url, location, resume_url, theme, section_order
             FROM profile WHERE id = 'profile'"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Profile {
            id:            row.get("id"),
            name:          row.get("name"),
            title:         row.get("title"),
            bio:           row.get("bio"),
            email:         row.get("email"),
            github:        row.get("github"),
            linkedin:      row.get("linkedin"),
            twitter:       row.get("twitter"),
            avatar_url:    row.get("avatar_url"),
            location:      row.get("location"),
            resume_url:    row.get("resume_url"),
            summary:       row.get("summary"),
            theme:         row.get("theme"),
            section_order: row.get("section_order"),
        })
    }

    pub async fn update_profile(&self, p: &Profile) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO profile (id, name, title, bio, summary, email, github, linkedin, twitter, avatar_url, location, resume_url, theme, section_order)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               name=excluded.name, title=excluded.title, bio=excluded.bio, summary=excluded.summary,
               email=excluded.email, github=excluded.github, linkedin=excluded.linkedin,
               twitter=excluded.twitter, avatar_url=excluded.avatar_url,
               location=excluded.location, resume_url=excluded.resume_url, theme=excluded.theme,
               section_order=excluded.section_order"
        )
        .bind(&p.id).bind(&p.name).bind(&p.title).bind(&p.bio).bind(&p.summary)
        .bind(&p.email).bind(&p.github).bind(&p.linkedin).bind(&p.twitter)
        .bind(&p.avatar_url).bind(&p.location).bind(&p.resume_url).bind(&p.theme)
        .bind(&p.section_order)
        .execute(&self.pool).await?;
        Ok(())
    }

    // ─── Experiences ─────────────────────────────────────────────────────────

    pub async fn get_experiences(&self) -> Result<Vec<Experience>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, company, role, start_date, end_date, current, description, technologies, order_index
             FROM experiences ORDER BY order_index ASC"
        )
        .fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| Experience {
            id:           r.get("id"),
            company:      r.get("company"),
            role:         r.get("role"),
            start_date:   r.get("start_date"),
            end_date:     r.get("end_date"),
            current:      r.get::<i64, _>("current") != 0,
            description:  serde_json::from_str(r.get("description")).unwrap_or_default(),
            technologies: serde_json::from_str(r.get("technologies")).unwrap_or_default(),
            order_index:  r.get::<i64, _>("order_index") as i32,
        }).collect())
    }

    pub async fn upsert_experience(&self, e: &Experience) -> Result<(), sqlx::Error> {
        let desc = serde_json::to_string(&e.description).unwrap_or_default();
        let techs = serde_json::to_string(&e.technologies).unwrap_or_default();
        sqlx::query(
            "INSERT INTO experiences (id, company, role, start_date, end_date, current, description, technologies, order_index)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               company=excluded.company, role=excluded.role, start_date=excluded.start_date,
               end_date=excluded.end_date, current=excluded.current,
               description=excluded.description, technologies=excluded.technologies,
               order_index=excluded.order_index"
        )
        .bind(&e.id).bind(&e.company).bind(&e.role).bind(&e.start_date)
        .bind(&e.end_date).bind(e.current as i64).bind(&desc)
        .bind(&techs).bind(e.order_index)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_experience(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM experiences WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    // ─── Educations ──────────────────────────────────────────────────────────

    pub async fn get_educations(&self) -> Result<Vec<Education>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, institution, degree, field, start_year, end_year, current, description, gpa, order_index
             FROM educations ORDER BY order_index ASC"
        )
        .fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| Education {
            id:          r.get("id"),
            institution: r.get("institution"),
            degree:      r.get("degree"),
            field:       r.get("field"),
            start_year:  r.get("start_year"),
            end_year:    r.get("end_year"),
            current:     r.get::<i64, _>("current") != 0,
            description: r.get("description"),
            gpa:         r.get("gpa"),
            order_index: r.get::<i64, _>("order_index") as i32,
        }).collect())
    }

    pub async fn upsert_education(&self, e: &Education) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO educations (id, institution, degree, field, start_year, end_year, current, description, gpa, order_index)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               institution=excluded.institution, degree=excluded.degree, field=excluded.field,
               start_year=excluded.start_year, end_year=excluded.end_year,
               current=excluded.current, description=excluded.description,
               gpa=excluded.gpa, order_index=excluded.order_index"
        )
        .bind(&e.id).bind(&e.institution).bind(&e.degree).bind(&e.field)
        .bind(&e.start_year).bind(&e.end_year).bind(e.current as i64)
        .bind(&e.description).bind(&e.gpa).bind(e.order_index)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_education(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM educations WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    // ─── Projects ────────────────────────────────────────────────────────────

    pub async fn get_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, title, description, long_description, technologies, github_url, live_url, image_url, featured, order_index
             FROM projects ORDER BY order_index ASC"
        )
        .fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| Project {
            id:               r.get("id"),
            title:            r.get("title"),
            description:      r.get("description"),
            long_description: r.get("long_description"),
            technologies:     serde_json::from_str(r.get("technologies")).unwrap_or_default(),
            github_url:       r.get("github_url"),
            live_url:         r.get("live_url"),
            image_url:        r.get("image_url"),
            featured:         r.get::<i64, _>("featured") != 0,
            order_index:      r.get::<i64, _>("order_index") as i32,
        }).collect())
    }

    pub async fn upsert_project(&self, p: &Project) -> Result<(), sqlx::Error> {
        let techs = serde_json::to_string(&p.technologies).unwrap_or_default();
        sqlx::query(
            "INSERT INTO projects (id, title, description, long_description, technologies, github_url, live_url, image_url, featured, order_index)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               title=excluded.title, description=excluded.description,
               long_description=excluded.long_description, technologies=excluded.technologies,
               github_url=excluded.github_url, live_url=excluded.live_url,
               image_url=excluded.image_url, featured=excluded.featured,
               order_index=excluded.order_index"
        )
        .bind(&p.id).bind(&p.title).bind(&p.description).bind(&p.long_description)
        .bind(&techs).bind(&p.github_url).bind(&p.live_url).bind(&p.image_url)
        .bind(p.featured as i64).bind(p.order_index)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_project(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM projects WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    // ─── Skills ──────────────────────────────────────────────────────────────

    pub async fn get_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, category, level, order_index
             FROM skills ORDER BY category ASC, order_index ASC"
        )
        .fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| Skill {
            id:          r.get("id"),
            name:        r.get("name"),
            category:    r.get("category"),
            level:       r.get::<i64, _>("level") as i32,
            order_index: r.get::<i64, _>("order_index") as i32,
        }).collect())
    }

    pub async fn upsert_skill(&self, s: &Skill) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO skills (id, name, category, level, order_index)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               name=excluded.name, category=excluded.category,
               level=excluded.level, order_index=excluded.order_index"
        )
        .bind(&s.id).bind(&s.name).bind(&s.category).bind(s.level).bind(s.order_index)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_skill(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM skills WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    // ─── Certifications ──────────────────────────────────────────────────────

    pub async fn get_certifications(&self) -> Result<Vec<Certification>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, issuer, date, expiry_date, credential_url, order_index
             FROM certifications ORDER BY order_index ASC"
        )
        .fetch_all(&self.pool).await?;

        Ok(rows.iter().map(|r| Certification {
            id:             r.get("id"),
            name:           r.get("name"),
            issuer:         r.get("issuer"),
            date:           r.get("date"),
            expiry_date:    r.get("expiry_date"),
            credential_url: r.get("credential_url"),
            order_index:    r.get::<i64, _>("order_index") as i32,
        }).collect())
    }

    pub async fn upsert_certification(&self, c: &Certification) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO certifications (id, name, issuer, date, expiry_date, credential_url, order_index)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               name=excluded.name, issuer=excluded.issuer, date=excluded.date,
               expiry_date=excluded.expiry_date, credential_url=excluded.credential_url,
               order_index=excluded.order_index"
        )
        .bind(&c.id).bind(&c.name).bind(&c.issuer).bind(&c.date)
        .bind(&c.expiry_date).bind(&c.credential_url).bind(c.order_index)
        .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete_certification(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM certifications WHERE id = ?").bind(id).execute(&self.pool).await?;
        Ok(())
    }

    // ─── Auth ────────────────────────────────────────────────────────────────

    pub async fn verify_admin(&self, username: &str, password: &str) -> Result<bool, sqlx::Error> {
        let row = sqlx::query("SELECT password_hash FROM admin WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let hash: String = row.get("password_hash");
            Ok(bcrypt::verify(password, &hash).unwrap_or(false))
        } else {
            Ok(false)
        }
    }

    pub async fn change_password(&self, username: &str, new_password: &str) -> Result<(), sqlx::Error> {
        let hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST).unwrap();
        sqlx::query("UPDATE admin SET password_hash = ? WHERE username = ?")
            .bind(&hash).bind(username)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn save_theme(&self, theme: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE profile SET theme = ? WHERE id = 'profile'")
            .bind(theme)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}