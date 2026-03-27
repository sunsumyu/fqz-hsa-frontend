use leptos::*;
use leptos_router::use_navigate;
use crate::api::models::{InspectionTasksNoteAttrListVO, InspectionTasksNoteAttrValVO};

#[component]
pub fn TemplateEditor(
    inspection_id: i32,
    note_category: String,
    #[prop(optional)] on_save: Option<Callback<Vec<InspectionTasksNoteAttrValVO>>>,
) -> impl IntoView {
    let _ = inspection_id;
    
    // In a real app, this would be a create_resource call
    let templates: Vec<InspectionTasksNoteAttrListVO> = serde_json::from_str(include_str!("../api/templates.json"))
        .unwrap_or_default();
    
    let (templates_sig, _) = create_signal(templates);
    let (cur_index, set_cur_index) = create_signal(0);
    let (form_save_data, set_form_save_data) = create_signal(std::collections::HashMap::<i32, Vec<InspectionTasksNoteAttrValVO>>::new());

    let current_template = move || templates_sig.with(|t| t.get(cur_index.get()).cloned());

    let handle_save_and_next = move |_| {
        if let Some(curr) = current_template() {
            let template_id = curr.template_id;
            // logic to collect current form data would go here
            // For now, we mock the collection
            let mock_vals = curr.task_attr_list.clone(); 
            set_form_save_data.update(|map| {
                map.insert(template_id, mock_vals);
            });

            if cur_index.get() < templates_sig.with(|t| t.len()) - 1 {
                set_cur_index.update(|i| *i += 1);
            } else {
                // Final submission
                if let Some(cb) = on_save {
                    let mut all_data = Vec::new();
                    form_save_data.with(|map| {
                        for vals in map.values() {
                            all_data.extend(vals.clone());
                        }
                    });
                    cb.call(all_data);
                } else {
                    let _ = window().alert_with_message("文书已成功保存！");
                    let navigate = use_navigate();
                    navigate("/ledger", Default::default());
                }
            }
        }
    };

    view! {
        <div class="tmpl-container">
            <div class="left">
                <div class="title">"环节对应文书"</div>
                <div class="note-category-tag">{format!("当前环节：{}", note_category)}</div>
                <ul class="template-list">
                    {move || templates_sig.with(|t| {
                        t.iter().enumerate().map(|(idx, tmpl)| {
                            let name = tmpl.field_name.clone().unwrap_or_default();
                            view! {
                                <li 
                                    class=move || if cur_index.get() == idx { "item selected" } else { "item" }
                                    on:click=move |_| set_cur_index.set(idx)
                                >
                                    <i class="el-icon-document"></i> {format!(" {}", name)}
                                </li>
                            }
                        }).collect_view()
                    })}
                </ul>
            </div>

            <div class="form-area">
                <header class="title">
                    <div class="current-info">
                        <strong>{move || current_template().and_then(|t| t.field_name).unwrap_or_else(|| "未选择文书".to_string())}</strong>
                    </div>
                    <div class="actions">
                        <button class="btn btn-primary btn-sm" on:click=handle_save_and_next>
                            {move || if cur_index.get() < templates_sig.with(|t| t.len()) - 1 { "保存并填写下一个" } else { "完成并提交" }}
                        </button>
                    </div>
                </header>
                <div class="form-content">
                    {move || match current_template() {
                        Some(tmpl) => view! {
                            <DynamicForm fields=tmpl.task_attr_list.clone() />
                        }.into_view(),
                        None => view! { 
                            <div class="empty-state">
                                <i class="el-icon-folder-opened" style="font-size: 48px; color: #dcdfe6;"></i>
                                <p>"请从左侧选择文书模板进行编辑"</p>
                            </div> 
                        }.into_view()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn DynamicForm(fields: Vec<InspectionTasksNoteAttrValVO>) -> impl IntoView {
    view! {
        <form class="dynamic-form high-fidelity">
            {fields.into_iter().map(|field| {
                let name = field.field_name.unwrap_or_default();
                let attr = field.field_attr.unwrap_or_default();
                let field_type = field.field_type.unwrap_or(1);
                
                view! {
                    <div class="form-item" class:full-width=move || field_type == 2 || field_type == 6>
                        <label>{name}</label>
                        <div class="input-content">
                            {match field_type {
                                2 => view! { <textarea class="rich-textarea" name=attr placeholder="请输入内容..."></textarea> }.into_view(),
                                3 => view! { <input class="rich-input" type="date" name=attr /> }.into_view(),
                                4 => view! { 
                                    <div class="radio-group">
                                        <label class="radio-item"><input type="radio" name=attr.clone() /> "是"</label>
                                        <label class="radio-item"><input type="radio" name=attr /> "否"</label>
                                    </div>
                                }.into_view(),
                                6 => view! { <TableField attr=attr /> }.into_view(),
                                _ => view! { <input class="rich-input" type="text" name=attr placeholder="请输入..." /> }.into_view()
                            }}
                        </div>
                    </div>
                }
            }).collect_view()}
        </form>
    }
}

#[component]
fn TableField(attr: String) -> impl IntoView {
    let (rows, set_rows) = create_signal(vec![1]); // Mock row IDs
    
    let columns = if attr == "evidenceList" {
        vec!["序号", "证据名称", "类别", "数量", "来源", "备注"]
    } else {
        vec!["序号", "项目名称", "涉及金额", "违规事实", "备注"]
    };

    view! {
        <div class="grid-table-container">
            <table class="grid-table">
                <thead>
                    <tr>
                        {columns.iter().map(|c| view! { <th>{c.to_string()}</th> }).collect_view()}
                        <th style="width: 50px;">"操作"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || rows.get().iter().enumerate().map(|(idx, &id)| {
                        view! {
                            <tr key=id>
                                <td>{idx + 1}</td>
                                {columns[1..].iter().map(|_| view! { 
                                    <td><input type="text" class="cell-input" /></td> 
                                }).collect_view()}
                                <td>
                                    <button 
                                        type="button" 
                                        class="btn-icon delete"
                                        on:click=move |_| {
                                            set_rows.update(|r| {
                                                if idx < r.len() {
                                                    r.remove(idx);
                                                }
                                            });
                                        }
                                    >
                                        "×"
                                    </button>
                                </td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
            <button 
                type="button" 
                class="btn btn-secondary btn-sm" 
                style="margin-top: 8px;"
                on:click=move |_| set_rows.update(|r| r.push(r.last().cloned().unwrap_or(0) + 1))
            >
                "+ 新增一行"
            </button>
        </div>
    }
}
