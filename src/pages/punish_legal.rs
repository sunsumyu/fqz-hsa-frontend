use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, InspectionTasksNotePunishSubmitReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishLegalPage() -> impl IntoView {
    let (show_audit_modal, set_show_audit_modal) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    // 审核结果状态
    let (audit_conclusion, set_audit_conclusion) = create_signal(1); // 1: 通过, 2: 退回, 3: 不予处罚
    let (audit_opinion, set_audit_opinion) = create_signal(String::new());
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());
    
    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::LegalReview as i32),
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
        TableColumn::new("申请项目".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("指派时间".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_audit_modal.set(true);
                    }>
                        "法制审核"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
        }),
    ];

    let handle_submit = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        
        // Inject standalone UI fields into reqs
        let mut reqs = reqs_data.get();
        reqs.push(InspectionTasksNoteAttrValVO {
            id: None,
            inspection_id: Some(task_id as i32),
            template_id: Some(2),
            field_name: Some("审核结论".to_string()),
            field_attr: Some("audit_conclusion".to_string()),
            field_value: Some(audit_conclusion.get().to_string()),
            field_type: Some(1),
            field_class: None,
            required: Some(false),
        });
        reqs.push(InspectionTasksNoteAttrValVO {
            id: None,
            inspection_id: Some(task_id as i32),
            template_id: Some(2),
            field_name: Some("审核意见".to_string()),
            field_attr: Some("audit_opinion".to_string()),
            field_value: Some(audit_opinion.get()),
            field_type: Some(1),
            field_class: None,
            required: Some(false),
        });

        let req = crate::api::models::InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 2, 
            reqs,
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting Legal Review: {:?}", req);
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/legal/review", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("法制审核已提交成功！");
                    set_show_audit_modal.set(false);
                    let navigate = use_navigate();
                    if audit_conclusion.get() == 1 {
                        navigate("/punish-notice", Default::default());
                    } else {
                        task_resource.refetch();
                    }
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
                    <span>"首页"</span> " / " <span>"立案管理"</span> " / " <span class="active">"法制审核"</span>
                </div>
                <h2>"处罚案件法制审核"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            // 法制审核登记对话框
            <Modal 
                show=show_audit_modal 
                on_close=Callback::new(move |_| set_show_audit_modal.set(false)) 
                title="法制审核意见登记".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_audit_modal.set(false)>"关闭"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"填写审核表"</button>
                    <button class="btn btn-primary" on:click=handle_submit>"提交结果"</button>
                }.into_view()
            >
                <div class="legal-audit-form h-fidelity-form">
                    <div class="form-item mb-3">
                        <label class="required">"审核结论:"</label>
                        <select 
                            class="rich-input"
                            on:change=move |e| set_audit_conclusion.set(event_target_value(&e).parse().unwrap_or(1))
                        >
                            <option value="1">"审核通过 (进入告知环节)"</option>
                            <option value="2">"退回补正 (返回调查环节)"</option>
                            <option value="3">"不予处罚 (直接结案)"</option>
                        </select>
                    </div>
                    <div class="form-item mb-3">
                        <label>"审核意见:"</label>
                        <textarea 
                            class="rich-textarea" 
                            style="height: 100px;"
                            placeholder="请说明审核意见..."
                            on:input=move |e| set_audit_opinion.set(event_target_value(&e))
                        ></textarea>
                    </div>
                    {move || if !reqs_data.get().is_empty() {
                        view! {
                            <div class="status-badge success">
                                <i class="el-icon-circle-check"></i> " 已关联审核审批文书"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status-badge warning">
                                <i class="el-icon-warning-outline"></i> " 尚未填写文书，请点击「填写审核表」"
                            </div>
                        }.into_view()
                    }}
                </div>
            </Modal>

            // 文书填报覆盖层
            <Modal
                show=show_doc_editor
                on_close=Callback::new(move |_| set_show_doc_editor.set(false))
                title="法制审核 - 文书填写".to_string()
                width="95%".to_string()
            >
                <TemplateEditor 
                    inspection_id=selected_task.get().and_then(|t| t.id).unwrap_or(0) as i32
                    note_category="LEGAL_REVIEW".to_string() // 对应业务环节
                    on_save=Callback::new(move |data| {
                        set_reqs_data.set(data);
                        set_show_doc_editor.set(false);
                    })
                />
            </Modal>
        </div>
    }
}
