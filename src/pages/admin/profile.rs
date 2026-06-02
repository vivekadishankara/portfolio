use leptos::prelude::*;
use crate::models::Profile;
use crate::api::update_profile;
use super::shared::*;

#[component]
pub fn ProfileEditor(
    profile: Profile,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let name       = RwSignal::new(profile.name.clone());
    let title      = RwSignal::new(profile.title.clone());
    let bio        = RwSignal::new(profile.bio.clone());
    let summary    = RwSignal::new(profile.summary.clone());
    let email      = RwSignal::new(profile.email.clone());
    let github     = RwSignal::new(profile.github.clone());
    let linkedin   = RwSignal::new(profile.linkedin.clone());
    let twitter    = RwSignal::new(profile.twitter.clone());
    let location   = RwSignal::new(profile.location.clone());
    let avatar_url = RwSignal::new(profile.avatar_url.clone());
    let resume_url = RwSignal::new(profile.resume_url.clone());
    let theme      = RwSignal::new(profile.theme.clone());
    let sec_order  = RwSignal::new(profile.section_order.clone());
    let saving     = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let p = Profile {
            id: "profile".to_string(),
            name: name.get(), title: title.get(), bio: bio.get(),
            summary: summary.get(), email: email.get(),
            github: github.get(), linkedin: linkedin.get(), twitter: twitter.get(),
            location: location.get(), avatar_url: avatar_url.get(),
            resume_url: resume_url.get(), theme: theme.get(),
            section_order: sec_order.get(),
        };
        let t = token.get();
        async move { update_profile(t, p).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_)  => { show_toast.run((true, "Profile saved!".into())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Edit Profile">
            <div class="space-y-6 max-w-2xl">
                <div class="grid grid-cols-2 gap-4">
                    <FormField label="Full Name">
                        <input class={input_cls()} type="text" prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Job Title">
                        <input class={input_cls()} type="text" prop:value=move || title.get() on:input=move |e| title.set(event_target_value(&e))/>
                    </FormField>
                </div>
                <FormField label="Bio (short — shown in hero)">
                    <textarea class={textarea_cls()} rows="3" prop:value=move || bio.get() on:input=move |e| bio.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Summary (longer — shown in about section)">
                    <textarea class={textarea_cls()} rows="6" prop:value=move || summary.get() on:input=move |e| summary.set(event_target_value(&e))/>
                </FormField>
                <div class="grid grid-cols-2 gap-4">
                    <FormField label="Email">
                        <input class={input_cls()} type="email" prop:value=move || email.get() on:input=move |e| email.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Location">
                        <input class={input_cls()} type="text" prop:value=move || location.get() on:input=move |e| location.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="GitHub URL">
                        <input class={input_cls()} type="url" prop:value=move || github.get() on:input=move |e| github.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="LinkedIn URL">
                        <input class={input_cls()} type="url" prop:value=move || linkedin.get() on:input=move |e| linkedin.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Twitter URL">
                        <input class={input_cls()} type="url" prop:value=move || twitter.get() on:input=move |e| twitter.set(event_target_value(&e))/>
                    </FormField>
                    <FormField label="Avatar URL">
                        <input class={input_cls()} type="url" prop:value=move || avatar_url.get() on:input=move |e| avatar_url.set(event_target_value(&e))/>
                    </FormField>
                </div>
                <FormField label="Resume URL">
                    <input class={input_cls()} type="url" prop:value=move || resume_url.get() on:input=move |e| resume_url.set(event_target_value(&e))/>
                </FormField>

                <SectionOrderManager section_order=sec_order/>

                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE PROFILE" }}
                </button>
            </div>
        </AdminSection>
    }
}

// ── Section order drag-and-drop (↑/↓ buttons) ────────────────────────────────

fn section_display_name(key: &str) -> &'static str {
    match key {
        "experience"     => "Work History (Experience)",
        "projects"       => "Selected Work (Projects)",
        "skills"         => "Technical Stack (Skills)",
        "education"      => "Academic Background (Education)",
        "certifications" => "Credentials (Certifications)",
        _                => "Unknown Section",
    }
}

#[component]
fn SectionOrderManager(section_order: RwSignal<String>) -> impl IntoView {
    // Parse the comma-separated string into a reactive Vec
    let sections = RwSignal::new(
        section_order.get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>(),
    );

    // Keep the string signal in sync whenever sections changes
    Effect::new(move |_| {
        section_order.set(sections.get().join(","));
    });

    let move_up = move |idx: usize| {
        if idx > 0 {
            sections.update(|list| list.swap(idx, idx - 1));
        }
    };

    let move_down = move |idx: usize| {
        if idx < sections.get().len() - 1 {
            sections.update(|list| list.swap(idx, idx + 1));
        }
    };

    view! {
        <FormField label="Section Layout Order">
            <div class="space-y-2 mt-2 t-bg-card border t-border p-4 max-w-xl">
                {move || {
                    let list = sections.get();
                    let len  = list.len();
                    list.into_iter().enumerate().map(|(idx, key)| {
                        let is_first = idx == 0;
                        let is_last  = idx == len - 1;
                        let label    = section_display_name(&key);
                        view! {
                            <div class="flex items-center justify-between p-2.5 t-bg-primary border t-border hover:t-border-hover transition-colors">
                                <span class="font-mono text-xs t-text-secondary uppercase tracking-widest">
                                    {label}
                                </span>
                                <div class="flex gap-2">
                                    <button
                                        type="button"
                                        disabled=is_first
                                        on:click=move |_| move_up(idx)
                                        class="px-2.5 py-1 t-bg-secondary hover:t-bg-card disabled:opacity-30 disabled:cursor-not-allowed font-mono text-xs t-text-secondary border t-border transition-colors">
                                        "↑"
                                    </button>
                                    <button
                                        type="button"
                                        disabled=is_last
                                        on:click=move |_| move_down(idx)
                                        class="px-2.5 py-1 t-bg-secondary hover:t-bg-card disabled:opacity-30 disabled:cursor-not-allowed font-mono text-xs t-text-secondary border t-border transition-colors">
                                        "↓"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </FormField>
    }
}