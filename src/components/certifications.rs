use leptos::prelude::*;
use crate::models::Certification;
use super::shared::SectionLabel;

#[component]
pub fn CertificationsSection(certifications: Vec<Certification>) -> impl IntoView {
    if certifications.is_empty() {
        return view! { <></> }.into_any();
    }
    view! {
        <section class="py-32 px-6 border-t t-border t-bg-secondary">
            <div class="max-w-6xl mx-auto">
                <SectionLabel label="06 / Certifications"/>
                <h2 class="font-syne font-bold text-4xl md:text-5xl t-text-primary mt-6 mb-16">"Credentials"</h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {certifications.into_iter().map(|c| view! { <CertCard cert=c/> }).collect_view()}
                </div>
            </div>
        </section>
    }.into_any()
}

#[component]
fn CertCard(cert: Certification) -> impl IntoView {
    view! {
        <div class="border t-border hover:t-accent-border transition-colors p-5 t-bg-secondary group">
            <h3 class="font-syne font-semibold t-text-primary mb-2 group-hover:t-accent transition-colors">
                {cert.name}
            </h3>
            <p class="t-accent font-mono text-xs tracking-wide mb-3">{cert.issuer}</p>
            <p class="t-text-muted font-mono text-xs">{cert.date}</p>
            {cert.credential_url.map(|url| view! {
                <a href={url} target="_blank"
                    class="inline-block mt-3 font-mono text-xs t-text-muted hover:t-text-primary transition-colors uppercase tracking-widest">
                    "View Credential ↗"
                </a>
            })}
        </div>
    }
}
