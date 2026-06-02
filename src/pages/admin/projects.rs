use leptos::prelude::*;
use crate::models::Project;
use crate::api::{upsert_project, delete_project};
use super::shared::*;

#[component]
pub fn ProjectEditor(
    items: Vec<Project>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing   = RwSignal::new(Option::<Project>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let (id, t) = (id.clone(), token.get());
        async move { delete_project(t, id).await }
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
        <AdminSection title="Projects">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Project::default()))>
                    "+ ADD PROJECT"
                </button>
            </div>
            {move || editing.get().map(|project| view! {
                <ProjectForm project token show_toast on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="space-y-4">
                {move || items_sig.get().into_iter().map(|proj| {
                    let proj_clone = proj.clone();
                    view! {
                        <div class="border t-border p-5 flex items-start justify-between gap-4 hover:t-border-hover transition-colors">
                            <div>
                                <div class="flex items-center gap-3">
                                    <h3 class="font-syne font-semibold t-text-primary">{proj.title.clone()}</h3>
                                    {proj.featured.then(|| view! {
                                        <span class="px-2 py-0.5 t-accent-dim t-accent font-mono text-xs border t-accent-border">"FEATURED"</span>
                                    })}
                                </div>
                                <p class="t-text-muted text-xs mt-1 font-mono">{proj.description.chars().take(80).collect::<String>()}</p>
                            </div>
                            <div class="flex gap-3">
                                <button class={btn_secondary()} on:click=move |_| editing.set(Some(proj_clone.clone()))>"EDIT"</button>
                                <button class={btn_danger()}    on:click=move |_| { delete_action.dispatch(proj.id.clone()); }>"DELETE"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn ProjectForm(
    project: Project,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id           = project.id.clone();
    let title        = RwSignal::new(project.title);
    let description  = RwSignal::new(project.description);
    let long_desc    = RwSignal::new(project.long_description);
    let technologies = RwSignal::new(project.technologies.join(", "));
    let github_url   = RwSignal::new(project.github_url.unwrap_or_default());
    let live_url     = RwSignal::new(project.live_url.unwrap_or_default());
    let image_url    = RwSignal::new(project.image_url.unwrap_or_default());
    let featured     = RwSignal::new(project.featured);
    let saving       = RwSignal::new(false);

    let opt = |s: String| if s.is_empty() { None } else { Some(s) };

    let save = Action::new(move |_: &()| {
        let p = Project {
            id: id.clone(), title: title.get(), description: description.get(),
            long_description: long_desc.get(),
            technologies: technologies.get().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
            github_url: opt(github_url.get()), live_url: opt(live_url.get()),
            image_url:  opt(image_url.get()),  featured: featured.get(),
            order_index: 0,
        };
        let t = token.get();
        async move { upsert_project(t, p).await }
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
            <h3 class="font-syne font-bold text-lg t-accent">"Project Entry"</h3>
            <FormField label="Title">
                <input class={input_cls()} type="text" prop:value=move || title.get() on:input=move |e| title.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Short Description">
                <input class={input_cls()} type="text" prop:value=move || description.get() on:input=move |e| description.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Long Description">
                <textarea class={textarea_cls()} rows="5" prop:value=move || long_desc.get() on:input=move |e| long_desc.set(event_target_value(&e))/>
            </FormField>
            <FormField label="Technologies (comma-separated)">
                <input class={input_cls()} type="text" placeholder="Rust, Leptos, TailwindCSS"
                    prop:value=move || technologies.get() on:input=move |e| technologies.set(event_target_value(&e))/>
            </FormField>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="GitHub URL">
                    <input class={input_cls()} type="url" prop:value=move || github_url.get() on:input=move |e| github_url.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Live URL">
                    <input class={input_cls()} type="url" prop:value=move || live_url.get() on:input=move |e| live_url.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Image URL">
                    <input class={input_cls()} type="url" prop:value=move || image_url.get() on:input=move |e| image_url.set(event_target_value(&e))/>
                </FormField>
            </div>
            <div class="flex items-center gap-3">
                <input type="checkbox" id="featured" class="accent-emerald-500"
                    prop:checked=move || featured.get() on:change=move |e| featured.set(event_target_checked(&e))/>
                <label for="featured" class="font-mono text-xs t-text-muted tracking-widest uppercase">"Featured project"</label>
            </div>
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
