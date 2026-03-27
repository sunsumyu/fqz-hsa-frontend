use leptos::*;
use crate::api::models::InspectionTasksLedgerVO;

#[component]
pub fn LedgerPage() -> impl IntoView {
    let (selected_id, set_selected_id) = create_signal(None::<i32>);
    let (active_tab, set_active_tab) = create_signal("origin");

    let mock_data = vec![
        InspectionTasksLedgerVO {
            id: 1,
            case_no: "JC202403270001".to_string(),
            hospital_name: "XX市第一人民医院".to_string(),
            case_origin: "线索移交".to_string(),
            total_amount: 156800.50,
            status: "立案调查".to_string(),
            update_time: "2024-03-27 10:00".to_string(),
        },
        InspectionTasksLedgerVO {
            id: 2,
            case_no: "JC202403270002".to_string(),
            hospital_name: "XX大药房有限公司".to_string(),
            case_origin: "投诉举报".to_string(),
            total_amount: 12000.00,
            status: "行政处罚告知".to_string(),
            update_time: "2024-03-26 15:30".to_string(),
        },
        InspectionTasksLedgerVO {
            id: 3,
            case_no: "JC202403270003".to_string(),
            hospital_name: "XX中医门诊部".to_string(),
            case_origin: "飞行检查".to_string(),
            total_amount: 4500.00,
            status: "已结案".to_string(),
            update_time: "2024-03-20 09:12".to_string(),
        },
    ];

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
                            <th>"案源类别"</th>
                            <th>"涉案金额"</th>
                            <th>"当前环节"</th>
                            <th>"更新时间"</th>
                            <th>"操作"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {mock_data.into_iter().map(|item| {
                            let id = item.id;
                            view! {
                                <tr>
                                    <td><span class="text-primary">{item.case_no}</span></td>
                                    <td>{item.hospital_name}</td>
                                    <td>{item.case_origin}</td>
                                    <td class="text-danger">{format!("{:.2}", item.total_amount)}</td>
                                    <td><span class="status-tag">{item.status}</span></td>
                                    <td>{item.update_time}</td>
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
            {move || selected_id.get().map(|_id| view! {
                <div class="detail-drawer-mask" on:click=move |_| set_selected_id.set(None)>
                    <div class="detail-drawer" on:click=|e| e.stop_propagation()>
                        <header class="drawer-header">
                            <h3>"案件详情 - JC202403270001"</h3>
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
                            {move || match active_tab.get() {
                                "origin" => view! {
                                    <div class="info-grid">
                                        <div class="item"><label>"案件标题"</label><span>"关于线索疑似违规行为的立案调查"</span></div>
                                        <div class="item"><label>"稽查对象"</label><span>"XX市第一人民医院"</span></div>
                                        <div class="item"><label>"风险等级"</label><span class="text-danger">"高"</span></div>
                                        <div class="item"><label>"违规类型"</label><span>"欺诈骗保/串换项目"</span></div>
                                    </div>
                                }.into_view(),
                                "process" => view! {
                                    <div class="timeline">
                                        <div class="step active">"立案审批 - 已通过 (2024-03-25)"</div>
                                        <div class="step active">"调查取证 - 进行中"</div>
                                        <div class="step">"法制审核 - 待处理"</div>
                                    </div>
                                }.into_view(),
                                "docs" => view! {
                                    <ul class="doc-list">
                                        <li><i class="el-icon-document"></i> "案件处理审批表.docx"</li>
                                        <li><i class="el-icon-document"></i> "现场检查通知书.docx"</li>
                                    </ul>
                                }.into_view(),
                                _ => view! { <div>"加载中..."</div> }.into_view()
                            }}
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}
