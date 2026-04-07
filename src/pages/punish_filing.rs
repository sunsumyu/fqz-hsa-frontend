use leptos::*;
use leptos_router::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::InspectionTask;

#[component]
pub fn PunishFilingPage() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i32>);
    let (force_measure, set_force_measure) = create_signal(false);

    let task_resource = create_resource(
        || (),
        |_| async move {
            let req = crate::api::models::PageVO {
                condition: crate::api::models::InspectionTasksReq {
                    inspection_status: Some(1000), // FILING
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


    let handle_submit = move |_| {
        let task_id = selected_id.get().unwrap_or(0);
        let navigate = use_navigate();
        
        // Inject force measure into reqs
        let mut reqs = vec![];
        reqs.push(crate::api::models::InspectionTasksNoteAttrValVO {
            id: None,
            inspection_id: Some(task_id as i32),
            template_id: Some(1),
            field_name: Some("是否采取行政强制措施".to_string()),
            field_attr: Some("compulsoryMeasure".to_string()),
            field_value: Some(if force_measure.get() { "1".to_string() } else { "0".to_string() }),
            field_type: Some(1),
            field_class: None,
            required: Some(false),
        });

        let req = crate::api::models::InspectionTasksNotePunishSubmitSubReq {
            inspection_id: task_id as i32,
            template_id: 1, // Filing template
            legal_audit: 1, // Default to legal audit
            reqs,
        };

        spawn_local(async move {
            leptos::logging::log!("Submitting Filing Pass: {:?}", req);
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/pass", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("立案审批已成功提交！");
                    set_show_modal.set(false);
                    navigate("/punish-investigation", Default::default());
                }
                Err(e) => {
                    let _ = window().alert_with_message(&format!("提交失败: {}", e));
                }
            }
        });
    };

    let handle_reject = move || {
        let task_id = selected_id.get().unwrap_or(0);
        let req = crate::api::models::InspectionTasksNotePunishSubmitReq {
            inspection_id: task_id as i32,
            template_id: 1, 
            reqs: vec![],   
        };

        spawn_local(async move {
            match crate::api::client::post::<_, crate::api::models::WrapperResponse<bool>>("/taskpunish/nopass", &req).await {
                Ok(_) => {
                    let _ = window().alert_with_message("不予立案审批已成功提交！");
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
        TableColumn::new("案源类别".to_string(), |t: InspectionTask| t.source.clone().unwrap_or_default()),
        TableColumn::new("涉案金额".to_string(), |_: InspectionTask| format!("{:.2}", 0.0)),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let id = t.id.unwrap_or(0);
            view! {
                <button class="btn-link" on:click=move |_| {
                    set_selected_id.set(Some(id));
                    set_show_modal.set(true);
                }>"审批立案"</button>
            }.into_view()
        }),
    ];

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"Dashboard"</span> " / " <span>"立案管理"</span> " / " <span class="active">"待立案"</span>
                </div>
                <h2>"处罚案件立案审批"</h2>
            </header>

            <div class="view-container">
                <div class="filter-bar">
                    <div class="filter-item">
                        <label>"处罚编号:"</label>
                        <input type="text" placeholder="输入编号" />
                    </div>
                    <div class="filter-item">
                        <label>"对象名称:"</label>
                        <input type="text" placeholder="输入单位" />
                    </div>
                    <button class="btn btn-primary">"查询"</button>
                </div>

                <div class="data-table-wrapper">
                    {move || match task_resource.get() {
                        Some(data) => view! { 
                            <DataTable data=data columns=columns.clone() /> 
                        }.into_view(),
                        None => view! { <div class="loading">"加载中..."</div> }.into_view()
                    }}
                </div>
            </div>

            <Modal 
                show=show_modal 
                on_close=Callback::new(move |_| set_show_modal.set(false)) 
                title="立案调查审批".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_modal.set(false)>"取消"</button>
                    <button class="btn btn-danger" on:click=move |_| handle_reject()>"不予立案"</button>
                    <button class="btn btn-primary" on:click=handle_submit>"确认立案"</button>
                }.into_view()
            >
                <div class="punish-filing-form">
                    <div class="form-group" style="display: flex; align-items: center; gap: 20px;">
                        <label style="margin-bottom: 0;"><span style="color: red;">"*"</span> " 是否采取行政强制措施: "</label>
                        <div class="radio-group">
                            <label class="radio-item">
                                <input type="radio" name="force_measure" value="false" checked=move || !force_measure.get() 
                                    on:change=move |_| set_force_measure.set(false) />
                                "否"
                            </label>
                            <label class="radio-item">
                                <input type="radio" name="force_measure" value="true" checked=move || force_measure.get() 
                                    on:change=move |_| set_force_measure.set(true) />
                                "是"
                            </label>
                        </div>
                    </div>
                </div>
            </Modal>
        </div>
    }
}
