use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::models::*;
use crate::api::*;

fn get_token() -> String {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(storage)) = win.local_storage() {
                return storage.get_item("admin_token").ok().flatten().unwrap_or_default();
            }
        }
    }
    String::new()
}

fn get_username() -> String {
    #[cfg(feature = "hydrate")]
    {
        use web_sys::window;
        if let Some(win) = window() {
            if let Ok(Some(storage)) = win.local_storage() {
                return storage.get_item("admin_username").ok().flatten().unwrap_or("admin".to_string());
            }
        }
    }
    "admin".to_string()
}

#[derive(Clone, PartialEq)]
enum AdminTab {
    Profile,
    Experience,
    Education,
    Projects,
    Skills,
    Certifications,
    Settings,
    Theme,
}

impl std::fmt::Display for AdminTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AdminTab::Profile => write!(f, "Profile"),
            AdminTab::Experience => write!(f, "Experience"),
            AdminTab::Education => write!(f, "Education"),
            AdminTab::Projects => write!(f, "Projects"),
            AdminTab::Skills => write!(f, "Skills"),
            AdminTab::Certifications => write!(f, "Certifications"),
            AdminTab::Settings => write!(f, "Settings"),
            AdminTab::Theme => write!(f, "Theme"),
        }
    }
}

#[component]
pub fn AdminPage() -> impl IntoView {
    let navigate = use_navigate();
    let value = navigate.clone();
    let token = RwSignal::new(get_token());

    Effect::new(move |_| {
        let t = token.get();
        if t.is_empty() {
            value("/admin/login", Default::default());
        }
    });

    let data_resource = Resource::new(move || token.get(), |t| {
        async move {
            if t.is_empty() { return Err(ServerFnError::new("Not authenticated")); }
            get_portfolio_data().await
        }
    });

    let active_tab = RwSignal::new(AdminTab::Profile);
    let toast = RwSignal::new(Option::<(bool, String)>::None);

    let show_toast = move |success: bool, msg: String| {
        toast.set(Some((success, msg)));
        set_timeout(move || toast.set(None), std::time::Duration::from_secs(3));
    };

    let logout = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use web_sys::window;
            if let Some(win) = window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.remove_item("admin_token");
                    let _ = storage.remove_item("admin_username");
                }
            }
        }
        navigate("/admin/login", Default::default());
    };

    view! {
        <div class="min-h-screen bg-zinc-950 text-zinc-100 font-syne">
            {move || toast.get().map(|(ok, msg)| view! {
                <div class={format!("fixed top-6 right-6 z-50 px-5 py-3 font-mono text-xs tracking-wider border {}",
                    if ok { "bg-emerald-500/10 border-emerald-500/30 text-emerald-400" }
                    else { "bg-red-500/10 border-red-500/30 text-red-400" })}>
                    {msg}
                </div>
            })}

            <header class="border-b border-zinc-800 bg-zinc-900/50 px-6 py-4 flex items-center justify-between sticky top-0 z-40 backdrop-blur">
                <div class="flex items-center gap-6">
                    <a href="/" class="font-mono text-xs text-zinc-600 hover:text-zinc-400 transition-colors tracking-widest uppercase">
                        "← Portfolio"
                    </a>
                    <span class="text-zinc-700">"|"</span>
                    <span class="font-syne font-bold text-white text-sm">"Admin Dashboard"</span>
                </div>
                <button on:click=logout
                    class="font-mono text-xs text-zinc-600 hover:text-red-400 transition-colors tracking-widest uppercase">
                    "Sign Out"
                </button>
            </header>

            <div class="flex h-[calc(100vh-57px)]">
                <aside class="w-56 border-r border-zinc-800 bg-zinc-900/30 flex-shrink-0 overflow-y-auto">
                    <nav class="p-4 space-y-1">
                        {[
                            AdminTab::Profile,
                            AdminTab::Experience,
                            AdminTab::Education,
                            AdminTab::Projects,
                            AdminTab::Skills,
                            AdminTab::Certifications,
                            AdminTab::Settings,
                            AdminTab::Theme,
                        ].into_iter().map(|tab| {
                            let tab_clone = tab.clone();
                            let label = tab.to_string();
                            view! {
                                <button
                                    on:click=move |_| active_tab.set(tab_clone.clone())
                                    class=move || format!("w-full text-left px-4 py-3 font-mono text-xs tracking-widest uppercase transition-colors {}",
                                        if active_tab.get() == tab.clone() {
                                            "text-emerald-400 bg-emerald-500/10 border-l-2 border-emerald-400"
                                        } else {
                                            "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/50 border-l-2 border-transparent"
                                        })>
                                    {label}
                                </button>
                            }
                        }).collect_view()}
                    </nav>
                </aside>

                <main class="flex-1 overflow-y-auto p-8">
                    <Suspense fallback=move || view! { <div class="text-zinc-500 font-mono text-sm">"Loading..."</div> }>
                        {move || data_resource.get().map(|result| {
                            match result {
                                Ok(data) => {
                                    let data_sig = RwSignal::new(data);
                                    view! {
                                        <AdminContent
                                            active_tab=active_tab
                                            data=data_sig
                                            token=token
                                            show_toast=Callback::new(move |(ok, msg)| show_toast(ok, msg))
                                            on_refresh=Callback::new(move |_: ()| data_resource.refetch())
                                        />
                                    }.into_any()
                                }
                                Err(e) => view! { <p class="text-red-400 font-mono text-sm">{e.to_string()}</p> }.into_any()
                            }
                        })}
                    </Suspense>
                </main>
            </div>
        </div>
    }
}

#[component]
fn AdminContent(
    active_tab: RwSignal<AdminTab>,
    data: RwSignal<PortfolioData>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    view! {
        {move || match active_tab.get() {
            AdminTab::Profile => view! {
                <ProfileEditor profile=data.get().profile token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Experience => view! {
                <ExperienceEditor items=data.get().experiences token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Education => view! {
                <EducationEditor items=data.get().educations token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Projects => view! {
                <ProjectEditor items=data.get().projects token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Skills => view! {
                <SkillEditor items=data.get().skills token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Certifications => view! {
                <CertificationEditor items=data.get().certifications token=token show_toast=show_toast on_refresh=on_refresh/>
            }.into_any(),
            AdminTab::Settings => view! {
                <SettingsPanel token=token show_toast=show_toast/>
            }.into_any(),
            AdminTab::Theme => view! {
                <ThemePicker token=token show_toast=show_toast current_theme=data.get().profile.theme/>
            }.into_any(),
        }}
    }
}

// ─── Shared helpers ───────────────────────────────────────────────────────────

#[component]
fn AdminSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <h2 class="font-syne font-bold text-2xl text-white mb-8">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
fn FormField(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div>
            <label class="font-mono text-xs text-zinc-500 tracking-widest uppercase block mb-2">{label}</label>
            {children()}
        </div>
    }
}

fn input_class() -> &'static str {
    "w-full bg-zinc-900 border border-zinc-700 focus:border-emerald-500 text-white px-4 py-2.5 text-sm font-mono outline-none transition-colors"
}
fn textarea_class() -> &'static str {
    "w-full bg-zinc-900 border border-zinc-700 focus:border-emerald-500 text-white px-4 py-2.5 text-sm font-mono outline-none transition-colors resize-none"
}
fn btn_primary() -> &'static str {
    "px-6 py-2.5 bg-emerald-500 hover:bg-emerald-400 disabled:bg-zinc-700 text-zinc-950 font-syne font-bold text-xs tracking-widest uppercase transition-all duration-200 disabled:cursor-not-allowed"
}
fn btn_danger() -> &'static str {
    "px-4 py-2 border border-red-500/30 hover:border-red-500 text-red-500 hover:text-red-400 font-mono text-xs tracking-widest uppercase transition-colors"
}
fn btn_secondary() -> &'static str {
    "px-6 py-2.5 border border-zinc-600 hover:border-zinc-400 text-zinc-300 hover:text-white font-syne font-bold text-xs tracking-widest uppercase transition-colors"
}

// In Leptos 0.8, Callback uses run() not call()
fn run_cb<T: 'static>(cb: &Callback<T>, val: T) {
    cb.run(val);
}

// ─── Profile Editor ───────────────────────────────────────────────────────────

#[component]
fn ProfileEditor(
    profile: Profile,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let name = RwSignal::new(profile.name.clone());
    let title = RwSignal::new(profile.title.clone());
    let bio = RwSignal::new(profile.bio.clone());
    let summary = RwSignal::new(profile.summary.clone());
    let email = RwSignal::new(profile.email.clone());
    let github = RwSignal::new(profile.github.clone());
    let linkedin = RwSignal::new(profile.linkedin.clone());
    let twitter = RwSignal::new(profile.twitter.clone());
    let location = RwSignal::new(profile.location.clone());
    let avatar_url = RwSignal::new(profile.avatar_url.clone());
    let resume_url = RwSignal::new(profile.resume_url.clone());
    let profile_theme = RwSignal::new(profile.theme.clone());
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let p = Profile {
            id: "profile".to_string(),
            name: name.get(), title: title.get(), bio: bio.get(),
            summary: summary.get(),
            email: email.get(), github: github.get(), linkedin: linkedin.get(),
            twitter: twitter.get(), location: location.get(),
            avatar_url: avatar_url.get(), resume_url: resume_url.get(),
            theme: profile_theme.get(),
        };
        let t = token.get();
        async move { update_profile(t, p).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Profile saved!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Edit Profile">
            <div class="space-y-6 max-w-2xl">
                <div class="grid grid-cols-2 gap-4">
                    <FormField label="Full Name">
                        <input class={input_class()} type="text" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Job Title">
                        <input class={input_class()} type="text" prop:value=move || title.get() on:input=move |e| title.set(event_target_value(&e))/>
                    </FormField>
                </div>
                <FormField label="Bio (short, shown in hero)">
                    <textarea class={textarea_class()} rows="3" prop:value=move || bio.get() on:input=move |e| bio.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Summary (longer, shown in about section)">
                    <textarea class={textarea_class()} rows="6" prop:value=move || summary.get() on:input=move |e| summary.set(event_target_value(&e))/>
                </FormField>
                <div class="grid grid-cols-2 gap-4">
                    <FormField label="Email">
                        <input class={input_class()} type="email" prop:value=move || email.get() on:input=move |e| email.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Location">
                        <input class={input_class()} type="text" prop:value=move || location.get() on:input=move |e| location.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="GitHub URL">
                        <input class={input_class()} type="url" prop:value=move || github.get() on:input=move |e| github.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="LinkedIn URL">
                        <input class={input_class()} type="url" prop:value=move || linkedin.get() on:input=move |e| linkedin.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Twitter URL">
                        <input class={input_class()} type="url" prop:value=move || twitter.get() on:input=move |e| twitter.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Avatar URL">
                        <input class={input_class()} type="url" prop:value=move || avatar_url.get() on:input=move |e| avatar_url.set(event_target_value(&e))/>
                    </FormField>
                </div>
                <FormField label="Resume URL">
                    <input class={input_class()} type="url" prop:value=move || resume_url.get() on:input=move |e| resume_url.set(event_target_value(&e))/>
                </FormField>
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE PROFILE" }}
                </button>
            </div>
        </AdminSection>
    }
}

// ─── Experience Editor ────────────────────────────────────────────────────────

#[component]
fn ExperienceEditor(
    items: Vec<Experience>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing = RwSignal::new(Option::<Experience>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let id = id.clone();
        let t = token.get();
        async move { delete_experience(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_) => { show_toast.run((true, "Deleted!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Work Experience">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Experience::default()))>"+ ADD EXPERIENCE"</button>
            </div>
            {move || editing.get().map(|exp| view! {
                <ExperienceForm exp=exp token=token show_toast=show_toast on_refresh=on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|exp| {
                    let exp_clone = exp.clone();
                    view! {
                        <div class="border border-zinc-800 p-5 flex items-start justify-between gap-4 hover:border-zinc-600 transition-colors">
                            <div class="flex-1">
                                <h3 class="font-syne font-semibold text-white">{exp.role.clone()}</h3>
                                <p class="text-emerald-400 font-mono text-xs mt-1">{exp.company.clone()}</p>
                                <p class="text-zinc-500 font-mono text-xs mt-1">
                                    {exp.start_date.clone()} " – " {if exp.current { "Present".to_string() } else { exp.end_date.clone().unwrap_or_default() }}
                                </p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(exp_clone.clone()))>"EDIT"</button>
                                <button class={btn_danger()} on:click=move |_| { delete_action.dispatch(exp.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn ExperienceForm(
    exp: Experience,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id = exp.id.clone();
    let company = RwSignal::new(exp.company);
    let role = RwSignal::new(exp.role);
    let start_date = RwSignal::new(exp.start_date);
    let end_date = RwSignal::new(exp.end_date.unwrap_or_default());
    let current = RwSignal::new(exp.current);
    let description = RwSignal::new(exp.description.join("\n"));
    let technologies = RwSignal::new(exp.technologies.join(", "));
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let e = Experience {
            id: id.clone(), company: company.get(), role: role.get(),
            start_date: start_date.get(),
            end_date: if current.get() || end_date.get().is_empty() { None } else { Some(end_date.get()) },
            current: current.get(),
            description: description.get().lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            technologies: technologies.get().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_experience(t, e).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Saved!".to_string())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-8 border border-emerald-500/30 bg-emerald-500/5 p-6 space-y-5">
            <h3 class="font-syne font-bold text-lg text-emerald-400 mb-2">"Experience Entry"</h3>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Company">
                    <input class={input_class()} type="text" prop:value=move || company.get() on:input=move |e| company.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Role / Title">
                    <input class={input_class()} type="text" prop:value=move || role.get() on:input=move |e| role.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Start Date">
                    <input class={input_class()} type="text" placeholder="Jan 2022" prop:value=move || start_date.get() on:input=move |e| start_date.set(event_target_value(&e))/>
                </FormField>
                <FormField label="End Date">
                    <input class={input_class()} type="text" placeholder="Dec 2023" disabled=move || current.get()
                        prop:value=move || end_date.get() on:input=move |e| end_date.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="current-exp" class="accent-emerald-500"
                    prop:checked=move || current.get() on:change=move |e| current.set(event_target_checked(&e))/>
                <label for="current-exp" class="font-mono text-xs text-zinc-400 tracking-widest uppercase">"Currently working here"</label>
            </div>
            <FormField label="Description (one bullet point per line)">
                <textarea class={textarea_class()} rows="6" prop:value=move || description.get() on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Technologies (comma-separated)">
                <input class={input_class()} type="text" placeholder="Rust, TypeScript, PostgreSQL"
                    prop:value=move || technologies.get() on:input=move |e| technologies.set(event_target_value(&e))/>
            </FormField>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ─── Education Editor ─────────────────────────────────────────────────────────

#[component]
fn EducationEditor(
    items: Vec<Education>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing = RwSignal::new(Option::<Education>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let id = id.clone(); let t = token.get();
        async move { delete_education(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_) => { show_toast.run((true, "Deleted!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Education">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Education::default()))>"+ ADD EDUCATION"</button>
            </div>
            {move || editing.get().map(|edu| view! {
                <EducationForm edu=edu token=token show_toast=show_toast on_refresh=on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|edu| {
                    let edu_edit = edu.clone();
                    view! {
                        <div class="border border-zinc-800 p-5 flex items-start justify-between gap-4 hover:border-zinc-600 transition-colors">
                            <div>
                                <h3 class="font-syne font-semibold text-white">{edu.degree.clone()} " in " {edu.field.clone()}</h3>
                                <p class="text-emerald-400 font-mono text-xs mt-1">{edu.institution.clone()}</p>
                                <p class="text-zinc-500 font-mono text-xs mt-1">
                                    {edu.start_year.clone()} " – " {if edu.current { "Present".to_string() } else { edu.end_year.clone().unwrap_or_default() }}
                                </p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(edu_edit.clone()))>"EDIT"</button>
                                <button class={btn_danger()} on:click=move |_| { delete_action.dispatch(edu.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn EducationForm(
    edu: Education,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id = edu.id.clone();
    let institution = RwSignal::new(edu.institution);
    let degree = RwSignal::new(edu.degree);
    let field = RwSignal::new(edu.field);
    let start_year = RwSignal::new(edu.start_year);
    let end_year = RwSignal::new(edu.end_year.unwrap_or_default());
    let current = RwSignal::new(edu.current);
    let description = RwSignal::new(edu.description);
    let gpa = RwSignal::new(edu.gpa.unwrap_or_default());
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let e = Education {
            id: id.clone(), institution: institution.get(), degree: degree.get(),
            field: field.get(), start_year: start_year.get(),
            end_year: if current.get() || end_year.get().is_empty() { None } else { Some(end_year.get()) },
            current: current.get(), description: description.get(),
            gpa: if gpa.get().is_empty() { None } else { Some(gpa.get()) },
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_education(t, e).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Saved!".to_string())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-8 border border-emerald-500/30 bg-emerald-500/5 p-6 space-y-5">
            <h3 class="font-syne font-bold text-lg text-emerald-400">"Education Entry"</h3>
            <FormField label="Institution">
                <input class={input_class()} type="text" prop:value=move || institution.get() on:input=move |e| institution.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Degree">
                    <input class={input_class()} type="text" placeholder="B.Sc." prop:value=move || degree.get() on:input=move |e| degree.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Field of Study">
                    <input class={input_class()} type="text" placeholder="Computer Science" prop:value=move || field.get() on:input=move |e| field.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Start Year">
                    <input class={input_class()} type="text" placeholder="2018" prop:value=move || start_year.get() on:input=move |e| start_year.set(event_target_value(&e))/>
                </FormField>
                <FormField label="End Year">
                    <input class={input_class()} type="text" placeholder="2022" disabled=move || current.get()
                        prop:value=move || end_year.get() on:input=move |e| end_year.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="current-edu" class="accent-emerald-500"
                    prop:checked=move || current.get() on:change=move |e| current.set(event_target_checked(&e))/>
                <label for="current-edu" class="font-mono text-xs text-zinc-400 tracking-widest uppercase">"Currently enrolled"</label>
            </div>
            <FormField label="Description (optional)">
                <textarea class={textarea_class()} rows="3" prop:value=move || description.get() on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="GPA (optional)">
                <input class={input_class()} type="text" placeholder="3.8/4.0" prop:value=move || gpa.get() on:input=move |e| gpa.set(event_target_value(&e))/>
            </FormField>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ─── Project Editor ───────────────────────────────────────────────────────────

#[component]
fn ProjectEditor(
    items: Vec<Project>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing = RwSignal::new(Option::<Project>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let id = id.clone(); let t = token.get();
        async move { delete_project(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_) => { show_toast.run((true, "Deleted!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Projects">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Project::default()))>"+ ADD PROJECT"</button>
            </div>
            {move || editing.get().map(|project| view! {
                <ProjectForm project=project token=token show_toast=show_toast on_refresh=on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|proj| {
                    let proj_edit = proj.clone();
                    view! {
                        <div class="border border-zinc-800 p-5 flex items-start justify-between gap-4 hover:border-zinc-600 transition-colors">
                            <div>
                                <div class="flex items-center gap-3">
                                    <h3 class="font-syne font-semibold text-white">{proj.title.clone()}</h3>
                                    {if proj.featured {
                                        view! { <span class="px-2 py-0.5 bg-emerald-500/10 text-emerald-400 font-mono text-xs border border-emerald-500/20">"FEATURED"</span> }.into_any()
                                    } else { view! { <span></span> }.into_any() }}
                                </div>
                                <p class="text-zinc-500 text-xs mt-1 font-mono">{proj.description.chars().take(80).collect::<String>()}</p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(proj_edit.clone()))>"EDIT"</button>
                                <button class={btn_danger()} on:click=move |_| { delete_action.dispatch(proj.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn ProjectForm(
    project: Project,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id = project.id.clone();
    let title = RwSignal::new(project.title);
    let description = RwSignal::new(project.description);
    let long_desc = RwSignal::new(project.long_description);
    let technologies = RwSignal::new(project.technologies.join(", "));
    let github_url = RwSignal::new(project.github_url.unwrap_or_default());
    let live_url = RwSignal::new(project.live_url.unwrap_or_default());
    let image_url = RwSignal::new(project.image_url.unwrap_or_default());
    let featured = RwSignal::new(project.featured);
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let p = Project {
            id: id.clone(), title: title.get(), description: description.get(),
            long_description: long_desc.get(),
            technologies: technologies.get().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            github_url: if github_url.get().is_empty() { None } else { Some(github_url.get()) },
            live_url: if live_url.get().is_empty() { None } else { Some(live_url.get()) },
            image_url: if image_url.get().is_empty() { None } else { Some(image_url.get()) },
            featured: featured.get(), order_index: 0,
        };
        let t = token.get();
        async move { upsert_project(t, p).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Saved!".to_string())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-8 border border-emerald-500/30 bg-emerald-500/5 p-6 space-y-5">
            <h3 class="font-syne font-bold text-lg text-emerald-400">"Project Entry"</h3>
            <FormField label="Title">
                <input class={input_class()} type="text" prop:value=move || title.get() on:input=move |e| title.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Short Description">
                <input class={input_class()} type="text" prop:value=move || description.get() on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Long Description">
                <textarea class={textarea_class()} rows="5" prop:value=move || long_desc.get() on:input=move |e| long_desc.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Technologies (comma-separated)">
                <input class={input_class()} type="text" placeholder="Rust, Leptos, TailwindCSS"
                    prop:value=move || technologies.get() on:input=move |e| technologies.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="GitHub URL">
                    <input class={input_class()} type="url" prop:value=move || github_url.get() on:input=move |e| github_url.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Live URL">
                    <input class={input_class()} type="url" prop:value=move || live_url.get() on:input=move |e| live_url.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Image URL">
                    <input class={input_class()} type="url" prop:value=move || image_url.get() on:input=move |e| image_url.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="featured" class="accent-emerald-500"
                    prop:checked=move || featured.get() on:change=move |e| featured.set(event_target_checked(&e))/>
                <label for="featured" class="font-mono text-xs text-zinc-400 tracking-widest uppercase">"Featured project"</label>
            </div>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ─── Skills Editor ────────────────────────────────────────────────────────────

#[component]
fn SkillEditor(
    items: Vec<Skill>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing = RwSignal::new(Option::<Skill>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let id = id.clone(); let t = token.get();
        async move { delete_skill(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_) => { show_toast.run((true, "Deleted!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Skills">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Skill::default()))>"+ ADD SKILL"</button>
            </div>
            {move || editing.get().map(|skill| view! {
                <SkillForm skill=skill token=token show_toast=show_toast on_refresh=on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="grid md:grid-cols-2 gap-3">
                {move || items_sig.get().into_iter().map(|skill| {
                    let skill_edit = skill.clone();
                    view! {
                        <div class="border border-zinc-800 p-4 flex items-center justify-between hover:border-zinc-600 transition-colors">
                            <div>
                                <div class="flex items-center gap-3">
                                    <span class="font-medium text-white text-sm">{skill.name.clone()}</span>
                                    <div class="flex gap-1">
                                        {(1..=5).map(|i| {
                                            let lvl = skill.level;
                                            view! {
                                                <div class={if i <= lvl { "w-1.5 h-1.5 bg-emerald-400" } else { "w-1.5 h-1.5 bg-zinc-700" }}></div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                                <span class="font-mono text-xs text-zinc-500">{skill.category.clone()}</span>
                            </div>
                            <div class="flex gap-2">
                                <button class="font-mono text-xs text-zinc-500 hover:text-white transition-colors px-3 py-1 border border-transparent hover:border-zinc-600"
                                    on:click=move |_| editing.set(Some(skill_edit.clone()))>"EDIT"</button>
                                <button class="font-mono text-xs text-red-600 hover:text-red-400 transition-colors px-3 py-1"
                                    on:click=move |_| { delete_action.dispatch(skill.id.clone()); }>"×"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn SkillForm(
    skill: Skill,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id = skill.id.clone();
    let name = RwSignal::new(skill.name);
    let category = RwSignal::new(skill.category);
    let level = RwSignal::new(skill.level);
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let s = Skill { id: id.clone(), name: name.get(), category: category.get(), level: level.get(), order_index: 0 };
        let t = token.get();
        async move { upsert_skill(t, s).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Saved!".to_string())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-6 border border-emerald-500/30 bg-emerald-500/5 p-6 space-y-4">
            <h3 class="font-syne font-bold text-lg text-emerald-400">"Skill Entry"</h3>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Skill Name">
                    <input class={input_class()} type="text" placeholder="Rust"
                        prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Category">
                    <input class={input_class()} type="text" placeholder="Languages, Frameworks, Tools..."
                        prop:value=move || category.get() on:input=move |e| category.set(event_target_value(&e))/>
                </FormField>
            </div>
            <FormField label="Proficiency Level (1–5)">
                <div class="flex items-center gap-4">
                    <input type="range" min="1" max="5" class="accent-emerald-500 flex-1"
                        prop:value=move || level.get().to_string()
                        on:input=move |e| level.set(event_target_value(&e).parse().unwrap_or(3))/>
                    <div class="flex gap-1">
                        {move || (1..=5).map(|i| view! {
                            <div class={if i <= level.get() { "w-3 h-3 bg-emerald-400" } else { "w-3 h-3 bg-zinc-700" }}></div>
                        }).collect_view()}
                    </div>
                </div>
            </FormField>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ─── Certification Editor ─────────────────────────────────────────────────────

#[component]
fn CertificationEditor(
    items: Vec<Certification>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing = RwSignal::new(Option::<Certification>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let id = id.clone(); let t = token.get();
        async move { delete_certification(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_) => { show_toast.run((true, "Deleted!".to_string())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Certifications">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Certification::default()))>"+ ADD CERTIFICATION"</button>
            </div>
            {move || editing.get().map(|cert| view! {
                <CertificationForm cert=cert token=token show_toast=show_toast on_refresh=on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|cert| {
                    let cert_edit = cert.clone();
                    view! {
                        <div class="border border-zinc-800 p-5 flex items-start justify-between gap-4 hover:border-zinc-600 transition-colors">
                            <div>
                                <h3 class="font-syne font-semibold text-white">{cert.name.clone()}</h3>
                                <p class="text-emerald-400 font-mono text-xs mt-1">{cert.issuer.clone()}</p>
                                <p class="text-zinc-500 font-mono text-xs">{cert.date.clone()}</p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(cert_edit.clone()))>"EDIT"</button>
                                <button class={btn_danger()} on:click=move |_| { delete_action.dispatch(cert.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn CertificationForm(
    cert: Certification,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id = cert.id.clone();
    let name = RwSignal::new(cert.name);
    let issuer = RwSignal::new(cert.issuer);
    let date = RwSignal::new(cert.date);
    let expiry = RwSignal::new(cert.expiry_date.unwrap_or_default());
    let cred_url = RwSignal::new(cert.credential_url.unwrap_or_default());
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let c = Certification {
            id: id.clone(), name: name.get(), issuer: issuer.get(), date: date.get(),
            expiry_date: if expiry.get().is_empty() { None } else { Some(expiry.get()) },
            credential_url: if cred_url.get().is_empty() { None } else { Some(cred_url.get()) },
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_certification(t, c).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => { show_toast.run((true, "Saved!".to_string())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-6 border border-emerald-500/30 bg-emerald-500/5 p-6 space-y-4">
            <h3 class="font-syne font-bold text-lg text-emerald-400">"Certification Entry"</h3>
            <FormField label="Certification Name">
                <input class={input_class()} type="text" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Issuing Organization">
                <input class={input_class()} type="text" prop:value=move || issuer.get() on:input=move |e| issuer.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Issue Date">
                    <input class={input_class()} type="text" placeholder="Jan 2024"
                        prop:value=move || date.get() on:input=move |e| date.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Expiry Date (optional)">
                    <input class={input_class()} type="text" placeholder="Jan 2027"
                        prop:value=move || expiry.get() on:input=move |e| expiry.set(event_target_value(&e))/>
                </FormField>
            </div>
            <FormField label="Credential URL (optional)">
                <input class={input_class()} type="url" prop:value=move || cred_url.get() on:input=move |e| cred_url.set(event_target_value(&e))/>
            </FormField>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ─── Settings ─────────────────────────────────────────────────────────────────

#[component]
fn SettingsPanel(
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
) -> impl IntoView {
    let new_password = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let t = token.get();
        let u = get_username();
        let p = new_password.get();
        async move { change_password(t, u, p).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => {
                    show_toast.run((true, "Password changed!".to_string()));
                    new_password.set(String::new());
                    confirm_password.set(String::new());
                }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    let on_submit = move |_| {
        if new_password.get().len() < 6 {
            show_toast.run((false, "Password must be at least 6 characters".to_string()));
            return;
        }
        if new_password.get() != confirm_password.get() {
            show_toast.run((false, "Passwords do not match".to_string()));
            return;
        }
        saving.set(true);
        save.dispatch(());
    };

    view! {
        <AdminSection title="Settings">
            <div class="max-w-md space-y-6">
                <div class="border border-zinc-800 p-6 space-y-5">
                    <h3 class="font-syne font-semibold text-lg text-white">"Change Password"</h3>
                    <FormField label="New Password">
                        <input class={input_class()} type="password" autocomplete="new-password"
                            prop:value=move || new_password.get()
                            on:input=move |e| new_password.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Confirm Password">
                        <input class={input_class()} type="password" autocomplete="new-password"
                            prop:value=move || confirm_password.get()
                            on:input=move |e| confirm_password.set(event_target_value(&e))/>
                    </FormField>
                    <button class={btn_primary()} disabled=move || saving.get() on:click=on_submit>
                        {move || if saving.get() { "CHANGING..." } else { "CHANGE PASSWORD" }}
                    </button>
                </div>
                <div class="border border-zinc-800 p-6">
                    <h3 class="font-syne font-semibold text-lg text-white mb-4">"Quick Tips"</h3>
                    <ul class="space-y-2 text-zinc-500 font-mono text-xs">
                        <li>"• Change the default password immediately after first login"</li>
                        <li>"• JWT tokens expire after 24 hours"</li>
                        <li>"• Skill level: 1 = beginner, 5 = expert"</li>
                        <li>"• Featured projects appear in the large hero layout"</li>
                        <li>"• Technologies are comma-separated (Rust, Leptos, Axum)"</li>
                    </ul>
                </div>
            </div>
        </AdminSection>
    }
}

// ─── Theme Picker ─────────────────────────────────────────────────────────────

#[component]
fn ThemePicker(
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    current_theme: String,
) -> impl IntoView {
    let selected = RwSignal::new(current_theme);
    let saving = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let t = token.get();
        let theme = selected.get();
        async move { crate::api::save_theme(t, theme).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_) => {
                    show_toast.run((true, "Theme saved! Refresh to see changes.".to_string()));
                    // Apply theme immediately via JS
                    #[cfg(feature = "hydrate")]
                    {
                        use web_sys::window;
                        if let Some(win) = window() {
                            if let Some(doc) = win.document() {
                                if let Some(el) = doc.document_element() {
                                    let _ = el.set_attribute("data-theme", &selected.get_untracked());
                                }
                            }
                        }
                    }
                }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    let themes: Vec<(&'static str, &'static str, &'static str, &'static str)> = vec![
        ("dark-emerald", "Dark Emerald",  "#09090b", "#10b981"),
        ("dark-blue",    "Dark Blue",     "#0a0f1e", "#3b82f6"),
        ("dark-purple",  "Dark Purple",   "#0d0a1a", "#a855f7"),
        ("dark-rose",    "Dark Rose",     "#0f0a0a", "#f43f5e"),
        ("light",        "Light Purple",  "#ffffff", "#7c3aed"),
        ("light-blue",   "Light Blue",    "#f0f9ff", "#0284c7"),
    ];

    view! {
        <AdminSection title="Theme">
            <div class="max-w-2xl">
                <p class="font-mono text-xs text-zinc-500 tracking-widest mb-8 uppercase">
                    "Select a colour scheme for your portfolio"
                </p>

                <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-8">
                    {themes.into_iter().map(|(id, label, bg, accent)| {
                        let id_str = id.to_string();
                        let id_str2 = id_str.clone();
                        view! {
                            <button
                                on:click=move |_| selected.set(id_str.clone())
                                class=move || format!(
                                    "relative p-4 border-2 transition-all duration-200 text-left {}",
                                    if selected.get() == id_str2 {
                                        "border-white scale-105 shadow-lg"
                                    } else {
                                        "border-zinc-700 hover:border-zinc-400"
                                    }
                                )
                                style=move || format!("background-color: {}", bg)
                            >
                                // Accent swatch
                                <div class="w-8 h-8 rounded-full mb-3"
                                    style=move || format!("background-color: {}", accent)>
                                </div>
                                <p class="font-syne font-bold text-sm"
                                    style=move || format!("color: {}", if bg == "#ffffff" || bg == "#f0f9ff" { "#0f172a" } else { "#ffffff" })>
                                    {label}
                                </p>
                                <p class="font-mono text-xs mt-1 opacity-60"
                                    style=move || format!("color: {}", if bg == "#ffffff" || bg == "#f0f9ff" { "#0f172a" } else { "#ffffff" })>
                                    {id}
                                </p>
                                // Selected checkmark
                                {move || if selected.get() == id {
                                    view! {
                                        <div class="absolute top-2 right-2 w-5 h-5 rounded-full bg-white flex items-center justify-center">
                                            <span class="text-xs font-bold" style="color:#09090b">"✓"</span>
                                        </div>
                                    }.into_any()
                                } else { view! { <span></span> }.into_any() }}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Preview strip
                <div class="mb-8 p-5 border border-zinc-700 space-y-3"
                    style=move || {
                        let t = selected.get();
                        let (bg, accent) = theme_colors(&t);
                        format!("background-color: {}; border-color: {}40", bg, accent)
                    }>
                    <p class="font-mono text-xs tracking-widest uppercase opacity-50 text-white">"Preview"</p>
                    <div style=move || { let (_, accent) = theme_colors(&selected.get()); format!("color: {}", accent) }
                        class="font-syne font-bold text-2xl">
                        "Your Name"
                    </div>
                    <div class="text-zinc-400 text-sm">"Full Stack Engineer"</div>
                    <div class="flex gap-2 mt-2">
                        <span class="px-2 py-1 text-xs font-mono"
                            style=move || { let (_, accent) = theme_colors(&selected.get()); format!("color: {}; border: 1px solid {}40; background: {}15", accent, accent, accent) }>
                            "Rust"
                        </span>
                        <span class="px-2 py-1 text-xs font-mono"
                            style=move || { let (_, accent) = theme_colors(&selected.get()); format!("color: {}; border: 1px solid {}40; background: {}15", accent, accent, accent) }>
                            "Leptos"
                        </span>
                    </div>
                </div>

                <button
                    class="px-8 py-3 font-syne font-bold text-xs tracking-widest uppercase transition-all duration-200 disabled:opacity-50"
                    style=move || {
                        let (_, accent) = theme_colors(&selected.get());
                        format!("background-color: {}; color: #fff", accent)
                    }
                    disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "APPLY THEME" }}
                </button>
            </div>
        </AdminSection>
    }
}

fn theme_colors(theme: &str) -> (&'static str, &'static str) {
    match theme {
        "dark-blue"   => ("#0a0f1e", "#3b82f6"),
        "dark-purple" => ("#0d0a1a", "#a855f7"),
        "dark-rose"   => ("#0f0a0a", "#f43f5e"),
        "light"       => ("#ffffff", "#7c3aed"),
        "light-blue"  => ("#f0f9ff", "#0284c7"),
        _             => ("#09090b", "#10b981"), // dark-emerald default
    }
}
