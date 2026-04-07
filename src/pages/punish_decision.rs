use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, InsRecheckReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishDecisionPage() -> impl IntoView {
    let (show_decision_modal, set_show_decision_modal) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());
    
    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::Decision as i32),
                    ..Default::default()
                },
                page_num: 1,
                page_size: 10,
            };
            crate::api::client::post::<_, WrapperResponse<crate::api::models::PageResult<InspectionTask>>>("/taskpunish/page", &req)
                .await
                .map(|resp| resp.data.map(|d| d.data).unwrap_or_default())
                .unwrap_or_default()
        }
    );

    let cases = move || task_resource.get().unwrap_or_default();

    let columns = vec![
        TableColumn::new("处罚编号".to_string(), |t: InspectionTask| t.audit_no.unwrap_or_default()),
        TableColumn::new("被处罚对象".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("下达时间".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_decision_modal.set(true);
                    }>
                        "决定书确认"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
        }),
    ];

    let handle_submit = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = InsRecheckReq {
            inspection_id: task_id as i32,
            template_id: 4, // Assume template_id 4 for decision
            recheck: 0, 
            transfer: 0,
            reqs: reqs_data.get(),
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting InsRecheckReq: {:?}", req);
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/punish", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("处罚决定已提交完成！");
                    set_show_decision_modal.set(false);
                    let navigate = use_navigate();
                    navigate("/punish-execution", Default::default());
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("提交失败: {}", e));
                }
            }
        });
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"首页"</span> " / " <span>"行政处罚"</span> " / " <span class="active">"待决定"</span>
                </div>
                <h2>"行政处罚决定下达"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            <Modal 
                show=show_decision_modal 
                on_close=Callback::new(move |_| set_show_decision_modal.set(false)) 
                title="处罚决定确认".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_decision_modal.set(false)>"取消"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"填写决定书"</button>
                    <button class="btn btn-primary" on:click=handle_submit>"确认送达"</button>
                }.into_view()
            >
                <div class="decision-form h-fidelity-form">
                    <div class="info-alert mb-4">
                        <i class="el-icon-warning"></i> " 请确保《行政处罚决定书》中的处罚金额、种类与法制审核结论一致。"
                    </div>
                    {move || if !reqs_data.get().is_empty() {
                        view! {
                            <div class="status-badge success">
                                <i class="el-icon-circle-check"></i> " 《处罚决定书》数据已就绪"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status-badge warning">
                                <i class="el-icon-warning-outline"></i> " 尚缺决定书关键信息，请点击「填写决定书」"
                            </div>
                        }.into_view()
                    }}
                </div>
            </Modal>

            <Modal
                show=show_doc_editor
                on_close=Callback::new(move |_| set_show_doc_editor.set(false))
                title="处罚决定 - 文书下达".to_string()
                width="95%".to_string()
            >
                <TemplateEditor 
                    inspection_id=selected_task.get().and_then(|t| t.id).unwrap_or(0) as i32
                    note_category="PUNISH_DECISION".to_string()
                    on_save=Callback::new(move |data| {
                        set_reqs_data.set(data);
                        set_show_doc_editor.set(false);
                    })
                />
            </Modal>
        </div>
    }
}
