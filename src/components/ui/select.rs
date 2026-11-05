use leptos::prelude::*;

/// ドロップダウンの1項目。`class` は文字色などの追加クラス。
#[derive(Clone)]
pub struct SelectOption<T> {
    pub value: T,
    pub label: &'static str,
    pub class: &'static str,
}

impl<T> SelectOption<T> {
    pub fn new(value: T, label: &'static str, class: &'static str) -> Self {
        Self {
            value,
            label,
            class,
        }
    }
}

/// 汎用ドロップダウン。選択中の値は `value`、変更時は `on_change` で通知する。
#[component]
pub fn Select<T>(
    options: Vec<SelectOption<T>>,
    #[prop(into)] value: Signal<T>,
    on_change: Callback<T>,
    #[prop(into)] label: String,
) -> impl IntoView
where
    T: Copy + PartialEq + Send + Sync + 'static,
{
    let (open, set_open) = signal(false);
    let options = StoredValue::new(options);

    let current = move || {
        let v = value.get();
        options.with_value(|opts| {
            opts.iter()
                .find(|o| o.value == v)
                .map(|o| (o.label, o.class))
                .unwrap_or(("", ""))
        })
    };

    view! {
        <div class="relative inline-block text-sm">
            <button
                type="button"
                class="flex min-w-44 items-center justify-between gap-3 rounded-lg border border-site-border px-3 py-2 transition-colors hover:border-site-accent"
                aria-haspopup="listbox"
                aria-label=label.clone()
                aria-expanded=move || open.get().to_string()
                on:click=move |_| set_open.update(|o| *o = !*o)
            >
                <span class=move || current().1>{move || current().0}</span>
                <span aria-hidden="true" class="text-site-muted">"▾"</span>
            </button>
            <Show when=move || open.get()>
                <button
                    type="button"
                    class="fixed inset-0 z-10 cursor-default"
                    aria-hidden="true"
                    tabindex="-1"
                    on:click=move |_| set_open.set(false)
                />
                <ul
                    role="listbox"
                    aria-label=label.clone()
                    class="absolute left-0 z-20 mt-1 min-w-full overflow-hidden rounded-lg border border-site-border bg-site-page py-1 shadow-lg"
                >
                    {options.with_value(|opts| opts.iter().map(|o| {
                        let v = o.value;
                        let (text, class) = (o.label, o.class);
                        view! {
                            <li role="option" aria-selected=move || (value.get() == v).to_string()>
                                <button
                                    type="button"
                                    class=move || format!(
                                        "block w-full px-3 py-2 text-left hover:bg-site-subtle {class} {}",
                                        if value.get() == v { "font-semibold" } else { "" }
                                    )
                                    on:click=move |_| {
                                        on_change.run(v);
                                        set_open.set(false);
                                    }
                                >
                                    {text}
                                </button>
                            </li>
                        }
                    }).collect_view())}
                </ul>
            </Show>
        </div>
    }
}
