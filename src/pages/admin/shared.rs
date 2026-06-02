use leptos::prelude::*;

// ── Layout helpers ────────────────────────────────────────────────────────────

#[component]
pub fn AdminSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <h2 class="font-syne font-bold text-2xl t-text-primary mb-8">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
pub fn FormField(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <label class="font-mono text-xs t-text-muted tracking-widest uppercase block mb-2">{label}</label>
            {children()}
        </div>
    }
}

// ── CSS class helpers ─────────────────────────────────────────────────────────

pub fn input_cls()     -> &'static str { "w-full t-bg-secondary border t-border focus:t-accent t-text-primary px-4 py-2.5 text-sm font-mono outline-none transition-colors" }
pub fn textarea_cls()  -> &'static str { "w-full t-bg-secondary border t-border focus:t-accent t-text-primary px-4 py-2.5 text-sm font-mono outline-none transition-colors resize-none" }
pub fn btn_primary()   -> &'static str { "px-6 py-2.5 t-accent-bg t-accent-fg font-syne font-bold text-xs tracking-widest uppercase transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed" }
pub fn btn_secondary() -> &'static str { "px-6 py-2.5 border t-border hover:t-border-hover t-text-secondary font-syne font-bold text-xs tracking-widest uppercase transition-colors" }
pub fn btn_danger()    -> &'static str { "px-4 py-2 border border-red-500/30 hover:border-red-500 text-red-500 font-mono text-xs tracking-widest uppercase transition-colors" }

// ── Auth storage ──────────────────────────────────────────────────────────────

pub fn get_token() -> String {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(s)) = win.local_storage() {
                return s.get_item("admin_token").ok().flatten().unwrap_or_default();
            }
        }
    }
    String::new()
}

pub fn get_username() -> String {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(s)) = win.local_storage() {
                return s.get_item("admin_username").ok().flatten().unwrap_or("admin".to_string());
            }
        }
    }
    "admin".to_string()
}

pub fn clear_auth() {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(s)) = win.local_storage() {
                let _ = s.remove_item("admin_token");
                let _ = s.remove_item("admin_username");
            }
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn store_auth(token: &str, username: &str) {
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(s)) = win.local_storage() {
                let _ = s.set_item("admin_token", token);
                let _ = s.set_item("admin_username", username);
            }
        }
    }
}
