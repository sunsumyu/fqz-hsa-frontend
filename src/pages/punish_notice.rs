use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, InspectionTasksNotePunishSubmitReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishNoticePage() -> impl IntoView {
    let (show_notice_modal, set_show_notice_modal) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());
    
    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::WaitNotification as i32),
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
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.audit_no.unwrap_or_default()),
        TableColumn::new("被告知人/单位".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("指派时间".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_notice_modal.set(true);
                    }>
                        "告知确认"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
        }),
    ];

    let handle_submit = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 3, // Assume template_id 3 for notice
            reqs: reqs_data.get(),
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting InspectionTasksNotePunishSubmitReq: {:?}", req);
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/notice", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("处罚告知已提交完成！");
                    set_show_notice_modal.set(false);
                    let navigate = use_navigate();
                    navigate("/punish-decision", Default::default());
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
                    <span>"首页"</span> " / " <span>"立案管理"</span> " / " <span class="active">"待告知"</span>
                </div>
                <h2>"待下达行政处罚告知"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            <Modal 
                show=show_notice_modal 
                on_close=Callback::new(move |_| set_show_notice_modal.set(false)) 
                title="处罚前告知确认".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_notice_modal.set(false)>"取消"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"编辑告知书"</button>
                    <button class="btn btn-primary" on:click=handle_submit>"确认告知"</button>
                }.into_view()
            >
                <div class="notice-confirm-form h-fidelity-form">
                    <div class="info-alert mb-4">
                        <i class="el-icon-info"></i> " 系统将根据调查认定的违法事实自动生成《行政处罚事先告知书》。"
                    </div>
                    {move || if !reqs_data.get().is_empty() {
                        view! {
                            <div class="status-badge success">
                                <i class="el-icon-circle-check"></i> " 已预览并核对告知书文书属性"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status-badge warning">
                                <i class="el-icon-warning-outline"></i> " 建议点击「编辑告知书」进行最后确认"
                            </div>
                        }.into_view()
                    }}
                </div>
            </Modal>

            <Modal
                show=show_doc_editor
                on_close=Callback::new(move |_| set_show_doc_editor.set(false))
                title="事先告知 - 文书确认".to_string()
                width="95%".to_string()
            >
                <TemplateEditor 
                    inspection_id=selected_task.get().and_then(|t| t.id).unwrap_or(0) as i32
                    note_category="PUNISH_NOTICE".to_string()
                    on_save=Callback::new(move |data| {
                        set_reqs_data.set(data);
                        set_show_doc_editor.set(false);
                    })
                />
            </Modal>
        </div>
    }
}
