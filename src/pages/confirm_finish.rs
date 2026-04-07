use leptos::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, InspectionTasksNotePunishSubmitReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn ConfirmFinishPage() -> impl IntoView {
    let (show_close_modal, set_show_close_modal) = create_signal(false);
    let (selected_task, set_selected_task) = create_signal(None::<InspectionTask>);
    let (close_remark, set_close_remark) = create_signal(String::new());

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::WaitClose as i32), // 假设 1700
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
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.inspection_no.clone().unwrap_or_default()),
        TableColumn::new("任务名称".to_string(), |t: InspectionTask| t.inspection_name.clone().unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let task = t.clone();
            view! {
                <div class="table-actions">
                    <button class="btn-link" on:click=move |_| {
                        set_selected_task.set(Some(task.clone()));
                        set_show_close_modal.set(true);
                    }>
                        "结案确认"
                    </button>
                </div>
            }.into_view()
        }),
    ];

    let handle_feedback = move || {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 6,
            reqs: vec![],
        };

        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/case/feedback", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("结案反馈已提交！");
                    set_show_close_modal.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("提交失败: {}", e));
                }
            }
        });
    };

    let handle_endcase = move || {
        let task_id = selected_task.get().and_then(|t| t.id).unwrap_or(0);
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 6,
            reqs: vec![],
        };

        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/endcase", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("案件已正式终结！");
                    set_show_close_modal.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("结案失败: {}", e));
                }
            }
        });
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
                    <DataTable data=cases() columns=columns />
                </div>
            </div>

            <Modal 
                show=show_close_modal 
                on_close=Callback::new(move |_| set_show_close_modal.set(false)) 
                title="结案归档确认".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_close_modal.set(false)>"取消"</button>
                    <button class="btn btn-warning" on:click=move |_| handle_endcase()>"正式结案"</button>
                    <button class="btn btn-primary" on:click=move |_| handle_feedback()>"确认反馈"</button>
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
                            prop:value=close_remark
                        ></textarea>
                    </div>
                </div>
            </Modal>
        </div>
    }
}
