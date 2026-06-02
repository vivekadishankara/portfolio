use leptos::prelude::*;

#[component]
pub fn NavBar(name: String) -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 t-bg-primary/80 backdrop-blur-md border-b t-border">
            <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
                <a href="#hero" class="font-syne font-bold text-lg t-text-primary hover:t-accent transition-colors">
                    {name}
                </a>
                <div class="hidden md:flex items-center gap-8 text-sm font-mono tracking-widest t-text-secondary">
                    <a href="#about"      class="hover:t-accent transition-colors uppercase">"About"</a>
                    <a href="#experience" class="hover:t-accent transition-colors uppercase">"Experience"</a>
                    <a href="#projects"   class="hover:t-accent transition-colors uppercase">"Projects"</a>
                    <a href="#skills"     class="hover:t-accent transition-colors uppercase">"Skills"</a>
                    <a href="#education"  class="hover:t-accent transition-colors uppercase">"Education"</a>
                    <a href="#contact"    class="hover:t-accent transition-colors uppercase">"Contact"</a>
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
