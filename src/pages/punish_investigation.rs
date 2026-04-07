use leptos::*;
use leptos_router::use_navigate;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::components::template_editor::TemplateEditor;
use crate::api::models::{InspectionTask, InspectionTasksNoteAttrValVO, WrapperResponse, InspectionTasksNotePunishSubmitReq};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishInvestigationPage() -> impl IntoView {
    let (show_investigate_dialog, set_show_investigate_dialog) = create_signal(false);
    let (show_doc_editor, set_show_doc_editor) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    
    // 调查登记表单状态
    let (compulsory_measure, set_compulsory_measure) = create_signal(2); // 1: 是, 2: 否
    let (reqs_data, set_reqs_data) = create_signal(Vec::<InspectionTasksNoteAttrValVO>::new());

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::Investigation as i32),
                    ..Default::default()
                },
                page_num: 1,
                page_size: 10,
            };
            crate::api::client::post::<_, crate::api::models::WrapperResponse<crate::api::models::PageResult<InspectionTask>>>("/taskpunish/page", &req)
                .await
                .map(|resp| resp.data.map(|d| d.data).unwrap_or_default())
                .unwrap_or_default()
        }
    );

    let cases = move || task_resource.get().unwrap_or_default();

    let handle_save = move |_| {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = crate::api::models::InsInquiredReq {
            inspection_id: task_id as i32,
            template_id: 2, 
            compulsory_measure: compulsory_measure.get(),
            reqs: reqs_data.get(),
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting Investigation Inquire: {:?}", req);
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/inquire", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("调查任务已成功提交！");
                    set_show_investigate_dialog.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("提交失败: {}", e));
                }
            }
        });
    };

    let handle_stop = move || {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = crate::api::models::InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 2,
            reqs: vec![],
        };
        spawn_local(async move {
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/stopinquiry", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("调查已中止");
                    set_show_investigate_dialog.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("中止失败: {}", e));
                }
            }
        });
    };

    let handle_coerce = move || {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = crate::api::models::InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 2,
            reqs: reqs_data.get(),
        };
        spawn_local(async move {
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/inquire/compulsory/measure", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("强制措施已登记");
                    set_show_investigate_dialog.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("登记失败: {}", e));
                }
            }
        });
    };

    let handle_result_register = move || {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = crate::api::models::InsResultUploadReq {
            inspection_id: task_id as i32,
            template_id: 2,
            reqs: reqs_data.get(),
            legal_audit: 1, 
            transfor: 0,
            notice: 1,
        };
        spawn_local(async move {
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/result/registered", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("调查结果已成功登记！");
                    set_show_investigate_dialog.set(false);
                    let navigate = use_navigate();
                    navigate("/punish-legal", Default::default());
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("登记失败: {}", e));
                }
            }
        });
    };

    let columns = vec![
        TableColumn::new("处罚编号".to_string(), |t: InspectionTask| t.inspection_no.clone().unwrap_or_default()),
        TableColumn::new("稽查名称".to_string(), |t: InspectionTask| t.inspection_name.clone().unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let t_clone = t.clone();
            view! {
                <button class="btn-link" on:click=move |_| {
                    set_selected_task.set(Some(t_clone.clone()));
                    set_show_investigate_dialog.set(true);
                }>"办理详情"</button>
            }.into_view()
        }),
    ];

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
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            // 立案调查主对话框
            <Modal 
                show=show_investigate_dialog 
                on_close=Callback::new(move |_| set_show_investigate_dialog.set(false)) 
                title="立案调查登记".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_investigate_dialog.set(false)>"取消"</button>
                    <button class="btn btn-warning" on:click=move |_| handle_stop()>"中止调查"</button>
                    <button class="btn btn-success" on:click=move |_| handle_result_register()>"结果登记"</button>
                    <button class="btn btn-danger" on:click=move |_| handle_coerce()>"强制措施"</button>
                    <button class="btn btn-secondary" on:click=move |_| set_show_doc_editor.set(true)>"填写文书"</button>
                    <button class="btn btn-primary" on:click=handle_save>"确认提交"</button>
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
