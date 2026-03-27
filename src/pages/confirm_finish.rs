use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, TaskPunishCloseReq};
use crate::api::constants::PunishStatus;

#[component]
pub fn ConfirmFinishPage() -> impl IntoView {
    let (show_close_modal, set_show_close_modal) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    let (close_remark, set_close_remark) = create_signal(String::new());

    let tasks = vec![
        InspectionTask {
            id: Some(255),
            audit_no: Some("GZ20240722255".to_string()),
            inspection_name: Some("XX市第一人民医院行政处罚案".to_string()),
            inspection_status: Some(PunishStatus::Closed as i32),
            assign_time: Some("2024-07-22".to_string()),
            ..Default::default()
        },
    ];

    let columns = vec![
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.audit_no.unwrap_or_default()),
        TableColumn::new("任务名称".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_close_modal.set(true);
                    }>
                        "结案确认"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
        }),
    ];

    let handle_close_confirm = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = TaskPunishCloseReq {
            inspection_id: task_id as i32,
            close_remark: close_remark.get(),
        };
        leptos::logging::log!("Submitting TaskPunishCloseReq: {:?}", req);
        let _ = window().alert_with_message("案件已正式办结并归档！");
        set_show_close_modal.set(false);
        let navigate = use_navigate();
        navigate("/ledger", Default::default());
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"首页"</span> " / " <span>"行政处罚"</span> " / " <span class="active">"办结归档"</span>
                </div>
                <h2>"案件办结归档确认"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=tasks columns=columns />
                </div>
            </div>

            <Modal 
                show=show_close_modal 
                on_close=Callback::new(move |_| set_show_close_modal.set(false)) 
                title="结案归档确认".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_close_modal.set(false)>"取消"</button>
                    <button class="btn btn-primary" on:click=handle_close_confirm>"确认结案"</button>
                }.into_view()
            >
                <div class="close-confirm-form h-fidelity-form">
                    <div class="info-alert mb-4">
                        <i class="el-icon-warning"></i>
                        " 注意：结案确认后，该案件将进入历史库，不再支持修改相关文书及执行状态。"
                    </div>
                    <div class="form-item">
                        <label>"办结备注 (选填):"</label>
                        <textarea 
                            class="rich-textarea" 
                            style="height: 100px;" 
                            placeholder="输入办结意见或归档说明..."
                            on:input=move |e| set_close_remark.set(event_target_value(&e))
                        ></textarea>
                    </div>
                </div>
            </Modal>
        </div>
    }
}
