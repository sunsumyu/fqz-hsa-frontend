use leptos::*;
use leptos_router::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, TaskPunishInquireReq};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishInvestigationPage() -> impl IntoView {
    let (show_investigate_dialog, set_show_investigate_dialog) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    // 调查登记表单状态
    let (compulsory_measure, set_compulsory_measure) = create_signal(2); // 1: 是, 2: 否
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());

    let cases = vec![
        InspectionTask {
            id: Some(255),
            audit_no: Some("GZ20240722255".to_string()),
            inspection_name: Some("测试立案调查任务 - 湖南省人民医院".to_string()),
            inspection_status: Some(PunishStatus::Investigation as i32), // 使用正确的枚举项
            assign_time: Some("2024-07-22".to_string()),
            ..Default::default()
        },
    ];

    let columns = vec![
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.audit_no.unwrap_or_default()),
        TableColumn::new("稽查标题".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("指派时间".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_investigate_dialog.set(true);
                    }>
                        "调查"
                    </button>
                </div>
            }
        }),
    ];

    let handle_save = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = TaskPunishInquireReq {
            inspection_id: task_id as i32,
            compulsory_measure: Some(compulsory_measure.get()),
            reqs: reqs_data.get(),
        };
        leptos::logging::log!("Submitting TaskPunishInquireReq: {:?}", req);
        let _ = window().alert_with_message("立案调查结果已成功保存并提交！");
        set_show_investigate_dialog.set(false);
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"首页"</span> " / " <span>"行政处罚"</span> " / " <span class="active">"调查中"</span>
                </div>
                <h2>"立案调查管理"</h2>
            </header>

            <div class="view-container">
                <div class="data-table-wrapper">
                    <DataTable data=cases columns=columns />
                </div>
            </div>

            // 立案调查主对话框
            <Modal 
                show=show_investigate_dialog 
                on_close=Callback::new(move |_| set_show_investigate_dialog.set(false)) 
                title="立案调查登记".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_investigate_dialog.set(false)>"取消"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"填写文书"</button>
                    <button class="btn btn-primary" on:click=handle_save>"确认保存"</button>
                }.into_view()
            >
                <div class="investigation-dialog-content">
                    <div class="form-group mb-4">
                        <label class="block text-sm font-medium mb-2">
                            <span class="text-red-500">"*"</span> " 是否采取行政强制措施: "
                        </label>
                        <div class="flex gap-4">
                            <label class="flex items-center gap-2 cursor-pointer">
                                <input 
                                    type="radio" 
                                    name="measure" 
                                    checked=move || compulsory_measure.get() == 1
                                    on:change=move |_| set_compulsory_measure.set(1)
                                /> "是"
                            </label>
                            <label class="flex items-center gap-2 cursor-pointer">
                                <input 
                                    type="radio" 
                                    name="measure" 
                                    checked=move || compulsory_measure.get() == 2
                                    on:change=move |_| set_compulsory_measure.set(2)
                                /> "否"
                            </label>
                        </div>
                    </div>
                    {move || if !reqs_data.get().is_empty() {
                        view! {
                            <div class="status-badge success">
                                <i class="el-icon-circle-check"></i> " 已完成文书填报 (" {reqs_data.get().len()} " 个属性)"
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="status-badge warning">
                                <i class="el-icon-warning-outline"></i> " 尚未填写文书，请点击下方【填写文书】"
                            </div>
                        }.into_view()
                    }}
                </div>
            </Modal>

            // 文书填报覆盖层 (大窗口)
            <Modal
                show=show_doc_editor
                on_close=Callback::new(move |_| set_show_doc_editor.set(false))
                title="立案调查 - 文书填写".to_string()
                width="95%".to_string()
            >
                <TemplateEditor 
                    inspection_id=selected_task.get().and_then(|t| t.id).unwrap_or(0) as i32
                    note_category="FILINGCASE_TO_INQUIRING".to_string()
                    on_save=Callback::new(move |data| {
                        set_reqs_data.set(data);
                        set_show_doc_editor.set(false);
                    })
                />
            </Modal>
        </div>
    }
}
