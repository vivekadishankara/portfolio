use leptos::prelude::*;
use crate::models::Profile;
use super::shared::SectionLabelCentered;

#[component]
pub fn ContactSection(profile: Profile) -> impl IntoView {
    view! {
        <section id="contact" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto text-center">
                <SectionLabelCentered label="07 / Contact"/>
                <h2 class="font-syne font-bold text-5xl md:text-7xl t-text-primary mt-6 mb-6">
                    "Let's Work Together"
                </h2>
                <p class="t-text-secondary text-lg max-w-xl mx-auto mb-12">
                    "Have a project in mind or want to chat? My inbox is always open."
                </p>
                <a href={format!("mailto:{}", profile.email)}
                    class="inline-block px-12 py-4 t-accent-bg text-zinc-950 font-syne font-bold text-sm tracking-widest uppercase transition-all duration-200 hover:scale-105">
                    "Say Hello ↗"
                </a>
            </div>
        </section>
    }
}
