use leptos::prelude::*;
use crate::models::Education;
use crate::api::{upsert_education, delete_education};
use super::shared::*;

#[component]
pub fn EducationEditor(
    items: Vec<Education>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing   = RwSignal::new(Option::<Education>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let (id, t) = (id.clone(), token.get());
        async move { delete_education(t, id).await }
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
        <AdminSection title="Education">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Education::default()))>
                    "+ ADD EDUCATION"
                </button>
            </div>
            {move || editing.get().map(|edu| view! {
                <EducationForm edu token show_toast on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|edu| {
                    let edu_clone = edu.clone();
                    view! {
                        <div class="border t-border p-5 flex items-start justify-between gap-4 hover:t-border-hover transition-colors">
                            <div>
                                <h3 class="font-syne font-semibold t-text-primary">{edu.degree.clone()} " in " {edu.field.clone()}</h3>
                                <p class="t-accent font-mono text-xs mt-1">{edu.institution.clone()}</p>
                                <p class="t-text-muted font-mono text-xs mt-1">
                                    {edu.start_year.clone()} " – "
                                    {if edu.current { "Present".to_string() } else { edu.end_year.clone().unwrap_or_default() }}
                                </p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(edu_clone.clone()))>"EDIT"</button>
                                <button class={btn_danger()}    on:click=move |_| { delete_action.dispatch(edu.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn EducationForm(
    edu: Education,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id          = edu.id.clone();
    let institution = RwSignal::new(edu.institution);
    let degree      = RwSignal::new(edu.degree);
    let field       = RwSignal::new(edu.field);
    let start_year  = RwSignal::new(edu.start_year);
    let end_year    = RwSignal::new(edu.end_year.unwrap_or_default());
    let current     = RwSignal::new(edu.current);
    let description = RwSignal::new(edu.description);
    let gpa         = RwSignal::new(edu.gpa.unwrap_or_default());
    let saving      = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let e = Education {
            id: id.clone(), institution: institution.get(), degree: degree.get(),
            field: field.get(), start_year: start_year.get(),
            end_year: if current.get() || end_year.get().is_empty() { None } else { Some(end_year.get()) },
            current: current.get(), description: description.get(),
            gpa: if gpa.get().is_empty() { None } else { Some(gpa.get()) },
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_education(t, e).await }
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
            <h3 class="font-syne font-bold text-lg t-accent">"Education Entry"</h3>
            <FormField label="Institution">
                <input class={input_cls()} type="text" prop:value=move || institution.get() on:input=move |e| institution.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Degree">
                    <input class={input_cls()} type="text" placeholder="B.Sc." prop:value=move || degree.get() on:input=move |e| degree.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Field of Study">
                    <input class={input_cls()} type="text" placeholder="Computer Science" prop:value=move || field.get() on:input=move |e| field.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Start Year">
                    <input class={input_cls()} type="text" placeholder="2018" prop:value=move || start_year.get() on:input=move |e| start_year.set(event_target_value(&e))/>
                </FormField>
                <FormField label="End Year">
                    <input class={input_cls()} type="text" placeholder="2022" disabled=move || current.get()
                        prop:value=move || end_year.get() on:input=move |e| end_year.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="current-edu" class="accent-emerald-500"
                    prop:checked=move || current.get() on:change=move |e| current.set(event_target_checked(&e))/>
                <label for="current-edu" class="font-mono text-xs t-text-muted tracking-widest uppercase">"Currently enrolled"</label>
            </div>
            <FormField label="Description (optional)">
                <textarea class={textarea_cls()} rows="3" prop:value=move || description.get() on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="GPA (optional)">
                <input class={input_cls()} type="text" placeholder="3.8/4.0" prop:value=move || gpa.get() on:input=move |e| gpa.set(event_target_value(&e))/>
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
