use leptos::*;

#[component]
pub fn ComparisonModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    title: String,
) -> impl IntoView {
    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" on:click=move |_| set_show.set(false)>
                <div class="modal-container" on:click=|e| e.stop_propagation()>
                    <header class="modal-header">
                        <h3>{title.clone()}</h3>
                        <button class="close-btn" on:click=move |_| set_show.set(false)>"×"</button>
                    </header>
                    <div class="modal-content">
                        <div class="comparison-viewer">
                            <div class="viewer-panel">
                                <div class="panel-tag">"预取样本 / 原始图像"</div>
                                <div class="image-placeholder">
                                    <i class="el-icon-picture"></i>
                                    <span>"原始影像资料"</span>
                                </div>
                            </div>
                            <div class="viewer-panel">
                                <div class="panel-tag tag-audit">"稽核比对 / 识别结果"</div>
                                <div class="image-placeholder">
                                    <i class="el-icon-zoom-in"></i>
                                    <span>"稽核识别分析"</span>
                                </div>
                            </div>
                        </div>
                        <div class="comparison-info">
                            <div class="info-item">
                                <span class="label">"匹配度:"</span>
                                <span class="value" style="color: #ff4d4f; font-weight: bold;">"75.2% (异常)"</span>
                            </div>
                            <div class="info-item">
                                <span class="label">"识别结论:"</span>
                                <span class="value">"发现样本影像中存在疑似篡改或不一致性，建议进一步人工核查。"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
