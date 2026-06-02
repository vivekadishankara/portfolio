use leptos::prelude::*;
use crate::models::Experience;
use super::shared::{SectionLabel, EmptyState, TechTag};

#[component]
pub fn ExperienceSection(experiences: Vec<Experience>) -> impl IntoView {
    view! {
        <section id="experience" class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="02 / Experience"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Work History"</h2>
                {if experiences.is_empty() {
                    view! { <EmptyState message="No experience added yet"/> }.into_any()
                } else {
                    view! {
                        <div class="relative">
                            <div class="absolute left-0 top-0 bottom-0 w-px t-border"></div>
                            <div class="space-y-12">
                                {experiences.into_iter().map(|exp| view! { <ExperienceCard exp/> }).collect_view()}
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn ExperienceCard(exp: Experience) -> impl IntoView {
    let date_range = format_date_range(&exp);

    view! {
        <div class="relative pl-10">
            <div class="absolute left-0 top-2 w-2 h-2 -translate-x-[4.5px] rounded-full t-accent-bg"></div>
            <div class="t-bg-card border t-border p-6 hover:t-border-hover transition-colors">
                <div class="mb-4">
                    <div class="flex items-baseline gap-2 flex-wrap">
                        <h3 class="font-syne font-bold text-xl t-text-primary shrink-0">{exp.role}</h3>
                        <span class="font-mono text-xs t-text-muted tracking-wider whitespace-nowrap ml-2">
                            {date_range}
                        </span>
                    </div>
                    <p class="t-accent font-mono text-sm tracking-wide mt-1">{exp.company}</p>
                </div>
                <ul class="space-y-1 mb-4">
                    {exp.description.into_iter().map(|point| view! {
                        <li class="flex items-start gap-2">
                            <span class="t-accent shrink-0 leading-relaxed text-sm">"▸"</span>
                            <span class="t-text-secondary text-sm leading-relaxed">{point}</span>
                        </li>
                    }).collect_view()}
                </ul>
                <div class="flex flex-wrap gap-2">
                    {exp.technologies.into_iter().map(|t| view! { <TechTag name=t/> }).collect_view()}
                </div>
            </div>
        </div>
    }
}

fn format_date_range(exp: &Experience) -> String {
    let end = if exp.current {
        " – Present".to_string()
    } else {
        exp.end_date.clone()
            .filter(|s| !s.is_empty())
            .map(|d| format!(" – {d}"))
            .unwrap_or_default()
    };
    format!("{}{}", exp.start_date, end)
}
