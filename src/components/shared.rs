use leptos::prelude::*;

#[component]
pub fn SectionLabel(label: &'static str) -> impl IntoView {
    view! {
        <span class="font-mono text-xs t-text-muted tracking-[0.3em] uppercase">{label}</span>
    }
}

#[component]
pub fn SectionLabelCentered(label: &'static str) -> impl IntoView {
    view! {
        <span class="font-mono text-xs t-text-muted tracking-[0.3em] uppercase block">{label}</span>
    }
}

#[component]
pub fn EmptyState(message: &'static str) -> impl IntoView {
    view! {
        <div class="text-center py-16">
            <p class="t-text-muted font-mono text-sm">{message}</p>
        </div>
    }
}

/// Renders a link only when the value is non-empty.
#[component]
pub fn OptionalLink(
    label: &'static str,
    value: String,
    /// Pass a non-empty String to render as a link; omit or pass empty string for plain text.
    #[prop(optional, into)] href: Option<String>,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-0 p-4 border t-border hover:t-border-hover transition-colors">
            <span class="font-mono text-xs t-accent tracking-widest uppercase pt-1 min-w-24">{label}</span>
            {match href.filter(|s| !s.is_empty()) {
                Some(url) => view! {
                    <a href={url} target="_blank" class="t-text-secondary hover:t-text-primary text-sm transition-colors">
                        {value}
                        <span class="t-text-muted ml-1">"↗"</span>
                    </a>
                }.into_any(),
                None => view! {
                    <span class="t-text-secondary text-sm">{value}</span>
                }.into_any(),
            }}
        </div>
    }
}

/// Tech tag pill used on experience and project cards.
#[component]
pub fn TechTag(name: String) -> impl IntoView {
    view! {
        <span class="px-2 py-1 t-bg-secondary t-text-secondary font-mono text-xs">{name}</span>
    }
}

/// Accent-coloured tech tag used on featured project cards.
#[component]
pub fn AccentTag(name: String) -> impl IntoView {
    view! {
        <span class="px-2 py-1 t-accent-dim t-accent font-mono text-xs border t-accent-border">{name}</span>
    }
}
