use leptos::prelude::*;
use crate::models::Experience;
use crate::api::{upsert_experience, delete_experience};
use super::shared::*;

#[component]
pub fn ExperienceEditor(
    items: Vec<Experience>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing   = RwSignal::new(Option::<Experience>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let (id, t) = (id.clone(), token.get());
        async move { delete_experience(t, id).await }
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
        <AdminSection title="Work Experience">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Experience::default()))>
                    "+ ADD EXPERIENCE"
                </button>
            </div>
            {move || editing.get().map(|exp| view! {
                <ExperienceForm exp token show_toast on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|exp| {
                    let exp_clone = exp.clone();
                    view! {
                        <div class="border t-border p-5 flex items-start justify-between gap-4 hover:t-border-hover transition-colors">
                            <div class="flex-1">
                                <h3 class="font-syne font-semibold t-text-primary">{exp.role.clone()}</h3>
                                <p class="t-accent font-mono text-xs mt-1">{exp.company.clone()}</p>
                                <p class="t-text-muted font-mono text-xs mt-1">
                                    {exp.start_date.clone()} " – "
                                    {if exp.current { "Present".to_string() } else { exp.end_date.clone().unwrap_or_default() }}
                                </p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(exp_clone.clone()))>"EDIT"</button>
                                <button class={btn_danger()}    on:click=move |_| { delete_action.dispatch(exp.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn ExperienceForm(
    exp: Experience,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id           = exp.id.clone();
    let company      = RwSignal::new(exp.company);
    let role         = RwSignal::new(exp.role);
    let start_date   = RwSignal::new(exp.start_date);
    let end_date     = RwSignal::new(exp.end_date.unwrap_or_default());
    let current      = RwSignal::new(exp.current);
    let description  = RwSignal::new(exp.description.join("\n"));
    let technologies = RwSignal::new(exp.technologies.join(", "));
    let saving       = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let e = Experience {
            id: id.clone(), company: company.get(), role: role.get(),
            start_date: start_date.get(),
            end_date: if current.get() || end_date.get().is_empty() { None } else { Some(end_date.get()) },
            current: current.get(),
            description:  description.get().lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            technologies: technologies.get().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_experience(t, e).await }
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
        <div class="mb-8 border t-accent-border t-accent-dim p-6 space-y-5">
            <h3 class="font-syne font-bold text-lg t-accent">"Experience Entry"</h3>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Company">
                    <input class={input_cls()} type="text" prop:value=move || company.get() on:input=move |e| company.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Role / Title">
                    <input class={input_cls()} type="text" prop:value=move || role.get() on:input=move |e| role.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Start Date">
                    <input class={input_cls()} type="text" placeholder="Jan 2022" prop:value=move || start_date.get() on:input=move |e| start_date.set(event_target_value(&e))/>
                </FormField>
                <FormField label="End Date">
                    <input class={input_cls()} type="text" placeholder="Dec 2023" disabled=move || current.get()
                        prop:value=move || end_date.get() on:input=move |e| end_date.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="current-exp" class="accent-emerald-500"
                    prop:checked=move || current.get() on:change=move |e| current.set(event_target_checked(&e))/>
                <label for="current-exp" class="font-mono text-xs t-text-muted tracking-widest uppercase">
                    "Currently working here"
                </label>
            </div>
            <FormField label="Description (one bullet point per line)">
                <textarea class={textarea_cls()} rows="6"
                    prop:value=move || description.get()
                    on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Technologies (comma-separated)">
                <input class={input_cls()} type="text" placeholder="Rust, TypeScript, PostgreSQL"
                    prop:value=move || technologies.get() on:input=move |e| technologies.set(event_target_value(&e))/>
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
