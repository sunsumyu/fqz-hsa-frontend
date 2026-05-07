use leptos::*;
use std::collections::VecDeque;
use pulldown_cmark::{Parser, Options, html};

fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    // [V15.5] 视觉强固：预清理所有非渲染类系统标签
    let mut processed = text.to_string();
    
    // 移除所有 [[[...]]] 标签 (除了前端渲染协议要求的)
    let protocol_tags = ["AUDIT_REPORT_V2", "END_REPORT"];
    while let Some(start) = processed.find("[[[") {
        if let Some(end) = processed[start..].find("]]]") {
            let tag_content = &processed[start + 3..start + end];
            
            let mut should_keep = false;
            for p in protocol_tags.iter() {
                if tag_content.contains(p) {
                    should_keep = true;
                    break;
                }
            }
            
            if !should_keep {
                processed.replace_range(start..start + end + 3, "");
            } else {
                // 跳转过该标签，避免死循环
                break; 
            }
        } else { break; }
    }

    let is_v2_report = processed.contains("[[[AUDIT_REPORT_V2]]]");

    if is_v2_report {
        // 1. 预处理：剥离协议头尾
        processed = processed.replace("[[[AUDIT_REPORT_V2]]]", "<div class=\"audit-v2-container\">")
                             .replace("[[[END_REPORT]]]", "</div>");

        // 2. 核心指标组件：<StatGrid> -> <div class="audit-stat-grid">
        processed = processed.replace("<StatGrid>", "<div class=\"audit-stat-grid\">")
                             .replace("</StatGrid>", "</div>");
        
        // 3. 单体指标：<Stat label="总额" value="XXX" /> -> 语义化 DOM
        while let Some(start) = processed.find("<Stat ") {
            if let Some(end) = processed[start..].find("/>") {
                let full_tag = &processed[start..start + end + 2];
                let label = extract_attr(full_tag, "label").unwrap_or_default();
                let value = extract_attr(full_tag, "value").unwrap_or_default();
                let trend = extract_attr(full_tag, "trend").unwrap_or_default();
                let trend_html = if trend == "up" { "<span class=\"trend-up\">↑</span>" } else { "" };
                
                let replacement = format!(
                    "<div class=\"audit-stat-item\"><div class=\"stat-label\">{}</div><div class=\"stat-value\">{}{}</div></div>",
                    label, value, trend_html
                );
                processed = processed.replace(full_tag, &replacement);
            } else { break; }
        }

        // 4. 违规卡片：<ViolationCard type="fraud" title="XXX" amount="YYY"> -> Premium Card
        while let Some(start) = processed.find("<ViolationCard ") {
            if let Some(tag_end) = processed[start..].find(">") {
                let opening_tag = &processed[start..start + tag_end + 1];
                let v_type = extract_attr(opening_tag, "type").unwrap_or("abuse".to_string());
                let v_title = extract_attr(opening_tag, "title").unwrap_or("未知违规".to_string());
                let v_amount = extract_attr(opening_tag, "amount").unwrap_or("0".to_string());
                
                let replacement_head = format!(
                    "<div class=\"audit-card risk-{}\"><div class=\"card-badge\">{}</div><div class=\"card-title\">{}</div><div class=\"card-amount\">涉及到金额: {}元</div><div class=\"card-body\">",
                    v_type, 
                    if v_type == "fraud" { "严重欺诈" } else if v_type == "abuse" { "违规滥用" } else { "疑点筛查" },
                    v_title,
                    v_amount
                );
                processed = processed.replace(opening_tag, &replacement_head);
            } else { break; }
        }
        processed = processed.replace("</ViolationCard>", "</div></div>");

        // 5. 证据链：<EvidenceLink> -> 带有链接图标的文本
        processed = processed.replace("<EvidenceLink>", "<div class=\"evidence-link\">🔗 <span class=\"link-text\">证据追溯: ")
                             .replace("</EvidenceLink>", "</span></div>");
    }

    let parser = Parser::new_ext(&processed, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

// 辅助函数：手动提取属性值
fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = tag.find(&pattern) {
        let content_start = start + pattern.len();
        if let Some(end) = tag[content_start..].find("\"") {
            return Some(tag[content_start..content_start + end].to_string());
        }
    }
    None
}


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = processPalacePacket)]
    fn process_palace_packet(packet: &str);
}

// 模拟空实现的宏，防止非 WASM 环境报错
#[cfg(not(target_arch = "wasm32"))]
fn process_palace_packet(_: &str) {}

#[component]
pub fn AgentPage() -> impl IntoView {
    let (disable_security_mask, set_disable_security_mask) = use_context::<(ReadSignal<bool>, WriteSignal<bool>)>()
        .expect("Security mask context must be provided");

    // 全局效果：当关闭安全屏蔽时，给 body 添加特殊类
    create_effect(move |_| {
        let is_disabled = disable_security_mask.get();
        if let Some(body) = document().body() {
            if is_disabled {
                let _ = body.class_list().add_1("no-security-mask");
            } else {
                let _ = body.class_list().remove_1("no-security-mask");
            }
        }
    });

    let session_id_full = crate::api::client::get_or_create_session_id();
    let session_display = if session_id_full.len() > 8 {
        format!("...{}", &session_id_full[session_id_full.len() - 8..])
    } else {
        session_id_full.clone()
    };

    let (input_val, set_input_val) = create_signal(String::new());
    let (msg_id_counter, set_msg_id_counter) = create_signal(0usize);
    
    // [V4.2.5] 升级为 RwSignal 以强制触发 UI 响应，特别针对 Edge 浏览器渲染优化
    let messages = create_rw_signal(vec![
        (0usize, "ai".to_string(), create_rw_signal("你好！我是您的智能稽核助手。我可以帮您分析医疗数据、识别潜在欺诈风险，或者为您起草稽核报告。".to_string())),
    ]);
    
    let (loading, set_loading) = create_signal(false);
    
    // [V4.5.8] 消息排队系统：支持在回答时连续输入
    let (pending_queue, set_pending_queue) = create_signal(VecDeque::<String>::new());
    // [V4.5.9] 异步任务触发器：用于解耦队列弹出与任务执行
    let (next_task_trigger, set_next_task_trigger) = create_signal(None::<String>);
    
    // 强制清理缓存标志：启动日志
    create_effect(move |_| {
        logging::log!(">>> [SYSTEM] 智能稽核 WASM V4.2.5 (加固版) 载入成功");
    });
    
    // 模型选择状态
    let (selected_model, set_selected_model) = create_signal(None::<String>);
    // 实时运行中的引擎身份 [V4.5.4] 强制初始化为非 None 字符以示正在链接
    let (active_engine, set_active_engine) = create_signal(String::from("算力并网中..."));
    // [V15.5] 实时业务逻辑状态显示
    let (current_status, set_current_status) = create_signal(String::from("系统待命中"));
    
    // [V35.0] 实时算力治理统计 (HUD)
    let (total_tokens, set_total_tokens) = create_signal(0usize);
    let (total_cost, set_total_cost) = create_signal(0.0f64);
    let (current_role, set_current_role) = create_signal(String::from("IDLE"));
    let models_res = create_resource(|| (), |_| async { crate::api::client::get_models().await });
    
    // 获取会话历史资源
    let history_res = create_resource(|| (), |_| async { crate::api::client::get_history().await });

    // 滚动锚点
    let chat_history_ref = create_node_ref::<leptos::html::Div>();

    let scroll_to_bottom = move || {
        // 使用 .get_untracked() 避免在非响应式回调中触发告警
        if let Some(el) = chat_history_ref.get_untracked() {
            el.set_scroll_top(el.scroll_height());
        }
    };

    // 当历史记录加载完成时，同步到 messages 信号
    create_effect(move |_| {
        if let Some(Ok(history)) = history_res.get() {
            if !history.is_empty() {
                let msgs = history.into_iter()
                    .enumerate()
                    .map(|(i, m)| {
                        // [V15.6] 角色归一化：由于后端 API 已收敛 role，这里仅做格式适配
                        (i + 1, m.role, create_rw_signal(m.content))
                    }).collect::<Vec<_>>();
                
                messages.set(msgs);
                set_msg_id_counter.set(messages.get().len() + 1);
                
                // 加载历史后滚动到底部
                request_animation_frame(move || scroll_to_bottom());
            }
        }
    });

    // 初始化消息 ID
    create_effect(move |_| {
        set_msg_id_counter.update(|c| *c += 1);
    });

    // --- [V4.2 Intervention] 干预状态 ---
    let (selected_checkpoint, set_selected_checkpoint) = create_signal(None::<(String, String)>);
    let (intervention_input, set_intervention_input) = create_signal(String::new());

    // --- [V4.5] Token 限额引导切换 ---
    let (token_error_suggest, set_token_error_suggest) = create_signal(None::<String>);
    
    // 绑定 3D 引擎的回调
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        fn onMemPalaceIntervention(id: &str, version: &str);
    }

    create_effect(move |_| {
        let callback = Closure::wrap(Box::new(move |id: String, version: String| {
            web_sys::console::log_2(&"Intervention triggered for:".into(), &id.clone().into());
            set_selected_checkpoint.set(Some((id, version)));
        }) as Box<dyn FnMut(String, String)>);
        
        let window = web_sys::window().unwrap();
        let _ = js_sys::Reflect::set(&window, &"onMemPalaceIntervention".into(), callback.as_ref());
        callback.forget();
    });

    // --- [V4.5.8] 核心对话处理逻辑封装 ---
    let process_message = move |msg: String| {
        set_loading.set(true);
        let user_id = msg_id_counter.get();
        set_msg_id_counter.update(|c| *c += 1);
        
        messages.update(|msgs: &mut Vec<_>| msgs.push((user_id, "user".to_string(), create_rw_signal(msg.clone()))));
        
        let ai_id = msg_id_counter.get();
        set_msg_id_counter.update(|c| *c += 1);
        let ai_sig = create_rw_signal(String::new());
        messages.update(|msgs| msgs.push((ai_id, "ai".to_string(), ai_sig)));

        request_animation_frame(move || scroll_to_bottom());

        let msg_static = msg.clone();
        
        let try_process_queue = move || {
            if !loading.get_untracked() {
                let mut popped = None;
                set_pending_queue.update(|q| popped = q.pop_front());
                if let Some(m) = popped {
                    set_next_task_trigger.set(Some(m));
                }
            }
        };

        spawn_local(async move {
            use futures::StreamExt;
            let model_id_snapshot = selected_model.get_untracked();

            match crate::api::client::post_chat_stream(&msg_static, model_id_snapshot).await {
                Ok(mut stream) => {
                    while let Some(chunk_result) = stream.next().await {
                        if let Ok(mut chunk) = chunk_result {
                            // --- [V15.5 协议拦截器增强：支持物理跨包缓存] ---
                            let mut current_packet_buffer = String::new();
                            
                            // 空间遥测与协议包拦截逻辑
                            loop {
                                let pal_start_tag = "[[[";
                                let pal_end_tag = "]]]";
                                
                                if let Some(start) = chunk.find(pal_start_tag) {
                                    if let Some(end_) = chunk[start..].find(pal_end_tag) {
                                        let end = start + end_ + pal_end_tag.len();
                                        let packet = &chunk[start..end];
                                        
                                        // 4. 资源遥测拦截 (HUD)
                                        if packet.starts_with("[[[RESOURCE:") {
                                            let json_str = &packet[12..packet.len()-3];
                                            // 极简解析，避免引入 serde_json
                                            if let Some(cost_idx) = json_str.find("'cost':") {
                                                if let Some(cost_val) = json_str[cost_idx+7..].split(',').next() {
                                                    if let Ok(c) = cost_val.trim().parse::<f64>() {
                                                        set_total_cost.update(|curr| *curr += c);
                                                    }
                                                }
                                            }
                                            if let Some(role_idx) = json_str.find("'role': '") {
                                                if let Some(role_val) = json_str[role_idx+9..].split('\'').next() {
                                                    set_current_role.set(role_val.to_string());
                                                }
                                            }
                                            if let Some(prompt_idx) = json_str.find("'prompt':") {
                                                if let Some(p_val) = json_str[prompt_idx+9..].split(',').next() {
                                                    if let Ok(p) = p_val.trim().parse::<usize>() {
                                                        set_total_tokens.update(|curr| *curr += p);
                                                    }
                                                }
                                            }
                                            if let Some(comp_idx) = json_str.find("'completion':") {
                                                if let Some(c_val) = json_str[comp_idx+13..].split(',').next().or_else(|| json_str[comp_idx+13..].split('}').next()) {
                                                    if let Ok(c) = c_val.trim().parse::<usize>() {
                                                        set_total_tokens.update(|curr| *curr += c);
                                                    }
                                                }
                                            }
                                        }
                                        // 5. 3D 渲染与 Checkpoint 协议透传
                                        else {
                                            process_palace_packet(packet);
                                        }
                                        
                                        // 从内容中抹除该包
                                        let mut new_chunk = chunk.clone();
                                        new_chunk.replace_range(start..end, "");
                                        chunk = new_chunk;
                                        continue;
                                    } else {
                                        // 发现半截包，物理吞噬并等待下一包 (简化版：由于 WASM 环境限制，此处仅清理残留)
                                        break; 
                                    }
                                }
                                break;
                            }

                            if !chunk.is_empty() {
                                // [V15.5] 诊断日志：确认流式数据到达率
                                logging::log!("<<< CHUNK RECEIVED: {} chars", chunk.len());
                                
                                // 强制触发响应式更新
                                ai_sig.update(|s| s.push_str(&chunk));
                                
                                // 强制触发一次微任务调度，确保 UI 线程有机会更新
                                request_animation_frame(move || {
                                    scroll_to_bottom();
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    let err_str = e.to_string();
                    let error_feedback = if err_str.contains("403") {
                        "[审计权限异常]: 当前模型额度已耗尽 (403)，请切换模型。".to_string()
                    } else if err_str.contains("SystemMessage") {
                        "[对话协议冲突]: 系统状态同步异常，请刷新页面重新开始。".to_string()
                    } else {
                        format!("[系统逻辑中断]: {}", err_str)
                    };
                    ai_sig.set(error_feedback);
                }
            }
            // [V4.5.9] 强制复位加载态，确保按钮和 UI 指示器能恢复可用
            set_loading.set(false);
            try_process_queue();
        });
    };

    // 唯一合法的触发器监听：利用 event-bus 模式确保单线程队列安全消费
    create_effect(move |_| {
        if let Some(msg) = next_task_trigger.get() {
            set_next_task_trigger.set(None);
            process_message(msg);
        }
    });

    let handle_send = move || {
        let msg = input_val.get_untracked();
        if !msg.is_empty() {
            set_pending_queue.update(|q| q.push_back(msg.clone()));
            set_input_val.set(String::new());
            
            if !loading.get_untracked() {
                let mut popped = None;
                set_pending_queue.update(|q| popped = q.pop_front());
                if let Some(m) = popped {
                    set_next_task_trigger.set(Some(m));
                }
            }
        }
    };

    view! {
        <div class="page-container" style="background: #f0f2f5; padding: 24px;">
            <header class="page-header" style="background: transparent; border: none; padding: 0 0 24px 0; display: flex; justify-content: space-between; align-items: center;">
                <h2 style="font-size: 24px; font-weight: 700; color: #1a1a1a; display: flex; align-items: center; gap: 16px;">
                    "智能稽核助手"
                    <button 
                        type="button"
                        on:click=move |_| {
                            logging::log!(">>> [UI] 正在执行会话硬重置...");
                            crate::api::client::reset_session_id();
                            if let Some(win) = web_sys::window() {
                                let _ = win.location().set_href("/agent");
                            }
                        }
                        style="background: #409eff; color: white; border: none; padding: 4px 12px; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 4px; transition: all 0.2s; box-shadow: 0 2px 8px rgba(64,158,255,0.3);"
                        on:mouse_over=move |e| { let _ = event_target::<web_sys::HtmlElement>(&e).style().set_property("background", "#66b1ff"); }
                        on:mouse_out=move |e| { let _ = event_target::<web_sys::HtmlElement>(&e).style().set_property("background", "#409eff"); }
                    >
                        <i class="el-icon-plus"></i> "新对话"
                    </button>
                    <span style="font-size: 14px; font-weight: 400; color: #67c23a; vertical-align: middle;">
                        <span class="typing-dot"></span> " AI 在线预览"
                    </span>
                    <span style="font-size: 11px; font-weight: 400; color: #909399; margin-left: 8px; font-family: monospace; background: #eef1f6; padding: 2px 6px; border-radius: 4px;">
                        "SSET_ID: " {session_display}
                    </span>
                </h2>
                
                <div style="display: flex; align-items: center; gap: 12px;">
                    // 模型选择下拉框
                    <div class="model-selector" style="background: rgba(255,255,255,0.8); padding: 4px 12px; border-radius: 20px; border: 1px solid #dcdfe6; display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 13px; color: #909399;">"算力模型:"</span>
                        <select 
                            style="border: none; background: transparent; font-size: 13px; color: #303133; font-weight: 600; cursor: pointer; outline: none;"
                            prop:value=move || selected_model.get().unwrap_or_else(|| "auto".to_string())
                            on:change=move |e| {
                                let val = event_target_value(&e);
                                if val == "auto" {
                                    set_selected_model.set(None);
                                } else {
                                    set_selected_model.set(Some(val));
                                }
                            }
                        >
                            <option value="auto">"自动路由 (智能切换)"</option>
                            <Suspense fallback=move || view! { <option disabled=true>"加载中..."</option> }>
                                {move || models_res.get().map(|res| {
                                    match res {
                                        Ok(models) => models.into_iter().map(|m| {
                                            let display_name = if !m.is_available {
                                                format!("{} ({})", m.name, m.status_msg)
                                            } else {
                                                m.name.clone()
                                            };
                                            let is_disabled = !m.is_available;
                                            view! { 
                                                <option 
                                                    value=move || m.id.clone() 
                                                    prop:disabled=is_disabled
                                                    style=move || if is_disabled { "color: #c0c4cc;" } else { "" }
                                                >
                                                    {display_name}
                                                </option> 
                                            }
                                        }).collect_view(),
                                        Err(_) => view! { <option disabled=true>"列表获取请求受阻"</option> }.into_view()
                                    }
                                })}
                            </Suspense>
                        </select>
                    </div>

                    // 实时算力状态指示灯
                    <div style="font-size: 13px; color: #67c23a; background: rgba(103,194,58,0.1); padding: 4px 12px; border-radius: 20px; border: 1px solid rgba(103,194,58,0.2); display: flex; align-items: center; gap: 6px;">
                        <span class="typing-dot" style="background: #67c23a; width: 6px; height: 6px; box-shadow: 0 0 8px #67c23a;"></span>
                        <span style="font-weight: 500;">{move || active_engine.get()}</span>
                    </div>

                    // [V15.5] 业务状态指示器 (业务视角)
                    <div style="font-size: 13px; color: #409eff; background: rgba(64,158,255,0.1); padding: 4px 12px; border-radius: 20px; border: 1px solid rgba(64,158,255,0.2); display: flex; align-items: center; gap: 6px;">
                        <i class="el-icon-loading" style="font-size: 14px;"></i>
                        <span style="font-weight: 600;">{move || current_status.get()}</span>
                    </div>

                    </div>

                // [V35.0] 实时算力治理 HUD (实时监控面板)
                <div class="governance-hud" style="display: flex; gap: 16px; margin-bottom: 20px; flex-wrap: wrap;">
                    <div class="hud-item" style="background: rgba(30,41,59,0.9); backdrop-filter: blur(8px); border: 1px solid rgba(255,255,255,0.1); padding: 12px 20px; border-radius: 12px; min-width: 140px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);">
                        <div style="font-size: 11px; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;">"当前链路角色"</div>
                        <div style="font-size: 16px; font-weight: 700; color: #38bdf8; font-family: monospace;">{move || current_role.get()}</div>
                    </div>
                    <div class="hud-item" style="background: rgba(30,41,59,0.9); backdrop-filter: blur(8px); border: 1px solid rgba(255,255,255,0.1); padding: 12px 20px; border-radius: 12px; min-width: 140px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);">
                        <div style="font-size: 11px; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;">"累计 Token 消耗"</div>
                        <div style="font-size: 16px; font-weight: 700; color: #facc15;">{move || total_tokens.get()}</div>
                    </div>
                    <div class="hud-item" style="background: rgba(30,41,59,0.9); backdrop-filter: blur(8px); border: 1px solid rgba(255,255,255,0.1); padding: 12px 20px; border-radius: 12px; min-width: 140px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);">
                        <div style="font-size: 11px; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 4px;">"当前会话预估成本"</div>
                        <div style="font-size: 16px; font-weight: 700; color: #fb7185;">"¥" {move || format!("{:.4}", total_cost.get())}</div>
                    </div>
                </div>
            </header>

            <div class="agent-container">
                <div class="agent-main">
                    <div class="chat-history" id="chat-scroller" node_ref=chat_history_ref>
                        <For
                            each=move || messages.get()
                            key=|m| m.0
                            children=move |(_id, sender, content_sig)| {
                                let class = if sender == "ai" { "chat-bubble bubble-ai" } else { "chat-bubble bubble-user" };
                                
                                view! { 
                                    <div class=class>
                                        <div 
                                            class="chat-content markdown-content"
                                            inner_html=move || render_markdown(&content_sig.get())
                                        ></div>
                                    </div>
                                }
                            }
                        />

                        // [V4.5.9] 全局式加载指示器：确保永远处于消息列表最底部，且全局唯一
                        {move || loading.get().then(|| view! {
                            <div class="chat-bubble bubble-ai" style="margin-bottom: 24px; width: fit-content;">
                                <div class="loading-content-placeholder" style="display: flex; align-items: center; gap: 8px;">
                                    <span class="typing-text" style="font-size: 0.85em; opacity: 0.7; color: #409eff; font-weight: 600;">"引擎正在深度审计中..."</span>
                                    <div class="dot-jump">
                                        <span class="typing-dot" style="background: #409eff"></span>
                                        <span class="typing-dot" style="background: #409eff; animation-delay: 0.2s"></span>
                                        <span class="typing-dot" style="background: #409eff; animation-delay: 0.4s"></span>
                                    </div>
                                </div>
                            </div>
                        })}
                    </div>

                    <div class="chat-input-area">
                        <form 
                            class="chat-input-wrapper"
                            on:submit=move |ev| {
                                ev.prevent_default();
                                handle_send();
                            }
                        >
                            <input 
                                type="text" 
                                placeholder="输入您的问题，例如：‘总结本月高风险案例’" 
                                prop:value=input_val
                                on:input=move |e| set_input_val.set(event_target_value(&e))
                            />
                            <button 
                                type="submit"
                                class="btn btn-primary" 
                                style="border-radius: 20px; padding: 0 24px;"
                            >
                                {move || {
                                    let q_len = pending_queue.get().len();
                                    if q_len > 0 { format!("排队中 ({})", q_len) }
                                    else if loading.get() { "处理中...".to_string() }
                                    else { "发送".to_string() }
                                }}
                            </button>
                        </form>
                    </div>
                </div>

                <aside class="agent-side">
                    <div class="context-task">
                        <h4>"当前关注任务"</h4>
                        <div style="margin-bottom: 8px; opacity: 0.9;">"编号: TEST-PUN-001"</div>
                        <div style="font-weight: 600; font-size: 14px; margin-bottom: 12px;">"长沙第一医院违规报销案"</div>
                        <div style="font-size: 12px; line-height: 1.5; color: rgba(255,255,255,0.7);">
                            "当前进度：立案待审批。系统已自动关联《医疗保障基金使用监督管理条例》第十五条。"
                        </div>
                    </div>

                    <div class="quick-action-card">
                        <div class="action-title">"智能辅助工具"</div>
                        <div class="action-chips">
                            <span class="chip">"风险点识别"</span>
                            <span class="chip">"同类案例比对"</span>
                            <span class="chip">"合规性初筛"</span>
                            <span class="chip">"生成笔录草稿"</span>
                        </div>
                    </div>

                    <div class="quick-action-card">
                        <div class="action-title">"知识库搜索"</div>
                        <div class="action-chips">
                            <span class="chip" style="background: #f6ffed; border-color: #b7eb8f; color: #52c41a;">"最新监管政策"</span>
                            <span class="chip" style="background: #fff7e6; border-color: #ffd591; color: #fa8c16;">"地方裁量基准"</span>
                        </div>
                    </div>
                </aside>
            </div>

            // --- [V4.2 Intervention HUB] ---
            {move || selected_checkpoint.get().map(|(id, version): (String, String)| view! {
                <div class="intervention-overlay" style="position: fixed; inset: 0; background: rgba(0,0,0,0.6); display: flex; justify-content: center; align-items: center; z-index: 20000; backdrop-filter: blur(4px);">
                    <div class="intervention-card" style="background: #fff; width: 500px; border-radius: 12px; overflow: hidden; box-shadow: 0 10px 40px rgba(0,0,0,0.3);">
                        <div style="background: #1e293b; color: white; padding: 16px 20px; display: flex; justify-content: space-between; align-items: center;">
                            <h3 style="margin: 0; font-size: 16px;">"时空干预面板 (Intervention HUB)"</h3>
                            <button on:click=move |_| set_selected_checkpoint.set(None) style="background: transparent; border: none; color: white; cursor: pointer;">"✕"</button>
                        </div>
                        
                        <div style="padding: 20px;">
                            <div style="font-size: 12px; color: #64748b; margin-bottom: 12px;">
                                "Checkpoint ID: " <code style="background: #f1f5f9; padding: 2px 4px; border-radius: 4px;">{id.clone()}</code>
                                <span style="margin-left: 12px;">"版本: " {version}</span>
                            </div>

                            <div style="margin-bottom: 16px;">
                                <label style="display: block; font-size: 14px; font-weight: 600; margin-bottom: 8px;">"人工注入/修正线索 (Findings Override)"</label>
                                <textarea 
                                    style="width: 100%; height: 120px; border: 1px solid #dcdfe6; border-radius: 8px; padding: 12px; font-size: 13px; font-family: inherit; resize: none;"
                                    placeholder="输入修正后的业务事实，AI 将根据此事实重新推演..."
                                    on:input=move |e| set_intervention_input.set(event_target_value(&e))
                                    prop:value=intervention_input
                                />
                            </div>

                            <div style="display: flex; gap: 12px;">
                                <button 
                                    class="btn" 
                                    style="flex: 1; background: #60a5fa; color: white;"
                                    on:click=move |_| {
                                        let input = intervention_input.get();
                                        spawn_local(async move {
                                            let _ = crate::api::client::update_state(&input).await;
                                            set_selected_checkpoint.set(None);
                                            // TODO: 触发自动重启
                                        });
                                    }
                                >
                                    "注入修正并继续"
                                </button>
                                <button 
                                    class="btn" 
                                    style="flex: 1; background: #f1f5f9; color: #1e293b;"
                                    on:click=move |_| set_selected_checkpoint.set(None)
                                >
                                    "取消"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            })}

            // --- [V4.5 Token Switch Modal] ---
            {move || token_error_suggest.get().map(|suggestion| view! {
                <div class="token-modal-overlay" style="position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; justify-content: center; align-items: center; z-index: 30000; backdrop-filter: blur(8px);">
                    <div style="background: #fff; width: 420px; border-radius: 16px; padding: 32px; text-align: center; box-shadow: 0 20px 50px rgba(0,0,0,0.5);">
                        <div style="width: 64px; height: 64px; background: #fff7e6; color: #faad14; border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 20px; font-size: 32px;">"!"</div>
                        <h3 style="margin-bottom: 12px; font-weight: 700;">"当前模型额度已满"</h3>
                        <p style="color: #64748b; font-size: 14px; line-height: 1.6; margin-bottom: 24px;">"您的当前选中的模型由于由于额度受限制已暂停服务。为了保证您的审计工作不被中断，建议一键切换到可用的算力方案。"</p>
                        
                        <div style="background: #f8fafc; border-radius: 8px; padding: 16px; margin-bottom: 24px; border: 1px dashed #cbd5e1;">
                            <span style="font-size: 12px; color: #94a3b8; display: block; margin-bottom: 4px;">"建议切换至"</span>
                            <span style="font-weight: 700; color: #1e293b; font-size: 16px;">{suggestion.clone()}</span>
                        </div>

                        <div style="display: flex; flex-direction: column; gap: 12px;">
                            <button 
                                style="background: #2563eb; color: white; border: none; padding: 12px; border-radius: 8px; font-weight: 600; cursor: pointer;"
                                on:click=move |_| {
                                    if suggestion != "None" {
                                        set_selected_model.set(Some(suggestion.clone()));
                                        set_token_error_suggest.set(None);
                                        // 触发重新发送逻辑，此处通过重新调用 handle_send
                                        handle_send();
                                    } else {
                                        set_token_error_suggest.set(None);
                                    }
                                }
                            >
                                "立即切换并继续推理"
                            </button>
                            <button 
                                style="background: #f1f5f9; color: #475569; border: none; padding: 12px; border-radius: 8px; font-weight: 500; cursor: pointer;"
                                on:click=move |_| set_token_error_suggest.set(None)
                            >
                                "稍后处理"
                            </button>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}
