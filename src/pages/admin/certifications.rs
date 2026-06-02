use leptos::prelude::*;
use crate::models::Certification;
use crate::api::{upsert_certification, delete_certification};
use super::shared::*;

#[component]
pub fn CertificationEditor(
    items: Vec<Certification>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing   = RwSignal::new(Option::<Certification>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let (id, t) = (id.clone(), token.get());
        async move { delete_certification(t, id).await }
    });

    Effect::new(move |_| {
        if let Some(r) = delete_action.value().get() {
            match r {
                Ok(_)  => { show_toast.run((true, "Deleted!".into())); on_refresh.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <AdminSection title="Certifications">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Certification::default()))>
                    "+ ADD CERTIFICATION"
                </button>
            </div>
            {move || editing.get().map(|cert| view! {
                <CertificationForm cert token show_toast on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|cert| {
                    let cert_clone = cert.clone();
                    view! {
                        <div class="border t-border p-5 flex items-start justify-between gap-4 hover:t-border-hover transition-colors">
                            <div>
                                <h3 class="font-syne font-semibold t-text-primary">{cert.name.clone()}</h3>
                                <p class="t-accent font-mono text-xs mt-1">{cert.issuer.clone()}</p>
                                <p class="t-text-muted font-mono text-xs">{cert.date.clone()}</p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(cert_clone.clone()))>"EDIT"</button>
                                <button class={btn_danger()}    on:click=move |_| { delete_action.dispatch(cert.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn CertificationForm(
    cert: Certification,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id       = cert.id.clone();
    let name     = RwSignal::new(cert.name);
    let issuer   = RwSignal::new(cert.issuer);
    let date     = RwSignal::new(cert.date);
    let expiry   = RwSignal::new(cert.expiry_date.unwrap_or_default());
    let cred_url = RwSignal::new(cert.credential_url.unwrap_or_default());
    let saving   = RwSignal::new(false);

    let opt = |s: String| if s.is_empty() { None } else { Some(s) };

    let save = Action::new(move |_: &()| {
        let c = Certification {
            id: id.clone(), name: name.get(), issuer: issuer.get(), date: date.get(),
            expiry_date:    opt(expiry.get()),
            credential_url: opt(cred_url.get()),
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_certification(t, c).await }
    });

    Effect::new(move |_| {
        if let Some(r) = save.value().get() {
            saving.set(false);
            match r {
                Ok(_)  => { show_toast.run((true, "Saved!".into())); on_refresh.run(()); on_close.run(()); }
                Err(e) => show_toast.run((false, e.to_string())),
            }
        }
    });

    view! {
        <div class="mb-6 border t-accent-border t-accent-dim p-6 space-y-4">
            <h3 class="font-syne font-bold text-lg t-accent">"Certification Entry"</h3>
            <FormField label="Certification Name">
                <input class={input_cls()} type="text"
                    prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Issuing Organization">
                <input class={input_cls()} type="text"
                    prop:value=move || issuer.get() on:input=move |e| issuer.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Issue Date">
                    <input class={input_cls()} type="text" placeholder="Jan 2024"
                        prop:value=move || date.get() on:input=move |e| date.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Expiry Date (optional)">
                    <input class={input_cls()} type="text" placeholder="Jan 2027"
                        prop:value=move || expiry.get() on:input=move |e| expiry.set(event_target_value(&e))/>
                </FormField>
            </div>
            <FormField label="Credential URL (optional)">
                <input class={input_cls()} type="url"
                    prop:value=move || cred_url.get() on:input=move |e| cred_url.set(event_target_value(&e))/>
            </FormField>
            <div class="flex gap-4">
                <button class={btn_primary()} disabled=move || saving.get()
                    on:click=move |_| { saving.set(true); save.dispatch(()); }>
                    {move || if saving.get() { "SAVING..." } else { "SAVE" }}
                </button>
                <button class={btn_secondary()} on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}
