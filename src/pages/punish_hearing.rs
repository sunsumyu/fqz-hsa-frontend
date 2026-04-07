use leptos::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::{InspectionTask, InspectionTasksNotePunishSubmitReq, WrapperResponse};
use crate::api::constants::PunishStatus;

#[component]
pub fn PunishHearingPage() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i32>);

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(PunishStatus::Hearing as i32),
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

    let handle_hearing_finish = move |_| {
        let task_id = selected_id.get().unwrap_or(0);
        let req = InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id,
            template_id: 8, // Hearing Record template
            reqs: vec![],
        };
        spawn_local(async move {
            match crate::api::client::post::<_, WrapperResponse<bool>>("/taskpunish/hearing", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("听证程序已处理完成！");
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
        TableColumn::new("稽查名称".to_string(), |t: InspectionTask| t.inspection_name.clone().unwrap_or_default()),
        TableColumn::new("听证状态".to_string(), |_: InspectionTask| "待处理听证".to_string()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let id = t.id.unwrap_or(0);
            view! {
                <button class="btn-link" on:click=move |_| {
                    set_selected_id.set(Some(id));
                    set_show_modal.set(true);
                }>"听证记录"</button>
            }.into_view()
        }),
    ];

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"首页"</span> " / " <span>"权利保障"</span> " / " <span class="active">"听证程序"</span>
                </div>
                <h2>"行政处罚听证程序处理"</h2>
            </header>

            <div class="view-container">
                <DataTable data=cases() columns=columns />
            </div>

            <Modal show=show_modal on_close=Callback::new(move |_| set_show_modal.set(false)) title="听证记录登记".to_string()>
                <div class="hearing-form h-fidelity-form">
                    <p>"在此处登记听证笔录并确认为后续处罚决定提供依据。"</p>
                    <button class="btn btn-primary" on:click=handle_hearing_finish>"完成听证"</button>
                </div>
            </Modal>
        </div>
    }
}
