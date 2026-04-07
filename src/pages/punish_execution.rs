use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, InspectionTasksNotePunishSubmitReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishExecutionPage() -> impl IntoView {
    let (show_exec_modal, set_show_exec_modal) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    let (execution_status, set_execution_status) = create_signal(1); // 1: 已执行, 2: 强制执行中
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());
    
    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::Execution as i32),
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
        TableColumn::new("决定下达日期".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_exec_modal.set(true);
                    }>
                        "执行确认"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
        }),
    ];

    let handle_submit = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        
        let mut reqs = reqs_data.get();
        // Inject execution status
        reqs.push(InspectionTasksNoteAttrValVO {
            id: None,
            inspection_id: Some(task_id as i32),
            template_id: Some(5),
            field_name: Some("执行结果".to_string()),
            field_attr: Some("executionStatus".to_string()),
            field_value: Some(execution_status.get().to_string()),
            field_type: Some(1),
            field_class: None,
            required: Some(false),
        });
        
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 5, // Assume template_id 5 for execution
            reqs,
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting InspectionTasksNotePunishSubmitReq: {:?}", req);
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/exec", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("执行确认已提交完成！");
                    set_show_exec_modal.set(false);
                    task_resource.refetch();
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
                    <span>"首页"</span> " / " <span>"行政处罚"</span> " / " <span class="active">"执行中"</span>
                </div>
                <h2>"行政处罚执行情况跟踪"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            <Modal 
                show=show_exec_modal 
                on_close=Callback::new(move |_| set_show_exec_modal.set(false)) 
                title="处罚执行情况确认".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_exec_modal.set(false)>"取消"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"录入执行文书"</button>
                    <button class="btn btn-primary" on:click=handle_submit>"确认流水"</button>
                }.into_view()
            >
                <div class="execution-form h-fidelity-form">
                    <div class="form-item mb-4">
                        <label class="required">"执行结果:"</label>
                        <select 
                            class="rich-input"
                            on:change=move |e| set_execution_status.set(event_target_value(&e).parse().unwrap_or(1))
                        >
                            <option value="1">"已全额缴纳罚款 (正常执行完毕)"</option>
                            <option value="2">"拒不履行 (转入法院强制执行)"</option>
                        </select>
                    </div>
                    {move || if !reqs_data.get().is_empty() {
                        view! {
                            <div class="status-badge success">
                                <i class="el-icon-circle-check"></i> " 执行反馈文书/回执已录取"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status-badge warning">
                                <i class="el-icon-warning-outline"></i> " 请上传缴费回执或强制执行申请书"
                            </div>
                        }.into_view()
                    }}
                </div>
            </Modal>

            <Modal
                show=show_doc_editor
                on_close=Callback::new(move |_| set_show_doc_editor.set(false))
                title="执行阶段 - 文书录取".to_string()
                width="95%".to_string()
            >
                <TemplateEditor 
                    inspection_id=selected_task.get().and_then(|t| t.id).unwrap_or(0) as i32
                    note_category="PUNISH_EXECUTION".to_string()
                    on_save=Callback::new(move |data| {
                        set_reqs_data.set(data);
                        set_show_doc_editor.set(false);
                    })
                />
            </Modal>
        </div>
    }
}
