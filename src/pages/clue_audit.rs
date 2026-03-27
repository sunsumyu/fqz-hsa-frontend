use leptos::*;
use leptos_router::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::modal::Modal;
use crate::api::models::InspectionTask;
use crate::api::constants::PunishStatus;

#[component]
pub fn ClueAuditPage() -> impl IntoView {
    let (show_filing_modal, set_show_filing_modal) = create_signal(false);
    let (selected_id, set_selected_id) = create_signal(None::<i64>);
    
    // Popup form state
    let (_is_merge, set_is_merge) = create_signal("no".to_string());
    let (case_search, _set_case_search) = create_signal("".to_string());
    let (_event_id, set_event_id) = create_signal("".to_string());
    let (_audit_opinion, set_audit_opinion) = create_signal("".to_string());

    let cases = vec![
        InspectionTask {
            id: Some(1),
            task_id: Some(6),
            main_task_code: Some("659096748706234368".to_string()),
            audit_no: Some("GZ20240722255".to_string()),
            inspection_name: Some("二九一医院(稽核)".to_string()),
            inspection_status: Some(PunishStatus::WaitPreAudit as i32),
            assign_time: Some("2024-07-22".to_string()),
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
                    <button class="btn btn-sm">"协议处罚"</button>
                    <button class="btn btn-primary btn-sm" on:click=move |_| {
                        set_selected_id.set(Some(id as i64));
                        set_show_filing_modal.set(true);
                    }>
                        "立案调查"
                    </button>
                </div>
            }
        }),
    ];

    let modal_footer = move || view! {
        <button class="btn" on:click=move |_| set_show_filing_modal.set(false)>"取消"</button>
        <A href=format!("/document-edit/{}", selected_id.get().unwrap_or(0)) class="btn">"填写文书"</A>
        <button class="btn btn-primary" on:click=move |_| set_show_filing_modal.set(false)>"保存"</button>
    }.into_view();

    view! {
        <div class="page-container">
            <header class="page-header">
                <div class="breadcrumb">
                    <span>"Dashboard"</span> " / " <span>"任务稽查"</span> " / " <span class="active">"预检中"</span>
                </div>
                <h2>"任务预检 / 稽核处理"</h2>
            </header>

            <div class="view-container">
                <div class="filter-bar">
                    <div class="filter-item">
                        <label>"请输入稽查编码:"</label>
                        <input type="text" placeholder="稽查编码" />
                    </div>
                    <div class="filter-item">
                        <label>"请输入稽查标题:"</label>
                        <input type="text" placeholder="稽查标题" />
                    </div>
                    <button class="btn btn-primary">"查询"</button>
                    <button class="btn">"重置"</button>
                </div>

                <div class="data-table-wrapper">
                    <DataTable data=cases columns=columns />
                </div>
            </div>

            <Modal 
                show=show_filing_modal 
                on_close=Callback::new(move |_| set_show_filing_modal.set(false)) 
                title="立案调查".to_string()
                footer=modal_footer()
            >
                <div class="filing-popup-form" style="padding: 10px;">
                    <div style="display: flex; gap: 20px; margin-bottom: 15px;">
                        <div style="flex: 1;">
                            <label style="display: block; margin-bottom: 5px;">"是否串并案"</label>
                            <select 
                                style="width: 100%; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px;"
                                on:change=move |e| set_is_merge.set(event_target_value(&e))
                            >
                                <option value="no">"否"</option>
                                <option value="yes">"是"</option>
                            </select>
                        </div>
                        <div style="flex: 1;">
                            <label style="display: block; margin-bottom: 5px;">"案件"</label>
                            <div style="display: flex;">
                                <input 
                                    type="text" 
                                    placeholder="请选择或搜索案件" 
                                    style="flex: 1; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px 0 0 4px;"
                                    prop:value=case_search
                                />
                                <button style="padding: 0 12px; background: #f5f7fa; border: 1px solid #dcdfe6; border-left: none; border-radius: 0 4px 4px 0;">
                                    <i class="el-icon-search"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    <div style="margin-bottom: 15px;">
                        <label style="display: block; margin-bottom: 5px;">"事件"</label>
                        <select 
                            style="width: 100%; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px;"
                            on:change=move |e| set_event_id.set(event_target_value(&e))
                        >
                            <option value="">"请选择事件"</option>
                            <option value="1">"违规结算事件 A"</option>
                            <option value="2">"串换项目事件 B"</option>
                        </select>
                    </div>

                    <div>
                        <label style="display: block; margin-bottom: 5px;">"审核意见"</label>
                        <textarea 
                            style="width: 100%; height: 80px; padding: 8px; border: 1px solid #dcdfe6; border-radius: 4px; resize: none;"
                            placeholder="请输入审核意见"
                            on:input=move |e| set_audit_opinion.set(event_target_value(&e))
                        ></textarea>
                    </div>
                </div>
            </Modal>
        </div>
    }
}
