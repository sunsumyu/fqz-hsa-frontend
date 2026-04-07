use leptos::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, InspectionTasksNotePunishSubmitReq, WrapperResponse};

#[component]
pub fn PunishTransferPage() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i32>);

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
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

    let handle_transfer = move |_| {
        let task_id = selected_id.get().unwrap_or(0);
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id,
            template_id: 15, // Transfer template
            reqs: vec![],
        };
        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/transfer", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("案件已成功提交移送处理！");
                    set_show_modal.set(false);
                    task_resource.refetch();
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("移送失败: {}", e));
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
                }>"移送处理"</button>
            }.into_view()
        }),
    ];

    view! {
        <div class="page-container">
            <header class="page-header">
                <h2>"涉嫌犯罪案件移送管理"</h2>
            </header>
            <DataTable data=cases() columns=columns />
            <Modal show=show_modal on_close=Callback::new(move |_| set_show_modal.set(false)) title="案件移送审批".to_string()>
                <div class="transfer-form">
                    <p>"确认将该案件移送至司法机关处理？"</p>
                    <button class="btn btn-danger" on:click=handle_transfer>"确认移送"</button>
                </div>
            </Modal>
        </div>
    }
}
