use leptos::prelude::*;
use leptos_meta::Title;
use crate::api::get_portfolio_data;
use crate::models::PortfolioData;
use crate::components::{
    nav::{NavBar, Footer},
    hero::HeroSection,
    about::AboutSection,
    experience::ExperienceSection,
    projects::ProjectsSection,
    skills::SkillsSection,
    education::EducationSection,
    certifications::CertificationsSection,
    contact::ContactSection,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let data = Resource::new(|| (), |_| get_portfolio_data());
    view! {
        <div class="min-h-screen t-bg-primary t-text-primary font-syne">
            <Suspense fallback=move || view! { <LoadingScreen/> }>
                {move || data.get().map(|result| match result {
                    Ok(p)  => view! { <PortfolioContent portfolio=p/> }.into_any(),
                    Err(e) => view! { <ErrorScreen message=e.to_string()/> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn PortfolioContent(portfolio: PortfolioData) -> impl IntoView {
    let sections: Vec<String> = portfolio.profile.section_order
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let PortfolioData { profile, experiences, educations, projects, skills, certifications } = portfolio;

    view! {
        <Title text=profile.name.clone()/>
        <NavBar name=profile.name.clone()/>
        <HeroSection profile=profile.clone()/>
        <AboutSection profile=profile.clone()/>
        {sections.into_iter().map(|section| match section.as_str() {
            "experience"    => view! { <ExperienceSection    experiences=experiences.clone()/>    }.into_any(),
            "projects"      => view! { <ProjectsSection      projects=projects.clone()/>          }.into_any(),
            "skills"        => view! { <SkillsSection        skills=skills.clone()/>              }.into_any(),
            "education"     => view! { <EducationSection     educations=educations.clone()/>      }.into_any(),
            "certifications"=> view! { <CertificationsSection certifications=certifications.clone()/> }.into_any(),
            _               => view! { <></> }.into_any(),
        }).collect_view()}
        <ContactSection profile=profile/>
        <Footer/>
    }
}

#[component]
fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="min-h-screen t-bg-primary flex items-center justify-center">
            <div class="text-center">
                <div class="w-12 h-12 border-2 t-accent-border border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
                <p class="t-text-secondary font-mono text-sm tracking-widest">"LOADING..."</p>
            </div>
        </div>
    }
}

#[component]
fn ErrorScreen(message: String) -> impl IntoView {
    view! {
        <div class="min-h-screen t-bg-primary flex items-center justify-center">
            <div class="text-center max-w-md">
                <p class="text-red-400 font-mono text-sm mb-2">"ERROR"</p>
                <p class="t-text-secondary">{message}</p>
            </div>
        </div>
    }
}
