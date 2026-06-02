use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::api::admin_login;

#[component]
pub fn LoginPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let loading = RwSignal::new(false);

    let login_action = Action::new(move |_: &()| {
        let u = username.get();
        let p = password.get();
        async move {
            admin_login(u, p).await
        }
    });

    let _navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(result) = login_action.value().get() {
            loading.set(false);
            match result {
                Ok(resp) if resp.success => {
                    #[cfg(feature = "hydrate")]
                    if let Some(token) = resp.token {
                        crate::pages::admin::shared::store_auth(&token, &username.get_untracked());
                        _navigate("/admin", Default::default());
                    }
                }
                Ok(resp) => error.set(resp.message),
                Err(e) => error.set(e.to_string()),
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        loading.set(true);
        error.set(String::new());
        login_action.dispatch(());
    };

    view! {
        <div class="min-h-screen t-bg-primary text-zinc-100 font-syne flex items-center justify-center px-6">
            <div class="absolute inset-0 bg-[linear-gradient(to_right,#1a1a1a_1px,transparent_1px),linear-gradient(to_bottom,#1a1a1a_1px,transparent_1px)] bg-[size:4rem_4rem] opacity-40"></div>

            <div class="relative z-10 w-full max-w-sm">
                <a href="/" class="block font-mono text-xs t-text-muted hover:t-text-secondary transition-colors tracking-widest uppercase mb-12">
                    "← Back to Portfolio"
                </a>

                <h1 class="font-syne font-extrabold text-4xl t-text-primary mb-2">"Admin Login"</h1>
                <p class="font-mono text-xs t-text-muted tracking-widest mb-10">"PORTFOLIO MANAGEMENT"</p>

                <form on:submit=on_submit class="space-y-6">
                    <div>
                        <label class="font-mono text-xs text-zinc-500 tracking-widest uppercase block mb-2">"Username"</label>
                        <input
                            type="text"
                            autocomplete="username"
                            class="w-full t-bg-card border t-border focus:t-border-accent t-text-primary px-4 py-3 text-sm font-mono outline-none transition-colors"
                            on:input=move |ev| username.set(event_target_value(&ev))
                            prop:value=move || username.get()
                        />
                    </div>

                    <div>
                        <label class="font-mono text-xs text-zinc-500 tracking-widest uppercase block mb-2">"Password"</label>
                        <input
                            type="password"
                            autocomplete="current-password"
                            class="w-full t-bg-card border t-border focus:t-border-accent t-text-primary px-4 py-3 text-sm font-mono outline-none transition-colors"
                            on:input=move |ev| password.set(event_target_value(&ev))
                            prop:value=move || password.get()
                        />
                    </div>

                    {move || {
                        let err = error.get();
                        if !err.is_empty() {
                            view! {
                                <div class="bg-red-500/10 border border-red-500/30 px-4 py-3">
                                    <p class="text-red-400 font-mono text-xs">{err}</p>
                                </div>
                            }.into_any()
                        } else { view! { <span></span> }.into_any() }
                    }}

                    <button
                        type="submit"
                        disabled=move || loading.get()
                        class="w-full px-6 py-3 t-accent-bg t-accent-bg-hover disabled:bg-zinc-700 disabled:cursor-not-allowed t-accent-fg font-syne font-bold text-sm tracking-widest uppercase transition-all duration-200">
                        {move || if loading.get() { "SIGNING IN..." } else { "SIGN IN" }}
                    </button>
                </form>

                <p class="mt-8 font-mono text-xs t-text-muted text-center">
                    "Default: admin / admin123 — change after first login"
                </p>
            </div>
        </div>
    }
}
