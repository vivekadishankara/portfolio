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
                <div class="mt-8">
                    // Heading
                    <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mb-6 leading-tight max-w-2xl">
                        "A developer who cares about the details"
                    </h2>

                    // Location
                    <div class="flex items-center gap-3 t-text-muted font-mono text-sm mb-6">
                        <span class="t-accent">"📍"</span>
                        <span>{profile.location}</span>
                    </div>

                    // Bio section with floating cards
                    <div class="clearfix">
                        <div class="
                            float-right
                            sm:mr-8
                            w-full
                            sm:w-96
                            ml-0
                            sm:ml-8
                            mb-4
                            space-y-2
                        ">
                            <OptionalLink
                                label="Email"
                                value=profile.email.clone()
                            />

                            {(!profile.github.is_empty()).then(|| view! {
                                <OptionalLink
                                    label="GitHub"
                                    value=profile.github.replace("https://", "")
                                    href=profile.github.clone()
                                />
                            })}

                            {(!profile.linkedin.is_empty()).then(|| view! {
                                <OptionalLink
                                    label="LinkedIn"
                                    value="View Profile".to_string()
                                    href=profile.linkedin.clone()
                                />
                            })}
                        </div>

                        <p class="t-text-secondary text-lg leading-relaxed whitespace-pre-line">
                            {body}
                        </p>
                    </div>
                </div>            
            </div>
        </section>
    }
}
