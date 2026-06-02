use leptos::prelude::*;
use crate::models::Education;
use super::shared::{SectionLabel, EmptyState};

#[component]
pub fn EducationSection(educations: Vec<Education>) -> impl IntoView {
    view! {
        <section id="education" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="05 / Education"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Academic Background"</h2>
                {if educations.is_empty() {
                    view! { <EmptyState message="No education added yet"/> }.into_any()
                } else {
                    view! {
                        <div class="grid md:grid-cols-2 gap-6">
                            {educations.into_iter().map(|edu| view! { <EducationCard edu/> }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn EducationCard(edu: Education) -> impl IntoView {
    let years = format!("{} – {}",
        edu.start_year,
        if edu.current { "Present".to_string() } else { edu.end_year.unwrap_or_default() }
    );

    view! {
        <div class="border t-border hover:t-border-hover transition-colors p-6 t-bg-secondary">
            <div class="flex flex-wrap justify-between items-start gap-4 mb-4">
                <div>
                    <h3 class="font-syne font-bold text-lg t-text-primary">
                        {edu.degree} " in " {edu.field}
                    </h3>
                    <p class="t-accent font-mono text-sm mt-1">{edu.institution}</p>
                </div>
                <span class="font-mono text-xs t-text-muted">{years}</span>
            </div>
            {(!edu.description.is_empty()).then(|| view! {
                <p class="t-text-secondary text-sm leading-relaxed">{edu.description}</p>
            })}
            {edu.gpa.map(|gpa| view! {
                <p class="font-mono text-xs t-text-muted mt-3">"GPA: " {gpa}</p>
            })}
        </div>
    }
}
