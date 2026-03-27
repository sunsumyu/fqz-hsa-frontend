use leptos::*;
use crate::components::data_table::{DataTable, TableColumn};
use crate::components::comparison_modal::ComparisonModal;

#[derive(Clone, Default, Debug)]
pub struct AuditQueryItem {
    pub id: i32,
    pub audit_no: String,
    pub target_name: String,
    pub settle_start: String,
    pub settle_end: String,
    pub violation_amount: f64,
    pub violation_count: i32,
    pub status: i32,
}

#[component]
pub fn AuditQueryPage() -> impl IntoView {
    let (show_comparison, set_show_comparison) = create_signal(false);
    let (selected_title, set_selected_title) = create_signal(String::new());

    let items = vec![
        AuditQueryItem {
            id: 1,
            audit_no: "FLOW-202403-099".into(),
            target_name: "林*中".into(),
            settle_start: "2024-03-01".into(),
            settle_end: "2024-03-05".into(),
            violation_amount: 1450.50,
            violation_count: 3,
            status: 110,
        },
        AuditQueryItem {
            id: 2,
            audit_no: "FLOW-202403-102".into(),
            target_name: "广州市中医院(珠江新城院区)".into(),
            settle_start: "2024-02-15".into(),
            settle_end: "2024-03-10".into(),
            violation_amount: 58240.22,
            violation_count: 142,
            status: 120,
        },
    ];

    let columns = vec![
        TableColumn::new("序号".into(), |i: AuditQueryItem| i.id.to_string()),
        TableColumn::new("稽查编码".into(), |i: AuditQueryItem| i.audit_no),
        TableColumn::new("被稽查对象".into(), |i: AuditQueryItem| i.target_name),
        TableColumn::new("违规金额".into(), |i: AuditQueryItem| format!("¥ {:.2}", i.violation_amount)),
        TableColumn::new("违规笔数".into(), |i: AuditQueryItem| i.violation_count.to_string()),
        TableColumn::new("状态".into(), |i: AuditQueryItem| {
            let (label, class) = match i.status {
                110 => ("稽查中", "status-process"),
                120 => ("待复核", "status-process"),
                170 => ("已办结", "status-success"),
                _ => ("其他", "status-init"),
            };
            view! { <span class=format!("status-badge {}", class)>{label}</span> }.into_view()
        }),
        TableColumn::new("图片对比".into(), move |i: AuditQueryItem| {
            let title = format!("影像比对分析 - {}", i.audit_no);
            view! {
                <button class="btn btn-primary btn-sm" on:click=move |_| {
                    set_selected_title.set(title.clone());
                    set_show_comparison.set(true);
                }>
                    <i class="el-icon-picture"></i> " 查看对比"
                </button>
            }
        }),
    ];

    view! {
        <div class="page-container">
            <header class="page-header">
                <h2>"稽查进度查询"</h2>
            </header>

            <div class="view-container">
                <div class="filter-bar">
                    <div class="filter-item">
                        <label>"流水号:"</label>
                        <input type="text" placeholder="请输入流水号" />
                    </div>
                    <div class="filter-item">
                        <label>"医院/姓名:"</label>
                        <input type="text" placeholder="请输入对象名称" />
                    </div>
                    <button class="btn btn-primary">"搜索"</button>
                </div>

                <div class="data-table-wrapper">
                    <DataTable data=items columns=columns />
                </div>
            </div>

            <ComparisonModal 
                show=show_comparison 
                set_show=set_show_comparison 
                title=selected_title.get() 
            />
        </div>
    }
}
