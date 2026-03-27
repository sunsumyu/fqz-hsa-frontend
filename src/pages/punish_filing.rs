use leptos::*;
use leptos_router::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::InspectionTask;

#[component]
pub fn PunishFilingPage() -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i64>);
    let (force_measure, set_force_measure) = create_signal(false);

    let cases = vec![
        InspectionTask {
            id: Some(1),
            task_id: Some(9),
            main_task_code: Some("659096748706234368".to_string()),
            audit_no: Some("GZ20240723380".to_string()),
            inspection_name: Some("测试国家局申报-子任务-湖南省".to_string()),
            inspection_status: Some(1000), // FILING
            assign_time: Some("2024-07-23".to_string()),
            expire_time: Some("2024-07-26".to_string()),
            ..Default::default()
        },
    ];

    let columns = vec![
        TableColumn::new("序号".to_string(), |t: InspectionTask| t.id.unwrap_or(0).to_string()),
        TableColumn::new("任务ID".to_string(), |t: InspectionTask| t.task_id.unwrap_or(0).to_string()),
        TableColumn::new("主任务编码".to_string(), |t: InspectionTask| t.main_task_code.unwrap_or_default()),
        TableColumn::new("稽查编码".to_string(), |t: InspectionTask| t.audit_no.unwrap_or_default()),
        TableColumn::new("稽查标题".to_string(), |t: InspectionTask| t.inspection_name.unwrap_or_default()),
        TableColumn::new("指派时间".to_string(), |t: InspectionTask| t.assign_time.unwrap_or_default()),
        TableColumn::new("逾期时间".to_string(), |t: InspectionTask| t.expire_time.unwrap_or_default()),
        TableColumn::new("操作".to_string(), move |t: InspectionTask| {
            let id = t.id.unwrap_or(0);
            view! {
                <div class="table-actions">
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_id.set(Some(id as i64));
                        set_show_modal.set(true);
                    }>
                        <i class="el-icon-edit"></i> " 立案调查"
                    </button>
                    <button class="btn btn-sm">"详情"</button>
                </div>
            }
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
                        <input type="text" placeholder="输入单位或个人" />
                    </div>
                    <button class="btn btn-primary">"查询"</button>
                </div>

                <div class="data-table-wrapper">
                    <DataTable data=cases columns=columns />
                </div>
            </div>

            <Modal 
                show=show_modal 
                on_close=Callback::new(move |_| set_show_modal.set(false)) 
                title="立案调查".to_string()
                footer=view! {
                    <button class="btn" on:click=move |_| set_show_modal.set(false)>"取消"</button>
                    <A href=format!("/document-edit/{}", selected_id.get().unwrap_or(0)) class="btn btn-primary">"填写文书"</A>
                    <button class="btn btn-primary" on:click=move |_| set_show_modal.set(false)>"保存"</button>
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
                    <div style="margin-top: 20px; padding: 12px; background: #fff7e6; border: 1px solid #ffd591; border-radius: 4px; color: #fa8c16; font-size: 13px;">
                        <i class="el-icon-warning"></i>
                        " 说明：开启强制措施后，在调查阶段将激活【强制执行】文书填报环节。"
                    </div>
                </div>
            </Modal>
        </div>
    }
}
