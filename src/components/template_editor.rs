use leptos::*;
use leptos_router::use_navigate;
use crate::api::models::{InspectionTasksNoteAttrListVO, InspectionTasksNoteAttrValVO, WrapperResponse, InspectionTasksNotePunishSubmitReq};
use std::collections::HashMap;

#[component]
pub fn TemplateEditor(
    inspection_id: i32,
    note_category: String,
    #[prop(optional)] on_save: Option<Callback<Vec<InspectionTasksNoteAttrValVO>>>,
) -> impl IntoView {
    // In a real app, this would be a create_resource call
    let templates: Vec<InspectionTasksNoteAttrListVO> = serde_json::from_str(include_str!("../api/templates.json"))
        .unwrap_or_default();
    
    let (templates_sig, _) = create_signal(templates);
    let (cur_index, set_cur_index) = create_signal(0);
    
    // tracks all values across all templates: template_id -> attr_id -> value
    let (form_values, set_form_values) = create_signal(HashMap::<i32, HashMap<String, String>>::new());

    let current_template = move || templates_sig.with(|t| t.get(cur_index.get()).cloned());

    let collect_current_data = move || {
        if let Some(curr) = current_template() {
            let tid = curr.template_id;
            let mut reqs = Vec::new();
            form_values.with(|all_map| {
                if let Some(template_map) = all_map.get(&tid) {
                    for attr in &curr.task_attr_list {
                        let attr_name = attr.field_attr.clone().unwrap_or_default();
                        
                        // If it's a table field (type 6), we aggregate all cells into a JSON string
                        let val = if attr.field_type == Some(6) {
                            let mut grid_data = Vec::new();
                            // Scan for keys like "attr_name[row][col]"
                            // This is a bit inefficient but necessary for a dynamic grid
                            let prefix = format!("{}[", attr_name);
                            let mut rows_found = std::collections::BTreeMap::new();
                            
                            for (k, v) in template_map {
                                if k.starts_with(&prefix) {
                                    // Parse "attr_name[row][col]"
                                    let parts: Vec<&str> = k.split(|c| c == '[' || c == ']').filter(|s| !s.is_empty()).collect();
                                    if parts.len() == 3 {
                                        if let (Ok(row_idx), Ok(col_idx)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                                            rows_found.entry(row_idx).or_insert_with(std::collections::BTreeMap::new).insert(col_idx, v.clone());
                                        }
                                    }
                                }
                            }
                            
                            for (_r_idx, cols) in rows_found {
                                let row_vec: Vec<String> = cols.into_values().collect();
                                grid_data.push(row_vec);
                            }
                            
                            serde_json::to_string(&grid_data).unwrap_or_default()
                        } else {
                            template_map.get(&attr_name).cloned().unwrap_or_default()
                        };

                        reqs.push(InspectionTasksNoteAttrValVO {
                            id: attr.id,
                            inspection_id: Some(inspection_id),
                            template_id: Some(tid),
                            field_name: attr.field_name.clone(),
                            field_attr: Some(attr_name),
                            field_value: Some(val),
                            field_type: attr.field_type,
                            field_class: attr.field_class.clone(),
                            required: attr.required,
                        });
                    }
                }
            });
            reqs
        } else {
            Vec::new()
        }
    };

    let handle_save_and_next = move |_| {
        if cur_index.get() < templates_sig.with(|t| t.len()) - 1 {
            set_cur_index.update(|i| *i += 1);
        } else {
            // Signal the parent if on_save is provided
            let reqs = collect_current_data();
            if let Some(on_save_cb) = on_save {
                on_save_cb.call(reqs);
                return;
            }

            // Otherwise, Final submission directly
            if let Some(curr) = current_template() {
                // Validation before submission
                if inspection_id == 0 {
                    let _ = window().alert_with_message("无效的稽查任务ID(0)，请在台账重新选择任务。");
                    return;
                }

                let mut missing_fields = Vec::new();
                for r in &reqs {
                    if r.required == Some(true) {
                        let val = r.field_value.as_deref().unwrap_or("");
                        if val.is_empty() || val == "[]" {
                            missing_fields.push(r.field_name.clone().unwrap_or_else(|| "未知字段".to_string()));
                        }
                    }
                }

                if !missing_fields.is_empty() {
                    let _ = window().alert_with_message(&format!("提交失败：以下必填项未填写: {}", missing_fields.join(", ")));
                    return;
                }

                let batch_req = InspectionTasksNotePunishSubmitReq {
                    inspection_id,
                    template_id: curr.template_id,
                    reqs,
                };

                spawn_local(async move {
                    match crate::api::client::post::<_, WrapperResponse<String>>("/insp/tempattrval/batch/add", &batch_req).await {
                        Ok(resp) => {
                            let msg = resp.data.unwrap_or_else(|| "提交成功".to_string());
                            let _ = window().alert_with_message(&msg);
                            let navigate = use_navigate();
                            navigate("/ledger", Default::default());
                        }
                        Err(e) => {
                            let _ = window().alert_with_message(&format!("提交失败: {}", e));
                        }
                    }
                });
            }
        }
    };

    view! {
        <div class="tmpl-container">
            <div class="left">
                <div class="title">"环节对应文书"</div>
                <div class="note-category-tag" style="padding: 8px 16px; font-size: 12px; border-bottom: 1px solid #f0f0f0; background: #fafafa; color: #666;">
                    {format!("当前环节：{}", note_category)}
                </div>
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
                        Some(tmpl) => {
                            let tid = tmpl.template_id;
                            view! {
                                <DynamicForm 
                                    fields=tmpl.task_attr_list.clone() 
                                    on_input=move |attr, val| {
                                        set_form_values.update(|map| {
                                            map.entry(tid).or_default().insert(attr, val);
                                        });
                                    }
                                    values=Signal::derive(move || form_values.with(|m| m.get(&tid).cloned().unwrap_or_default()))
                                />
                            }.into_view()
                        },
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
fn DynamicForm<F>(
    fields: Vec<InspectionTasksNoteAttrValVO>, 
    on_input: F,
    #[prop(into)] values: MaybeSignal<HashMap<String, String>>,
) -> impl IntoView 
where F: Fn(String, String) + Clone + 'static
{
    view! {
        <form class="dynamic-form high-fidelity" on:submit=|e| e.prevent_default()>
            {fields.into_iter().map(|field| {
                let name = field.field_name.unwrap_or_default();
                let attr = field.field_attr.clone().unwrap_or_default();
                let field_type = field.field_type.unwrap_or(1);
                let on_input = on_input.clone();
                let attr_clone = attr.clone();
                let values = values.clone();

                view! {
                    <div class="form-item" class:full-width=move || field_type == 2 || field_type == 6>
                        <label>{name}</label>
                        <div class="input-content">
                            {match field_type {
                                2 => view! { 
                                    <textarea 
                                        class="rich-textarea" 
                                        placeholder="请输入内容..."
                                        on:input=move |ev| on_input(attr_clone.clone(), event_target_value(&ev))
                                        prop:value=move || values.with(|m| m.get(&attr).cloned().unwrap_or_default())
                                    ></textarea> 
                                }.into_view(),
                                3 => view! { 
                                    <input 
                                        class="rich-input" type="date" 
                                        on:input=move |ev| on_input(attr_clone.clone(), event_target_value(&ev))
                                        prop:value=move || values.with(|m| m.get(&attr).cloned().unwrap_or_default())
                                    /> 
                                }.into_view(),
                                4 => {
                                    let _on_input_y = on_input.clone();
                                    let attr_y = attr_clone.clone();
                                    let _on_input_n = on_input.clone();
                                    let attr_n = attr_clone.clone();
                                    view! { 
                                        <div class="radio-group" on:change=move |ev| on_input(attr_clone.clone(), event_target_value(&ev))>
                                            <label class="radio-item"><input type="radio" name=attr_y value="1" /> "是"</label>
                                            <label class="radio-item"><input type="radio" name=attr_n value="0" /> "否"</label>
                                        </div>
                                    }.into_view()
                                },
                                6 => {
                                    let on_input_c = on_input.clone();
                                    let values_c = values.clone();
                                    view! { <TableField attr=attr_clone on_input=on_input_c values=values_c /> }.into_view()
                                },
                                _ => view! { 
                                    <input 
                                        class="rich-input" type="text" placeholder="请输入..." 
                                        on:input=move |ev| on_input(attr_clone.clone(), event_target_value(&ev))
                                        prop:value=move || values.with(|m| m.get(&attr).cloned().unwrap_or_default())
                                    /> 
                                }.into_view()
                            }}
                        </div>
                    </div>
                }
            }).collect_view()}
        </form>
    }
}

#[component]
fn TableField<F>(attr: String, on_input: F, #[prop(into)] values: MaybeSignal<HashMap<String, String>>) -> impl IntoView 
where F: Fn(String, String) + Clone + 'static
{
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
                        let on_input = on_input.clone();
                        let attr_row = attr.clone();
                        let values = values.clone();
                        view! {
                            <tr key=id>
                                <td>{idx + 1}</td>
                                {columns[1..].iter().enumerate().map(|(col_idx, _)| {
                                    let on_input_c = on_input.clone();
                                    let values_c = values.clone();
                                    let attr_cell = format!("{}[{}][{}]", attr_row, idx, col_idx);
                                    let attr_cell_key = attr_cell.clone();
                                    view! { 
                                        <td>
                                            <input 
                                                type="text" 
                                                class="cell-input" 
                                                on:input=move |ev| on_input_c(attr_cell_key.clone(), event_target_value(&ev))
                                                prop:value=move || values_c.with(|m| m.get(&attr_cell).cloned().unwrap_or_default())
                                            />
                                        </td> 
                                    }
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
