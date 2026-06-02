use leptos::prelude::*;
use std::collections::HashMap;
use crate::models::Skill;
use super::shared::{SectionLabel, EmptyState};

#[component]
pub fn SkillsSection(skills: Vec<Skill>) -> impl IntoView {
    let categories = group_by_category(skills);

    view! {
        <section id="skills" class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="04 / Skills"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Technical Stack"</h2>
                {if categories.is_empty() {
                    view! { <EmptyState message="No skills added yet"/> }.into_any()
                } else {
                    view! {
                        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                            {categories.into_iter().map(|(cat, skills)| view! {
                                <SkillCategory category=cat skills/>
                            }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn SkillCategory(category: String, skills: Vec<Skill>) -> impl IntoView {
    view! {
        <div>
            <h3 class="font-mono text-xs t-accent tracking-widest uppercase mb-4">{category}</h3>
            <div class="space-y-3">
                {skills.into_iter().map(|skill| view! { <SkillRow skill/> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn SkillRow(skill: Skill) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between">
            <span class="t-text-secondary text-sm font-medium">{skill.name}</span>
            <div class="flex gap-1">
                {(1..=5).map(|i| view! {
                    <div class={if i <= skill.level { "w-2 h-2 t-accent-bg" } else { "w-2 h-2 bg-zinc-700" }}></div>
                }).collect_view()}
            </div>
        </div>
    }
}

fn group_by_category(skills: Vec<Skill>) -> Vec<(String, Vec<Skill>)> {
    let mut map: HashMap<String, Vec<Skill>> = HashMap::new();
    for skill in skills {
        map.entry(skill.category.clone()).or_default().push(skill);
    }
    let mut result: Vec<(String, Vec<Skill>)> = map.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}
