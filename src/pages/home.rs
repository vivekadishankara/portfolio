use leptos::prelude::*;
use crate::models::*;
use crate::api::get_portfolio_data;

#[component]
pub fn HomePage() -> impl IntoView {
    let data = Resource::new(|| (), |_| get_portfolio_data());

    view! {
        <div class="min-h-screen t-bg-primary t-text-primary font-syne">
            <Suspense fallback=move || view! { <LoadingScreen/> }>
                {move || data.get().map(|result| {
                    match result {
                        Ok(portfolio) => view! { <PortfolioContent portfolio=portfolio/> }.into_any(),
                        Err(e) => view! { <ErrorScreen message=e.to_string()/> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
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

#[component]
fn PortfolioContent(portfolio: PortfolioData) -> impl IntoView {
    let profile = portfolio.profile.clone();
    let experiences = portfolio.experiences.clone();
    let educations = portfolio.educations.clone();
    let projects = portfolio.projects.clone();
    let skills = portfolio.skills.clone();
    let certifications = portfolio.certifications.clone();

    view! {
        <NavBar name=profile.name.clone()/>
        <HeroSection profile=profile.clone()/>
        <AboutSection profile=profile.clone()/>
        <ExperienceSection experiences=experiences/>
        <ProjectsSection projects=projects/>
        <SkillsSection skills=skills/>
        <EducationSection educations=educations/>
        <CertificationsSection certifications=certifications/>
        <ContactSection profile=profile/>
        <Footer/>
    }
}

// ─── Navigation ──────────────────────────────────────────────────────────────

#[component]
fn NavBar(name: String) -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 t-bg-primary/80 backdrop-blur-md border-b t-border">
            <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
                <a href="#hero" class="font-syne font-bold text-lg t-text-primary hover:t-accent transition-colors">
                    {name}
                </a>
                <div class="hidden md:flex items-center gap-8 text-sm font-mono tracking-widest t-text-secondary">
                    <a href="#about" class="hover:t-accent transition-colors uppercase">"About"</a>
                    <a href="#experience" class="hover:t-accent transition-colors uppercase">"Experience"</a>
                    <a href="#projects" class="hover:t-accent transition-colors uppercase">"Projects"</a>
                    <a href="#skills" class="hover:t-accent transition-colors uppercase">"Skills"</a>
                    <a href="#education" class="hover:t-accent transition-colors uppercase">"Education"</a>
                    <a href="#contact" class="hover:t-accent transition-colors uppercase">"Contact"</a>
                </div>
            </div>
        </nav>
    }
}

// ─── Hero ─────────────────────────────────────────────────────────────────────

#[component]
fn HeroSection(profile: Profile) -> impl IntoView {
    view! {
        <section id="hero" class="min-h-screen flex flex-col justify-center relative overflow-hidden px-6 pt-20">
            // Grid background
            <div class="absolute inset-0 bg-[linear-gradient(to_right,#1a1a1a_1px,transparent_1px),linear-gradient(to_bottom,#1a1a1a_1px,transparent_1px)] bg-[size:4rem_4rem] opacity-40"></div>
            // Glow accent
            <div class="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] rounded-full t-accent-bg/5 blur-[120px] pointer-events-none"></div>

            <div class="max-w-6xl mx-auto w-full relative z-10">
                <p class="font-mono t-accent text-sm tracking-[0.3em] mb-6 uppercase">"Hello, I'm"</p>
                <h1 class="font-syne font-extrabold text-6xl md:text-8xl lg:text-9xl t-text-primary leading-none tracking-tight mb-6">
                    {profile.name}
                </h1>
                <div class="flex items-center gap-4 mb-8">
                    <div class="h-px w-12 t-accent-bg"></div>
                    <p class="font-syne text-xl md:text-2xl t-text-secondary font-medium">{profile.title}</p>
                </div>
                <p class="t-text-secondary text-lg max-w-xl leading-relaxed mb-12 font-light">{profile.bio.clone()}</p>

                <div class="flex flex-wrap gap-4">
                    <a href="#contact"
                        class="px-8 py-3 t-accent-bg hover:t-accent-bg text-zinc-950 font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                        "Get In Touch"
                    </a>
                    <a href="#projects"
                        class="px-8 py-3 border t-border-hover hover:t-accent-border hover:t-accent t-text-secondary font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                        "View Work"
                    </a>
                    {if !profile.resume_url.is_empty() {
                        view! {
                            <a href={profile.resume_url} target="_blank"
                                class="px-8 py-3 border border-zinc-700 hover:border-zinc-500 t-text-secondary hover:text-zinc-200 font-syne font-bold text-sm tracking-wider transition-all duration-200 uppercase">
                                "Resume ↗"
                            </a>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                </div>

                <div class="flex gap-6 mt-12">
                    {if !profile.github.is_empty() {
                        view! {
                            <a href={profile.github} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">
                                "GitHub"
                            </a>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                    {if !profile.linkedin.is_empty() {
                        view! {
                            <a href={profile.linkedin} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">
                                "LinkedIn"
                            </a>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                    {if !profile.twitter.is_empty() {
                        view! {
                            <a href={profile.twitter} target="_blank" class="t-text-muted hover:t-accent transition-colors font-mono text-sm tracking-widest uppercase">
                                "Twitter"
                            </a>
                        }.into_any()
                    } else { view! { <span></span> }.into_any() }}
                </div>
            </div>

            <div class="absolute bottom-12 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 t-text-muted">
                <span class="font-mono text-xs tracking-widest">"SCROLL"</span>
                <div class="w-px h-12 bg-gradient-to-b from-zinc-600 to-transparent"></div>
            </div>
        </section>
    }
}

// ─── About ────────────────────────────────────────────────────────────────────

#[component]
fn AboutSection(profile: Profile) -> impl IntoView {
    view! {
        <section id="about" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="01 / About"/>
                <div class="grid lg:grid-cols-2 gap-16 mt-16 items-start">
                    <div>
                        <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mb-8 leading-tight">
                            "A developer who cares about the details"
                        </h2>

                        // Summary if available, fallback to bio
                        {if !profile.summary.is_empty() {
                            view! {
                                <p class="t-text-secondary text-lg leading-relaxed mb-4">{profile.summary.clone()}</p>
                            }.into_any()
                        } else {
                            view! {
                                <p class="t-text-secondary text-lg leading-relaxed mb-4">{profile.bio.clone()}</p>
                            }.into_any()
                        }}

                        <div class="flex items-center gap-3 t-text-muted font-mono text-sm mt-6">
                            <span class="t-accent">{"📍"}</span>
                            <span>{profile.location}</span>
                        </div>
                    </div>
                    <div class="space-y-4">
                        <InfoCard label="Email" value=profile.email link=None/>
                        {if !profile.github.is_empty() {
                            view! { <InfoCard label="GitHub" value=profile.github.replace("https://", "") link=Some(profile.github.clone())/> }.into_any()
                        } else { view! { <span></span> }.into_any() }}
                        {if !profile.linkedin.is_empty() {
                            view! { <InfoCard label="LinkedIn" value="View Profile".to_string() link=Some(profile.linkedin.clone())/> }.into_any()
                        } else { view! { <span></span> }.into_any() }}
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn InfoCard(label: &'static str, value: String, link: Option<String>) -> impl IntoView {
    view! {
        <div class="flex items-start gap-4 p-4 border t-border hover:t-border-hover transition-colors">
            <span class="font-mono text-xs t-accent tracking-widest uppercase pt-1 min-w-24">{label}</span>
            {if let Some(href) = link {
                view! {
                    <a href={href} target="_blank" class="t-text-secondary hover:t-text-primary text-sm transition-colors">
                        {value}
                        <span class="t-text-muted ml-1">"↗"</span>
                    </a>
                }.into_any()
            } else {
                view! { <span class="t-text-secondary text-sm">{value}</span> }.into_any()
            }}
        </div>
    }
}

// ─── Experience ───────────────────────────────────────────────────────────────

#[component]
fn ExperienceSection(experiences: Vec<Experience>) -> impl IntoView {
    view! {
        <section id="experience" class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="02 / Experience"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Work History"</h2>

                {if experiences.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <p class="t-text-muted font-mono text-sm">"No experience added yet"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="relative">
                            <div class="absolute left-0 top-0 bottom-0 w-px t-bg-secondary"></div>
                            <div class="space-y-12">
                                {experiences.into_iter().map(|exp| view! { <ExperienceCard exp=exp/> }).collect_view()}
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
    view! {
        <div class="relative pl-10">
            <div class="absolute left-0 top-2 w-2 h-2 -translate-x-[4.5px] rounded-full t-accent-bg"></div>
            <div class="t-bg-card border t-border p-6 hover:t-border-hover transition-colors">
                <div class="mb-4">
                    <div class="flex items-baseline gap-6 flex-wrap">
                        <h3 class="font-syne font-bold text-xl t-text-primary shrink-0">{exp.role}</h3>
                        <span class="font-mono text-xs t-text-muted tracking-wider whitespace-nowrap">
                            {exp.start_date.clone()}
                            {if exp.current {
                                " – Present".to_string()
                            } else {
                                exp.end_date.clone()
                                    .filter(|s| !s.is_empty())
                                    .map(|d| format!(" – {}", d))
                                    .unwrap_or_default()
                            }}
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
                    {exp.technologies.into_iter().map(|tech| view! {
                        <span class="px-2 py-1 t-bg-secondary t-text-secondary font-mono text-xs">{tech}</span>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

// ─── Projects ─────────────────────────────────────────────────────────────────

#[component]
fn ProjectsSection(projects: Vec<Project>) -> impl IntoView {
    let featured: Vec<Project> = projects.iter().filter(|p| p.featured).cloned().collect();
    let rest: Vec<Project> = projects.iter().filter(|p| !p.featured).cloned().collect();

    view! {
        <section id="projects" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="03 / Projects"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Selected Work"</h2>

                {if projects.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <p class="t-text-muted font-mono text-sm">"No projects added yet"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            {if !featured.is_empty() {
                                view! {
                                    <div class="space-y-8 mb-16">
                                        {featured.into_iter().map(|p| view! { <FeaturedProjectCard project=p/> }).collect_view()}
                                    </div>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}

                            {if !rest.is_empty() {
                                view! {
                                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                                        {rest.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}
                                    </div>
                                }.into_any()
                            } else { view! { <span></span> }.into_any() }}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

#[component]
fn FeaturedProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="border t-border hover:border-emerald-500/40 transition-all duration-300 p-8 t-bg-secondary group">
            <div class="flex flex-wrap items-start justify-between gap-4 mb-6">
                <div>
                    <span class="font-mono text-xs t-accent tracking-widest uppercase mb-3 block">"Featured Project"</span>
                    <h3 class="font-syne font-bold text-2xl t-text-primary group-hover:t-accent transition-colors">{project.title}</h3>
                </div>
                <div class="flex gap-4">
                    {project.github_url.as_ref().map(|url| view! {
                        <a href={url.clone()} target="_blank" class="font-mono text-xs t-text-muted hover:t-text-primary transition-colors uppercase tracking-widest">
                            "GitHub ↗"
                        </a>
                    })}
                    {project.live_url.as_ref().map(|url| view! {
                        <a href={url.clone()} target="_blank" class="font-mono text-xs t-text-muted hover:t-accent transition-colors uppercase tracking-widest">
                            "Live ↗"
                        </a>
                    })}
                </div>
            </div>
            <p class="t-text-secondary leading-relaxed mb-6">{if project.long_description.is_empty() { project.description.clone() } else { project.long_description.clone() }}</p>
            <div class="flex flex-wrap gap-2">
                {project.technologies.into_iter().map(|tech| view! {
                    <span class="px-2 py-1 t-accent-bg/10 t-accent font-mono text-xs border t-accent-border">{tech}</span>
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="border t-border hover:t-border-hover transition-all duration-300 p-6 t-bg-secondary group flex flex-col h-full">
            <div class="flex items-start justify-between mb-4">
                <h3 class="font-syne font-bold text-lg t-text-primary group-hover:t-accent transition-colors">{project.title}</h3>
                <div class="flex gap-3 ml-4">
                    {project.github_url.as_ref().map(|url| view! {
                        <a href={url.clone()} target="_blank" class="t-text-muted hover:t-text-primary transition-colors text-xs font-mono">"GH"</a>
                    })}
                    {project.live_url.as_ref().map(|url| view! {
                        <a href={url.clone()} target="_blank" class="t-text-muted hover:t-accent transition-colors text-xs font-mono">"↗"</a>
                    })}
                </div>
            </div>
            <p class="t-text-muted text-sm leading-relaxed flex-1 mb-4">{project.description}</p>
            <div class="flex flex-wrap gap-1.5">
                {project.technologies.into_iter().take(4).map(|tech| view! {
                    <span class="px-2 py-0.5 t-bg-secondary t-text-secondary font-mono text-xs">{tech}</span>
                }).collect_view()}
            </div>
        </div>
    }
}

// ─── Skills ───────────────────────────────────────────────────────────────────

#[component]
fn SkillsSection(skills: Vec<Skill>) -> impl IntoView {
    let mut categories: std::collections::HashMap<String, Vec<Skill>> = std::collections::HashMap::new();
    for skill in skills {
        categories.entry(skill.category.clone()).or_default().push(skill);
    }
    let mut categories: Vec<(String, Vec<Skill>)> = categories.into_iter().collect();
    categories.sort_by(|a, b| a.0.cmp(&b.0));

    view! {
        <section id="skills" class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="04 / Skills"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Technical Stack"</h2>

                {if categories.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <p class="t-text-muted font-mono text-sm">"No skills added yet"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                            {categories.into_iter().map(|(category, skills)| view! {
                                <div>
                                    <h3 class="font-mono text-xs t-accent tracking-widest uppercase mb-4">{category}</h3>
                                    <div class="space-y-3">
                                        {skills.into_iter().map(|skill| view! {
                                            <div class="flex items-center justify-between">
                                                <span class="t-text-secondary text-sm font-medium">{skill.name}</span>
                                                <div class="flex gap-1">
                                                    {(1..=5).map(|i| view! {
                                                        <div class={if i <= skill.level {
                                                            "w-2 h-2 t-accent-bg"
                                                        } else {
                                                            "w-2 h-2 bg-zinc-700"
                                                        }}></div>
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

// ─── Education ────────────────────────────────────────────────────────────────

#[component]
fn EducationSection(educations: Vec<Education>) -> impl IntoView {
    view! {
        <section id="education" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="05 / Education"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Academic Background"</h2>

                {if educations.is_empty() {
                    view! {
                        <div class="text-center py-16">
                            <p class="t-text-muted font-mono text-sm">"No education added yet"</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid md:grid-cols-2 gap-6">
                            {educations.into_iter().map(|edu| view! {
                                <div class="border t-border hover:t-border-hover transition-colors p-6 t-bg-secondary">
                                    <div class="flex flex-wrap justify-between items-start gap-4 mb-4">
                                        <div>
                                            <h3 class="font-syne font-bold text-lg t-text-primary">{edu.degree.clone()} " in " {edu.field}</h3>
                                            <p class="t-accent font-mono text-sm mt-1">{edu.institution}</p>
                                        </div>
                                        <span class="font-mono text-xs t-text-muted">
                                            {edu.start_year} " – " {if edu.current { "Present".to_string() } else { edu.end_year.unwrap_or_default() }}
                                        </span>
                                    </div>
                                    {if !edu.description.is_empty() {
                                        view! { <p class="t-text-secondary text-sm leading-relaxed">{edu.description}</p> }.into_any()
                                    } else { view! { <span></span> }.into_any() }}
                                    {edu.gpa.as_ref().map(|gpa| view! {
                                        <p class="font-mono text-xs t-text-muted mt-3">"GPA: " {gpa.clone()}</p>
                                    })}
                                </div>
                            }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </section>
    }
}

// ─── Certifications ───────────────────────────────────────────────────────────

#[component]
fn CertificationsSection(certifications: Vec<Certification>) -> impl IntoView {
    if certifications.is_empty() {
        return view! { <span></span> }.into_any();
    }

    view! {
        <section class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="06 / Certifications"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Credentials"</h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {certifications.into_iter().map(|cert| view! {
                        <div class="border t-border hover:border-emerald-500/40 transition-colors p-5 t-bg-secondary group">
                            <h3 class="font-syne font-semibold t-text-primary mb-2 group-hover:t-accent transition-colors">{cert.name}</h3>
                            <p class="t-accent font-mono text-xs tracking-wide mb-3">{cert.issuer}</p>
                            <p class="t-text-muted font-mono text-xs">{cert.date}</p>
                            {cert.credential_url.as_ref().map(|url| view! {
                                <a href={url.clone()} target="_blank"
                                    class="inline-block mt-3 font-mono text-xs t-text-muted hover:t-text-primary transition-colors uppercase tracking-widest">
                                    "View Credential ↗"
                                </a>
                            })}
                        </div>
                    }).collect_view()}
                </div>
            </div>
        </section>
    }.into_any()
}

// ─── Contact ──────────────────────────────────────────────────────────────────

#[component]
fn ContactSection(profile: Profile) -> impl IntoView {
    view! {
        <section id="contact" class="py-32 px-6 border-t t-border">
            <div class="max-w-6xl mx-auto text-center">
                <SectionLabelCentered label="07 / Contact"/>
                <h2 class="font-syne font-bold text-5xl md:text-7xl t-text-primary mt-6 mb-6">"Let's Work Together"</h2>
                <p class="t-text-secondary text-lg max-w-xl mx-auto mb-12">"Have a project in mind or want to chat? My inbox is always open."</p>
                <a href={format!("mailto:{}", profile.email)}
                    class="inline-block px-12 py-4 t-accent-bg hover:t-accent-bg text-zinc-950 font-syne font-bold text-sm tracking-widest uppercase transition-all duration-200 hover:scale-105">
                    "Say Hello ↗"
                </a>
            </div>
        </section>
    }
}

// ─── Footer ───────────────────────────────────────────────────────────────────

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t t-border py-8 px-6">
            <div class="max-w-6xl mx-auto flex flex-wrap items-center justify-between gap-4">
                <p class="font-mono text-xs t-text-muted tracking-widest">"BUILT WITH RUST + LEPTOS"</p>
                <a href="/admin/login" class="font-mono text-xs t-text-muted hover:t-text-muted transition-colors tracking-widest">"ADMIN"</a>
            </div>
        </footer>
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

#[component]
fn SectionLabel(label: &'static str) -> impl IntoView {
    view! {
        <span class="font-mono text-xs t-text-muted tracking-[0.3em] uppercase">{label}</span>
    }
}

#[component]
fn SectionLabelCentered(label: &'static str) -> impl IntoView {
    view! {
        <span class="font-mono text-xs t-text-muted tracking-[0.3em] uppercase block">{label}</span>
    }
}
