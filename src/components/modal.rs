use leptos::*;

#[component]
pub fn Modal(
    #[prop(into)] show: MaybeSignal<bool>,
    on_close: Callback<()>,
    title: String,
    children: Children,
    #[prop(optional)] footer: Option<View>,
    #[prop(optional, default = "500px".to_string())] width: String,
) -> impl IntoView {
    let children_fragment = children();
    let footer_view = footer.map(|f| view! {
        <footer class="modal-footer" style="padding: 16px 24px; border-top: 1px solid #f0f0f0; display: flex; justify-content: flex-end; gap: 12px;">
            {f}
        </footer>
    }.into_view());

    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" on:click=move |_| on_close.call(())>
                <div class="modal-container" style=format!("width: {};", width) on:click=|e| e.stop_propagation()>
                    <header class="modal-header">
                        <h3>{title.clone()}</h3>
                        <button class="close-btn" on:click=move |_| on_close.call(())>"×"</button>
                    </header>
                    <div class="modal-content" style="padding: 24px; overflow-y: auto; flex: 1;">
                        {children_fragment.clone()}
                    </div>
                    {footer_view.clone()}
                </div>
            </div>
        </Show>
    }
}
