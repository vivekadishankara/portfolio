use leptos::prelude::*;

fn section_nav_label(key: &str) -> &'static str {
    match key {
        "experience"     => "Experience",
        "projects"       => "Projects",
        "skills"         => "Skills",
        "education"      => "Education",
        "certifications" => "Certifications",
        _                => "",
    }
}

#[component]
pub fn NavBar(name: String, section_order: Vec<String>) -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 t-bg-primary/80 backdrop-blur-md border-b t-border">
            <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
                <a href="#hero" class="font-syne font-bold text-lg t-text-primary hover:t-accent transition-colors">
                    {name}
                </a>
                <div class="hidden md:flex items-center gap-4 text-sm font-mono tracking-widest t-text-secondary">
                    <a href="#about" class="hover:t-accent transition-colors uppercase">"About"</a>
                    {section_order.into_iter().filter_map(|key| {
                        let label = section_nav_label(&key);
                        if label.is_empty() { return None; }
                        let anchor = format!("#{key}");
                        Some(view! {
                            <a href={anchor} class="hover:t-accent transition-colors uppercase">{label}</a>
                        })
                    }).collect_view()}
                    <a href="#contact" class="hover:t-accent transition-colors uppercase">"Contact"</a>
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t t-border py-8 px-6">
            <div class="max-w-6xl mx-auto flex flex-wrap items-center justify-between gap-4">
                <p class="font-mono text-xs t-text-muted tracking-widest">"BUILT WITH RUST + LEPTOS"</p>
                <a href="/admin/login" class="font-mono text-xs t-text-muted hover:t-text-secondary transition-colors tracking-widest">"ADMIN"</a>
            </div>
        </footer>
    }
}