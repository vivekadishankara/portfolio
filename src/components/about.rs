use leptos::prelude::*;
use crate::models::Profile;
use super::shared::{SectionLabel, OptionalLink};

#[component]
pub fn AboutSection(profile: Profile) -> impl IntoView {
    let body = if !profile.summary.is_empty() { profile.summary.clone() } else { profile.bio.clone() };

    view! {
        <section id="about" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="01 / About"/>
                <div class="grid lg:grid-cols-2 gap-16 mt-16 items-start">
                    <div>
                        <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mb-8 leading-tight">
                            "A developer who cares about the details"
                        </h2>
                        <p class="t-text-secondary text-lg leading-relaxed mb-6">{body}</p>
                        <div class="flex items-center gap-3 t-text-muted font-mono text-sm mt-6">
                            <span class="t-accent">"📍"</span>
                            <span>{profile.location}</span>
                        </div>
                    </div>
                    <div class="space-y-4">
                        <OptionalLink label="Email" value=profile.email.clone()/>
                        {(!profile.github.is_empty()).then(|| view! {
                            <OptionalLink label="GitHub"
                                value=profile.github.replace("https://", "")
                                href=profile.github.clone()/>
                        })}
                        {(!profile.linkedin.is_empty()).then(|| view! {
                            <OptionalLink label="LinkedIn"
                                value="View Profile".to_string()
                                href=profile.linkedin.clone()/>
                        })}
                    </div>
                </div>
            </div>
        </section>
    }
}
