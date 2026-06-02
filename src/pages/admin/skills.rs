use leptos::prelude::*;
use crate::models::Skill;
use crate::api::{upsert_skill, delete_skill};
use super::shared::*;

#[component]
pub fn SkillEditor(
    items: Vec<Skill>,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
) -> impl IntoView {
    let editing   = RwSignal::new(Option::<Skill>::None);
    let items_sig = RwSignal::new(items);

    let delete_action = Action::new(move |id: &String| {
        let (id, t) = (id.clone(), token.get());
        async move { delete_skill(t, id).await }
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
        <AdminSection title="Skills">
            <div class="mb-6">
                <button class={btn_primary()} on:click=move |_| editing.set(Some(Skill::default()))>
                    "+ ADD SKILL"
                </button>
            </div>
            {move || editing.get().map(|skill| view! {
                <SkillForm skill token show_toast on_refresh
                    on_close=Callback::new(move |_: ()| editing.set(None))/>
            })}
            <div class="grid md:grid-cols-2 gap-3">
                {move || items_sig.get().into_iter().map(|skill| {
                    let skill_clone = skill.clone();
                    view! {
                        <div class="border t-border p-4 flex items-center justify-between hover:t-border-hover transition-colors">
                            <div>
                                <div class="flex items-center gap-3">
                                    <span class="font-medium t-text-primary text-sm">{skill.name.clone()}</span>
                                    <div class="flex gap-1">
                                        {(1..=5).map(|i| {
                                            let lvl = skill.level;
                                            view! { <div class={if i <= lvl { "w-1.5 h-1.5 t-accent-bg" } else { "w-1.5 h-1.5 t-border" }}></div> }
                                        }).collect_view()}
                                    </div>
                                </div>
                                <span class="font-mono text-xs t-text-muted">{skill.category.clone()}</span>
                            </div>
                            <div class="flex gap-2">
                                <button class="font-mono text-xs t-text-muted hover:t-text-primary transition-colors px-3 py-1"
                                    on:click=move |_| editing.set(Some(skill_clone.clone()))>"EDIT"</button>
                                <button class="font-mono text-xs text-red-500 hover:text-red-400 transition-colors px-3 py-1"
                                    on:click=move |_| { delete_action.dispatch(skill.id.clone()); }>"×"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </AdminSection>
    }
}

#[component]
fn SkillForm(
    skill: Skill,
    token: RwSignal<String>,
    show_toast: Callback<(bool, String)>,
    on_refresh: Callback<()>,
    on_close: Callback<()>,
) -> impl IntoView {
    let id       = skill.id.clone();
    let name     = RwSignal::new(skill.name);
    let category = RwSignal::new(skill.category);
    let level    = RwSignal::new(skill.level);
    let saving   = RwSignal::new(false);

    let save = Action::new(move |_: &()| {
        let s = Skill { id: id.clone(), name: name.get(), category: category.get(), level: level.get(), order_index: 0 };
        let t = token.get();
        async move { upsert_skill(t, s).await }
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
            <h3 class="font-syne font-bold text-lg t-accent">"Skill Entry"</h3>
            <div class="grid grid-cols-2 gap-4">
                <FormField label="Skill Name">
                    <input class={input_cls()} type="text" placeholder="Rust"
                        prop:value=move || name.get() on:input=move |e| name.set(event_target_value(&e))/>
                </FormField>
                <FormField label="Category">
                    <input class={input_cls()} type="text" placeholder="Languages, Frameworks, Tools..."
                        prop:value=move || category.get() on:input=move |e| category.set(event_target_value(&e))/>
                </FormField>
            </div>
            <FormField label="Proficiency Level (1–5)">
                <div class="flex items-center gap-4">
                    <input type="range" min="1" max="5" class="t-accent flex-1"
                        prop:value=move || level.get().to_string()
                        on:input=move |e| level.set(event_target_value(&e).parse().unwrap_or(3))/>
                    <div class="flex gap-1">
                        {move || (1..=5).map(|i| view! {
                            <div class={if i <= level.get() { "w-3 h-3 t-accent-bg" } else { "w-3 h-3 t-border" }}></div>
                        }).collect_view()}
                    </div>
                </div>
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
