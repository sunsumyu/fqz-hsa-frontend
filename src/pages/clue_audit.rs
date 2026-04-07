use leptos::*;
use leptos_router::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, PageVO, InspectionTasksReq, WrapperResponse, PageResult, InspPrecheckReq};
use crate::api::constants::PunishStatus;

#[component]
pub fn ClueAuditPage() -> impl IntoView {
    let (show_filing_modal, set_show_filing_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i64>);
    
    // Popup form state
    let (case_repeat_flag, set_case_repeat_flag) = create_signal(0);
    let (checked_reason, set_checked_reason) = create_signal(String::new());
    let (punish_method, set_punish_method) = create_signal(2); // 2: 立案调查

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = PageVO {
                condition: InspectionTasksReq {
                    inspection_status: Some(PunishStatus::WaitPreAudit as i32),
                    ..Default::default()
                },
                page_num: 1,
                page_size: 10,
            };
            crate::api::client::post::<_, WrapperResponse<PageResult<InspectionTask>>>("/insp/tasks/page", &req)
                .await
                .map(|resp| resp.data.map(|d| d.data).unwrap_or_default())
                .unwrap_or_default()
        }
    );

    let handle_precheck_save = move |_| {
        let id = selected_id.get().unwrap_or(0) as i32;
        if id == 0 { return; }
        
        let req = InspPrecheckReq {
            inspection_id: Some(id),
            case_repeat_flag: Some(case_repeat_flag.get()),
            case_repeat_id: None,
            event: Some("1".to_string()),
            checked_reason: Some(checked_reason.get()),
            punish_method: Some(punish_method.get()),
            result: None,
            punish_submit: None,
        };

        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/insp/tasks/precheck", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("提交成功！");
                    set_show_filing_modal.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("操作失败: {}", e));
                }
            }
        });
    };

    let columns = vec![
        TableColumn::new("序号".to_string(), |t: InspectionTask| t.id.unwrap_or(0).to_string()),
        TableColumn::new("任务ID".to_string(), |t: InspectionTask| t.task_id.unwrap_or(0).to_string()),
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.inspection_no.clone().unwrap_or_default()),
        TableColumn::new("稽查标题".to_string(), |t: InspectionTask| t.inspection_name.clone().unwrap_or_default()),
        TableColumn::new("被稽查点".to_string(), |t: InspectionTask| t.inspection_name.clone().unwrap_or_else(|| "刀疤刘".to_string())),
        TableColumn::new("指派时间".to_string(), |t: InspectionTask| t.assign_time.clone().unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let id = t.id.unwrap_or(0);
            view! {
                <div class="table-actions">
                    <button class="btn btn-sm">"协议处罚"</button>
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_id.set(Some(id as i64));
                        set_show_filing_modal.set(true);
                    }>
                        "立案调查"
                    </button>
                </div>
            }
        }),
    ];

    let modal_footer = move || view! {
        <button class="btn" on:click=move |_| set_show_filing_modal.set(false)>"取消"</button>
        <button 
            class="btn" 
            on:click=move |_| {
                if let Some(id) = selected_id.get() {
                    let navigate = use_navigate();
                    navigate(&format!("/document-edit/{}", id), Default::default());
                } else {
                    let _ = window().alert_with_message("请先选择一条任务。");
                }
            }
        >
            "填写文书"
        </button>
        <button class="btn btn-primary" on:click=handle_precheck_save>"保存"</button>
    }.into_view();

    view! {
        <div class="page-container">
            <header class="page-header">
                <h2>"线索分配与稽查"</h2>
            </header>

            <div class="view-container">
                <div class="filter-bar">
                    <div class="filter-item">
                        <label>"稽查对象"</label>
                        <input type="text" placeholder="输入名称/编码..." />
                    </div>
                    <div class="filter-item">
                        <label>"任务状态"</label>
                        <select>
                            <option value="130">"待预检"</option>
                            <option value="110">"稽查中"</option>
                        </select>
                    </div>
                    <button class="btn btn-primary" on:click=move |_| task_resource.refetch()>"查询"</button>
                    <button class="btn">"重置"</button>
                </div>

                <div class="data-table-wrapper">
                    {move || match task_resource.get() {
                        Some(data) => view! { 
                            <DataTable data=data columns=columns.clone() /> 
                        }.into_view(),
                        None => view! { <div class="loading">"加载中..."</div> }.into_view()
                    }}
                </div>
            </div>

            <Modal 
                show=show_filing_modal 
                title="立案调查预检结论".to_string() 
                on_close=Callback::new(move |_| set_show_filing_modal.set(false))
                footer=modal_footer()
            >
                <div class="form-container" style="padding: 20px;">
                    <div style="margin-bottom: 15px;">
                        <label style="display: block; margin-bottom: 5px;">"是否串并案"</label>
                        <select 
                            style="width: 100%; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px;"
                            on:change=move |e| set_case_repeat_flag.set(event_target_value(&e).parse().unwrap_or(0))
                        >
                            <option value="0">"否"</option>
                            <option value="1">"是"</option>
                        </select>
                    </div>

                    <div style="margin-bottom: 15px;">
                        <label style="display: block; margin-bottom: 5px;">"处理方式"</label>
                        <select 
                            style="width: 100%; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px;"
                            on:change=move |e| set_punish_method.set(event_target_value(&e).parse().unwrap_or(2))
                        >
                            <option value="2">"立案调查"</option>
                            <option value="1">"协议处罚"</option>
                        </select>
                    </div>

                    <div>
                        <label style="display: block; margin-bottom: 5px;">"审核意见"</label>
                        <textarea 
                            style="width: 100%; height: 80px; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px; resize: none;"
                            placeholder="请输入审核意见"
                            on:input=move |e| set_checked_reason.set(event_target_value(&e))
                        ></textarea>
                    </div>
                </div>
            </Modal>
        </div>
    }
}
