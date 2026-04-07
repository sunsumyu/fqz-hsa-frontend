use leptos::*;

#[component]
pub fn LedgerPage() -> impl IntoView {
    let (selected_id, set_selected_id) = create_signal(None::<i32>);
    let (active_tab, set_active_tab) = create_signal("origin");

    let ledger_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq::default(),
                page_num: 1,
                page_size: 10,
            };
            crate::api::client::post::<_, crate::api::models::WrapperResponse<crate::api::models::PageResult<crate::api::models::ViewInspTaskDTO>>>("/taskpunish/page", &req)
                .await
                .map(|resp| resp.data.map(|d| d.data).unwrap_or_default())
                .unwrap_or_default()
        }
    );

    let items = move || ledger_resource.get().unwrap_or_default();

    view! {
        <div class="ledger-container high-fidelity">
            <header class="ledger-header">
                <div class="filter-bar">
                    <div class="filter-item">
                        <label>"年份"</label>
                        <select class="form-select"><option>"2024"</option><option>"2023"</option></select>
                    </div>
                    <div class="filter-item">
                        <label>"任务状态"</label>
                        <select class="form-select">
                            <option>"全部"</option>
                            <option>"立案调查"</option>
                            <option>"法制审核"</option>
                            <option>"行政处罚"</option>
                        </select>
                    </div>
                    <div class="filter-item search">
                        <input type="text" placeholder="搜索案件编号、医院名称..." />
                    </div>
                    <button class="btn btn-primary">"查询"</button>
                </div>
            </header>

            <main class="ledger-main">
                <table class="ledger-table el-table">
                    <thead>
                        <tr>
                            <th>"案件编号"</th>
                            <th>"稽查对象"</th>
                            <th>"状态描述"</th>
                            <th>"来源方式"</th>
                            <th>"操作"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {items().into_iter().map(|item| {
                            let id = item.id;
                            let name = item.inspection_name.clone();
                            let no = item.inspection_no.clone();
                            view! {
                                <tr>
                                    <td><span class="text-primary">{no}</span></td>
                                    <td>{name}</td>
                                    <td><span class="status-tag">{item.status_desc.unwrap_or_default()}</span></td>
                                    <td>{if item.source_type == 1 { "日常巡查" } else { "专项检查" }}</td>
                                    <td>
                                        <button 
                                            class="btn btn-link btn-sm"
                                            on:click=move |_| set_selected_id.set(Some(id))
                                        >
                                            "查看详情"
                                        </button>
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            </main>

            // Detail Drawer
            {move || selected_id.get().and_then(|id| {
                items().into_iter().find(|t| t.id == id)
            }).map(|task| {
                let no = task.inspection_no.clone();
                let name = task.inspection_name.clone();
                let status = task.status_desc.clone().unwrap_or_default();
                view! {
                    <div class="detail-drawer-mask" on:click=move |_| set_selected_id.set(None)>
                        <div class="detail-drawer" on:click=|e| e.stop_propagation()>
                            <header class="drawer-header">
                                <h3>"案件详情 - " {no}</h3>
                                <button class="close-btn" on:click=move |_| set_selected_id.set(None)>"×"</button>
                            </header>
                            
                            <nav class="drawer-tabs">
                                <button 
                                    class=move || if active_tab.get() == "origin" { "tab active" } else { "tab" }
                                    on:click=move |_| set_active_tab.set("origin")
                                >
                                    "案件源详情"
                                </button>
                                <button 
                                    class=move || if active_tab.get() == "process" { "tab active" } else { "tab" }
                                    on:click=move |_| set_active_tab.set("process")
                                >
                                    "稽查进程"
                                </button>
                                <button 
                                    class=move || if active_tab.get() == "docs" { "tab active" } else { "tab" }
                                    on:click=move |_| set_active_tab.set("docs")
                                >
                                    "相关文书"
                                </button>
                            </nav>
    
                            <div class="drawer-body">
                                {move || {
                                    let name_c = name.clone();
                                    let status_c = status.clone();
                                    match active_tab.get() {
                                        "origin" => view! {
                                            <div class="info-grid">
                                                <div class="item"><label>"案件标题"</label><span>{name_c.clone()}</span></div>
                                                <div class="item"><label>"稽查对象"</label><span>{name_c}</span></div>
                                                <div class="item"><label>"当前状态"</label><span class="text-primary">{status_c}</span></div>
                                                <div class="item"><label>"违规类型"</label><span>"欺诈骗保/串换项目"</span></div>
                                            </div>
                                        }.into_view(),
                                        "process" => view! {
                                            <div class="timeline">
                                                <div class="step active">{status.clone()} " - 进行中"</div>
                                                <div class="step">"后续环节待触发"</div>
                                            </div>
                                        }.into_view(),
                                        "docs" => view! {
                                            <ul class="doc-list">
                                                <li><i class="el-icon-document"></i> "相关电子卷宗.pdf"</li>
                                            </ul>
                                        }.into_view(),
                                        _ => view! { <div>"加载中..."</div> }.into_view()
                                    }
                                }}
                            </div>

                            <footer class="drawer-footer p-3 border-top d-flex gap-2">
                                <button class="btn btn-primary btn-sm">"查阅案卷"</button>
                                <button class="btn btn-outline-danger btn-sm" 
                                    on:click=move |_| { let _ = window().alert_with_message("触发状态机操作: 终止调查"); }>
                                    "终止调查 (状态机)"
                                </button>
                                <button class="btn btn-outline-warning btn-sm"
                                    on:click=move |_| { let _ = window().alert_with_message("触发状态机操作: 案件移送"); }>
                                    "申请移送 (状态机)"
                                </button>
                            </footer>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}
