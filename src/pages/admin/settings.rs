use leptos::prelude::*;
use crate::api::{change_password, save_theme};
use super::shared::*;

// ── Settings ──────────────────────────────────────────────────────────────────

#[component]
pub fn SettingsPanel(
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
) -> impl IntoView {
    let new_password     = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let saving           = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let (t, u, p) = (token.get(), get_username(), new_password.get());
        async move { change_password(t, u, p).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => {
                    show_toast.run((true, "Password changed!".into()));
                    new_password.set(String::new());
                    confirm_password.set(String::new());
                }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    let on_submit = move |_| {
        if new_password.get().len() < 6 {
            show_toast.run((false, "Password must be at least 6 characters".into()));
            return;
        }
        if new_password.get() != confirm_password.get() {
            show_toast.run((false, "Passwords do not match".into()));
            return;
        }
        saving.set(true);
        save.dispatch(());
    };

    view! {
        <AdminSection title="Settings">
            <div class="max-w-md space-y-6">
                <div class="border t-border p-6 space-y-5">
                    <h3 class="font-syne font-semibold text-lg t-text-primary">"Change Password"</h3>
                    <FormField label="New Password">
                        <input class={input_cls()} type="password" autocomplete="new-password"
                            prop:value=move || new_password.get()
                            on:input=move |e| new_password.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Confirm Password">
                        <input class={input_cls()} type="password" autocomplete="new-password"
                            prop:value=move || confirm_password.get()
                            on:input=move |e| confirm_password.set(event_target_value(&e))/>
                    </FormField>
                    <button class={btn_primary()} disabled=move || saving.get() on:click=on_submit>
                        {move || if saving.get() { "CHANGING..." } else { "CHANGE PASSWORD" }}
                    </button>
                </div>
                <div class="border t-border p-6">
                    <h3 class="font-syne font-semibold text-lg t-text-primary mb-4">"Quick Tips"</h3>
                    <ul class="space-y-2 t-text-muted font-mono text-xs">
                        <li>"• Change the default password immediately after first login"</li>
                        <li>"• JWT tokens expire after 24 hours"</li>
                        <li>"• Skill level: 1 = beginner, 5 = expert"</li>
                        <li>"• Featured projects appear in the large hero layout"</li>
                        <li>"• Technologies are comma-separated (Rust, Leptos, Axum)"</li>
                        <li>"• Section order is comma-separated in Profile settings"</li>
                    </ul>
                </div>
            </div>
        </AdminSection>
    }
}

// ── Theme Picker ──────────────────────────────────────────────────────────────

const THEMES: &[(&str, &str, &str, &str)] = &[
    ("dark-emerald", "Dark Emerald",  "#09090b", "#10b981"),
    ("dark-blue",    "Dark Blue",     "#0a0f1e", "#3b82f6"),
    ("dark-purple",  "Dark Purple",   "#0d0a1a", "#a855f7"),
    ("dark-rose",    "Dark Rose",     "#0f0a0a", "#f43f5e"),
    ("light",        "Light Purple",  "#ffffff", "#7c3aed"),
    ("light-blue",   "Light Blue",    "#f0f9ff", "#0284c7"),
];

fn is_light(bg: &str) -> bool {
    matches!(bg, "#ffffff" | "#f0f9ff")
}

fn theme_colors(theme: &str) -> (&'static str, &'static str) {
    THEMES.iter()
        .find(|(id, _, _, _)| *id == theme)
        .map(|(_, _, bg, acc)| (*bg, *acc))
        .unwrap_or(("#09090b", "#10b981"))
}

#[cfg(feature = "hydrate")]
fn apply_theme_to_dom(theme: &str) {
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("data-theme", theme);
                }
            }
        }
    }
}

#[component]
pub fn ThemePicker(
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    current_theme: String,
) -> impl IntoView {
    let selected = RwSignal::new(current_theme);
    let saving   = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let (t, theme) = (token.get(), selected.get());
        async move { save_theme(t, theme).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => {
                    #[cfg(feature = "hydrate")]
                    apply_theme_to_dom(&selected.get_untracked());
                    show_toast.run((true, "Theme applied!".into()));
                }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Theme">
            <div class="max-w-2xl">
                <p class="font-mono text-xs t-text-muted tracking-widest mb-8 uppercase">
                    "Select a colour scheme for your portfolio"
                </p>

                // Theme cards grid
                <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-8">
                    {THEMES.iter().map(|(id, label, bg, accent)| {
                        let id_str  = id.to_string();
                        let id_str2 = id_str.clone();
                        let text_col = if is_light(bg) { "#0f172a" } else { "#ffffff" };
                        view! {
                            <button
                                on:click=move |_| selected.set(id_str.clone())
                                class=move || format!(
                                    "relative p-4 border-2 transition-all duration-200 text-left {}",
                                    if selected.get() == id_str2 { "border-white scale-105" } else { "border-zinc-700 hover:border-zinc-400" }
                                )
                                style=move || format!("background-color:{bg}")>
                                <div class="w-8 h-8 rounded-full mb-3" style=move || format!("background-color:{accent}")></div>
                                <p class="font-syne font-bold text-sm" style=move || format!("color:{text_col}")>{*label}</p>
                                <p class="font-mono text-xs mt-1 opacity-60" style=move || format!("color:{text_col}")>{*id}</p>
                                {move || (selected.get() == *id).then(|| view! {
                                    <div class="absolute top-2 right-2 w-5 h-5 rounded-full bg-white flex items-center justify-center">
                                        <span class="text-xs font-bold text-zinc-950">"✓"</span>
                                    </div>
                                })}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Live preview strip
                <div class="mb-8 p-5 border space-y-3"
                    style=move || {
                        let (bg, acc) = theme_colors(&selected.get());
                        format!("background-color:{bg}; border-color:{acc}40")
                    }>
                    <p class="font-mono text-xs tracking-widest uppercase opacity-40 text-white">"Preview"</p>
                    <div class="font-syne font-bold text-2xl"
                        style=move || { let (_, acc) = theme_colors(&selected.get()); format!("color:{acc}") }>
                        "Your Name"
                    </div>
                    <div class="text-sm opacity-70 text-white">"Full Stack Engineer"</div>
                    <div class="flex gap-2 mt-2">
                        {["Rust", "Leptos"].iter().map(|tag| view! {
                            <span class="px-2 py-1 text-xs font-mono"
                                style=move || {
                                    let (_, acc) = theme_colors(&selected.get());
                                    format!("color:{acc}; border:1px solid {acc}40; background:{acc}15")
                                }>
                                {*tag}
                            </span>
                        }).collect_view()}
                    </div>
                </div>

                <button
                    class="px-8 py-3 font-syne font-bold text-xs tracking-widest uppercase transition-all duration-200 disabled:opacity-50"
                    style=move || { let (_, acc) = theme_colors(&selected.get()); format!("background-color:{acc}; color:#fff") }
                    disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "APPLY THEME" }}
                </button>
            </div>
        </AdminSection>
    }
}
