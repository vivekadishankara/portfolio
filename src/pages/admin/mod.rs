mod shared;
mod profile;
mod experience;
mod education;
mod projects;
mod skills;
mod certifications;
mod settings;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::api::get_portfolio_data;
use crate::models::PortfolioData;
use shared::{get_token, clear_auth};

pub use profile::ProfileEditor;
pub use experience::ExperienceEditor;
pub use education::EducationEditor;
pub use projects::ProjectEditor;
pub use skills::SkillEditor;
pub use certifications::CertificationEditor;
pub use settings::{SettingsPanel, ThemePicker};

// ── Tab definition ────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum Tab {
    Profile, Experience, Education, Projects, Skills, Certifications, Settings, Theme,
}

impl Tab {
    fn all() -> &'static [Tab] {
        use Tab::*;
        &[Profile, Experience, Education, Projects, Skills, Certifications, Settings, Theme]
    }
    fn label(&self) -> &'static str {
        match self {
            Tab::Profile        => "Profile",
            Tab::Experience     => "Experience",
            Tab::Education      => "Education",
            Tab::Projects       => "Projects",
            Tab::Skills         => "Skills",
            Tab::Certifications => "Certifications",
            Tab::Settings       => "Settings",
            Tab::Theme          => "Theme",
        }
    }
}

// ── AdminPage ─────────────────────────────────────────────────────────────────

#[component]
pub fn AdminPage() -> impl IntoView {
    let navigate      = use_navigate();
    let nav_for_effect = navigate.clone();
    let token          = RwSignal::new(get_token());

    // Redirect to login if not authenticated
    Effect::new(move |_| {
        if token.get().is_empty() {
            nav_for_effect("/admin/login", Default::default());
        }
    });

    let data = Resource::new(
        move || token.get(),
        |t| async move {
            if t.is_empty() { return Err(ServerFnError::new("Not authenticated")); }
            get_portfolio_data().await
        },
    );

    let active_tab = RwSignal::new(Tab::Profile);
    let toast      = RwSignal::new(Option::<(bool, String)>::None);

    let show_toast = move |ok: bool, msg: String| {
        toast.set(Some((ok, msg)));
        set_timeout(move || toast.set(None), std::time::Duration::from_secs(3));
    };

    let logout = move |_| {
        clear_auth();
        navigate("/admin/login", Default::default());
    };

    view! {
        <div class="min-h-screen t-bg-primary t-text-primary font-syne">

            // Toast
            {move || toast.get().map(|(ok, msg)| view! {
                <div class=move || format!(
                    "fixed top-6 right-6 z-50 px-5 py-3 font-mono text-xs tracking-wider border {}",
                    if ok { "bg-emerald-500/10 border-emerald-500/30 text-emerald-400" }
                    else  { "bg-red-500/10 border-red-500/30 text-red-400" }
                )>{msg}</div>
            })}

            // Header
            <header class="border-b t-border t-bg-secondary px-6 py-4 flex items-center justify-between sticky top-0 z-40 backdrop-blur">
                <div class="flex items-center gap-6">
                    <a href="/" class="font-mono text-xs t-text-muted hover:t-text-secondary transition-colors tracking-widest uppercase">
                        "← Portfolio"
                    </a>
                    <span class="t-text-muted">"|"</span>
                    <span class="font-syne font-bold t-text-primary text-sm">"Admin Dashboard"</span>
                </div>
                <button on:click=logout
                    class="font-mono text-xs t-text-muted hover:text-red-400 transition-colors tracking-widest uppercase">
                    "Sign Out"
                </button>
            </header>

            <div class="flex h-[calc(100vh-57px)]">
                // Sidebar
                <aside class="w-56 border-r t-border t-bg-secondary flex-shrink-0 overflow-y-auto">
                    <nav class="p-4 space-y-1">
                        {Tab::all().iter().map(|tab| {
                            let t  = tab.clone();
                            let t2 = tab.clone();
                            view! {
                                <button
                                    on:click=move |_| active_tab.set(t.clone())
                                    class=move || format!(
                                        "w-full text-left px-4 py-3 font-mono text-xs tracking-widest uppercase transition-colors border-l-2 {}",
                                        if active_tab.get() == t2 {
                                            "t-accent t-accent-dim border-current"
                                        } else {
                                            "t-text-muted hover:t-text-secondary hover:t-bg-secondary border-transparent"
                                        }
                                    )>
                                    {tab.label()}
                                </button>
                            }
                        }).collect_view()}
                    </nav>
                </aside>

                // Main content
                <main class="flex-1 overflow-y-auto p-8">
                    <Suspense fallback=move || view! {
                        <p class="t-text-muted font-mono text-sm">"Loading..."</p>
                    }>
                        {move || data.get().map(|result| match result {
                            Ok(portfolio) => {
                                let d = RwSignal::new(portfolio);
                                let st = Callback::new(move |(ok, msg)| show_toast(ok, msg));
                                let rf = Callback::new(move |_: ()| data.refetch());
                                view! {
                                    <TabContent
                                        tab=active_tab
                                        data=d
                                        token=token
                                        show_toast=st
                                        on_refresh=rf
                                    />
                                }.into_any()
                            }
                            Err(e) => view! {
                                <p class="text-red-400 font-mono text-sm">{e.to_string()}</p>
                            }.into_any(),
                        })}
                    </Suspense>
                </main>
            </div>
        </div>
    }
}

// ── Tab content router ────────────────────────────────────────────────────────

#[component]
fn TabContent(
    tab: RwSignal<Tab>,
    data: RwSignal<PortfolioData>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    move || match tab.get() {
        Tab::Profile => view! {
            <ProfileEditor profile=data.get().profile token show_toast on_refresh/>
        }.into_any(),
        Tab::Experience => view! {
            <ExperienceEditor items=data.get().experiences token show_toast on_refresh/>
        }.into_any(),
        Tab::Education => view! {
            <EducationEditor items=data.get().educations token show_toast on_refresh/>
        }.into_any(),
        Tab::Projects => view! {
            <ProjectEditor items=data.get().projects token show_toast on_refresh/>
        }.into_any(),
        Tab::Skills => view! {
            <SkillEditor items=data.get().skills token show_toast on_refresh/>
        }.into_any(),
        Tab::Certifications => view! {
            <CertificationEditor items=data.get().certifications token show_toast on_refresh/>
        }.into_any(),
        Tab::Settings => view! {
            <SettingsPanel token show_toast/>
        }.into_any(),
        Tab::Theme => view! {
            <ThemePicker token show_toast current_theme=data.get().profile.theme/>
        }.into_any(),
    }
}
