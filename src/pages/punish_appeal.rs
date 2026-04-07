use leptos::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, InsAppealReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishAppealPage() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i32>);

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::Decision as i32), // After decision
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

    let handle_appeal = move |_| {
        let task_id = selected_id.get().unwrap_or(0);
        let req = InsAppealReq {
            inspection_id: task_id,
            template_id: 12,
            reqs: vec![],
            audit_conclusion: 1,
            audit_opinion: "维持原判".to_string(),
        };
        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/appeal", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("复议记录已提交！");
                    set_show_modal.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("提交失败: {}", e));
                }
            }
        });
    };

    let columns = vec![
        TableColumn::new("处罚编号".to_string(), |t: InspectionTask| t.inspection_no.clone().unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let id = t.id.unwrap_or(0);
            view! {
                <button class="btn-link" on:click=move |_| {
                    set_selected_id.set(Some(id));
                    set_show_modal.set(true);
                }>"复议登记"</button>
            }.into_view()
        }),
    ];

    view! {
        <div class="page-container">
            <header class="page-header">
                <h2>"行政复议与诉讼管理"</h2>
            </header>
            <DataTable data=cases() columns=columns />
            <Modal show=show_modal on_close=Callback::new(move |_| set_show_modal.set(false)) title="复议登记".to_string()>
                <div class="appeal-form">
                    <button class="btn btn-primary" on:click=handle_appeal>"提交复议结果"</button>
                </div>
            </Modal>
        </div>
    }
}
