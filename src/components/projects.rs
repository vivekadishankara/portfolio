use leptos::prelude::*;
use crate::models::Project;
use super::shared::{SectionLabel, EmptyState, TechTag, AccentTag};

#[component]
pub fn ProjectsSection(projects: Vec<Project>) -> impl IntoView {
    let featured: Vec<Project> = projects.iter().filter(|p| p.featured).cloned().collect();
    let rest:     Vec<Project> = projects.iter().filter(|p| !p.featured).cloned().collect();

    view! {
        <section id="projects" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="03 / Projects"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Selected Work"</h2>
                {if projects.is_empty() {
                    view! { <EmptyState message="No projects added yet"/> }.into_any()
                } else {
                    view! {
                        <div>
                            {(!featured.is_empty()).then(|| view! {
                                <div class="space-y-8 mb-16">
                                    {featured.into_iter().map(|p| view! { <FeaturedCard project=p/> }).collect_view()}
                                </div>
                            })}
                            {(!rest.is_empty()).then(|| view! {
                                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                                    {rest.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}
                                </div>
                            })}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn FeaturedCard(project: Project) -> impl IntoView {
    let desc = if project.long_description.is_empty() {
        project.description.clone()
    } else {
        project.long_description.clone()
    };

    view! {
        <div class="border t-border hover:t-accent-border transition-all duration-300 p-8 t-bg-secondary group">
            <div class="flex flex-wrap items-start justify-between gap-4 mb-6">
                <div>
                    <span class="font-mono text-xs t-accent tracking-widest uppercase mb-3 block">"Featured Project"</span>
                    <h3 class="font-syne font-bold text-2xl t-text-primary group-hover:t-accent transition-colors">
                        {project.title}
                    </h3>
                </div>
                <div class="flex gap-4">
                    {project.github_url.map(|url| view! {
                        <a href={url} target="_blank" class="font-mono text-xs t-text-muted hover:t-text-primary transition-colors uppercase tracking-widest">
                            "GitHub ↗"
                        </a>
                    })}
                    {project.live_url.map(|url| view! {
                        <a href={url} target="_blank" class="font-mono text-xs t-text-muted hover:t-accent transition-colors uppercase tracking-widest">
                            "Live ↗"
                        </a>
                    })}
                </div>
            </div>
            <p class="t-text-secondary leading-relaxed mb-6">{desc}</p>
            <div class="flex flex-wrap gap-2">
                {project.technologies.into_iter().map(|t| view! { <AccentTag name=t/> }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="border t-border hover:t-border-hover transition-all duration-300 p-6 t-bg-secondary group flex flex-col h-full">
            <div class="flex items-start justify-between mb-4">
                <h3 class="font-syne font-bold text-lg t-text-primary group-hover:t-accent transition-colors">
                    {project.title}
                </h3>
                <div class="flex gap-3 ml-4">
                    {project.github_url.map(|url| view! {
                        <a href={url} target="_blank" class="t-text-muted hover:t-text-primary transition-colors text-xs font-mono">"GH"</a>
                    })}
                    {project.live_url.map(|url| view! {
                        <a href={url} target="_blank" class="t-text-muted hover:t-accent transition-colors text-xs font-mono">"↗"</a>
                    })}
                </div>
            </div>
            <p class="t-text-muted text-sm leading-relaxed flex-1 mb-4">{project.description}</p>
            <div class="flex flex-wrap gap-1.5">
                {project.technologies.into_iter().take(4).map(|t| view! { <TechTag name=t/> }).collect_view()}
            </div>
        </div>
    }
}
