use leptos::prelude::*;
use crate::models::Profile;

#[component]
pub fn HeroSection(profile: Profile) -> impl IntoView {
    view! {
        <section id="hero" class="min-h-screen flex flex-col relative overflow-hidden px-6">
            <div class="absolute inset-0 t-hero-grid-bg opacity-50"></div>
            <div class="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] rounded-full t-accent-bg/5 blur-[120px] pointer-events-none"></div>

            <div class="h-20 flex-shrink-0"></div>

            <div class="flex-1 flex items-center relative z-10">
                <div class="max-w-6xl mx-auto w-full">
                    <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-8">
                        <TextContent profile=profile.clone()/>
                        <Avatar profile=profile/>
                    </div>
                </div>
            </div>

            <div class="flex flex-col items-center gap-1 t-text-muted opacity-50 pb-4 flex-shrink-0">
                <span class="font-mono text-xs tracking-widest">"SCROLL"</span>
                <div class="w-px h-8 bg-gradient-to-b from-zinc-600 to-transparent"></div>
            </div>
        </section>
    }
}

#[component]
fn TextContent(profile: Profile) -> impl IntoView {
    view! {
        <div class="flex-1 min-w-0">
            <p class="font-mono t-accent text-sm tracking-[0.3em] mb-4 uppercase">"Hello, I'm"</p>
            <h1 class="font-syne font-extrabold text-5xl md:text-7xl lg:text-8xl t-text-primary leading-none tracking-tight mb-4">
                {profile.name.clone()}
            </h1>
            <div class="flex items-center gap-4 mb-6">
                <div class="h-px w-12 t-accent-bg"></div>
                <p class="font-syne text-xl md:text-2xl t-text-secondary font-medium">{profile.title}</p>
            </div>
            <p class="t-text-secondary text-base md:text-lg max-w-xl leading-relaxed mb-8 font-light">
                {profile.bio.clone()}
            </p>
            <CtaButtons resume_url=profile.resume_url/>
            <SocialLinks github=profile.github linkedin=profile.linkedin twitter=profile.twitter/>
        </div>
    }
}

#[component]
fn CtaButtons(resume_url: String) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-3 mb-8">
            <a href="#contact"
                class="px-6 py-2.5 t-accent-bg text-zinc-950 font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                "Get In Touch"
            </a>
            <a href="#projects"
                class="px-6 py-2.5 border t-border hover:t-accent-border hover:t-accent t-text-secondary font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                "View Work"
            </a>
            <a href="/api/resume.pdf" download=true
                class="group relative px-6 py-2.5 border t-accent-border t-accent hover:text-zinc-950 font-syne font-bold text-sm tracking-wider transition-all duration-300 uppercase flex items-center gap-2 overflow-hidden">
                <span class="absolute inset-0 t-accent-bg translate-x-[-101%] group-hover:translate-x-0 transition-transform duration-300 ease-out -z-0"></span>
                <span class="relative z-10 flex items-center gap-2">
                    <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M4 16v2a2 2 0 002 2h12a2 2 0 002-2v-2M7 10l5 5 5-5M12 15V3"/>
                    </svg>
                    "Download Resume"
                </span>
            </a>
            {(!resume_url.is_empty()).then(|| view! {
                <a href={resume_url} target="_blank"
                    class="px-6 py-2.5 border t-border t-text-secondary font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                    "Resume ↗"
                </a>
            })}
        </div>
    }
}

#[component]
fn SocialLinks(github: String, linkedin: String, twitter: String) -> impl IntoView {
    view! {
        <div class="flex gap-6">
            {(!github.is_empty()).then(|| view! {
                <a href={github} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">"GitHub"</a>
            })}
            {(!linkedin.is_empty()).then(|| view! {
                <a href={linkedin} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">"LinkedIn"</a>
            })}
            {(!twitter.is_empty()).then(|| view! {
                <a href={twitter} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">"Twitter"</a>
            })}
        </div>
    }
}

#[component]
fn Avatar(profile: Profile) -> impl IntoView {
    (!profile.avatar_url.is_empty()).then(|| view! {
        <div class="flex-shrink-0 flex justify-center lg:justify-end lg:self-start lg:mt-10">
            <div class="relative">
                <div class="absolute -inset-1 rounded-full t-accent-bg opacity-20 blur-sm"></div>
                <div class="relative w-40 h-40 md:w-52 md:h-52 rounded-full overflow-hidden border-2 t-accent-border">
                    <img src={profile.avatar_url} alt={profile.name} class="w-full h-full object-cover"/>
                </div>
            </div>
        </div>
    })
}
