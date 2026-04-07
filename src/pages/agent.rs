use leptos::*;
use pulldown_cmark::{Parser, Options, html};

fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

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

    let (input_val, set_input_val) = create_signal(String::new());
    let (msg_id_counter, set_msg_id_counter) = create_signal(0usize);
    let (messages, set_messages) = create_signal(vec![
        (0usize, "ai".to_string(), create_rw_signal("你好！我是您的智能稽核助手。我可以帮您分析医疗数据、识别潜在欺诈风险，或者为您起草稽核报告。".to_string())),
    ]);
    let (loading, set_loading) = create_signal(false);
    
    // 模型选择状态
    let (selected_model, set_selected_model) = create_signal(None::<String>);
    // 实时运行中的引擎身份
    let (active_engine, set_active_engine) = create_signal(String::from("等待指令..."));
    
    // 获取后端模型列表资源
    let models_res = create_resource(|| (), |_| async { crate::api::client::get_models().await });
    
    // 滚动锚点
    let chat_history_ref = create_node_ref::<leptos::html::Div>();

    let scroll_to_bottom = move || {
        if let Some(el) = chat_history_ref.get() {
            el.set_scroll_top(el.scroll_height());
        }
    };

    // 初始化消息 ID
    set_msg_id_counter.update(|c| *c += 1);

    let handle_send = move || {
        let msg = input_val.get();
        if !msg.is_empty() && !loading.get() {
            let user_id = msg_id_counter.get();
            set_msg_id_counter.update(|c| *c += 1);
            
            set_messages.update(|msgs| msgs.push((user_id, "user".to_string(), create_rw_signal(msg.clone()))));
            set_input_val.set(String::new());
            set_loading.set(true);
            
            // 发送后立即拉到底部
            request_animation_frame(move || scroll_to_bottom());

            spawn_local(async move {
                use futures::StreamExt;
                
                match crate::api::client::post_chat_stream(&msg, selected_model.get()).await {
                    Ok(mut stream) => {
                        let mut ai_signal: Option<RwSignal<String>> = None;
                        while let Some(chunk_result) = stream.next().await {
                            if let Ok(mut chunk) = chunk_result {
                                // 协议拦截：精准检测模型身份元数据 ⟦ENGINE:id⟧
                                let start_tag = "⟦ENGINE:";
                                let end_tag = "⟧";
                                
                                if let Some(start) = chunk.find(start_tag) {
                                    if let Some(end) = chunk.find(end_tag) {
                                        // 动态获取标识内容并更新 UI
                                        let id_start = start + start_tag.len();
                                        let engine_id = &chunk[id_start..end];
                                        set_active_engine.set(engine_id.to_string());
                                        
                                        // 物理剔除协议标记（注意：replace_range 使用的是字节索引，end_tag 为 3 字节）
                                        let mut new_chunk = chunk.clone();
                                        new_chunk.replace_range(start..end + end_tag.len(), "");
                                        chunk = new_chunk;
                                    }
                                }

                                // 气泡保活逻辑：即使为空，只要是流的开始，就必须确保 ai_signal 物理存在
                                if ai_signal.is_none() {
                                    let ai_id = msg_id_counter.get();
                                    set_msg_id_counter.update(|c| *c += 1);
                                    let sig = create_rw_signal(String::new());
                                    set_messages.update(|msgs| msgs.push((ai_id, "ai".to_string(), sig)));
                                    ai_signal = Some(sig);
                                }

                                if !chunk.is_empty() {
                                    if let Some(sig) = ai_signal {
                                        sig.update(|s| s.push_str(&chunk));
                                        // 关键：每字必追。由于响应式渲染，此处调用确保视口跟随
                                        scroll_to_bottom();
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let err_id = msg_id_counter.get();
                        set_msg_id_counter.update(|c| *c += 1);
                        set_messages.update(|msgs| msgs.push((err_id, "ai".to_string(), create_rw_signal(format!("错误: {}", e)))));
                    }
                }
                set_loading.set(false);
            });
        }
    };

    view! {
        <div class="page-container" style="background: #f0f2f5; padding: 24px;">
            <header class="page-header" style="background: transparent; border: none; padding: 0 0 24px 0; display: flex; justify-content: space-between; align-items: center;">
                <h2 style="font-size: 24px; font-weight: 700; color: #1a1a1a;">
                    "智能稽核助手"
                    <span style="font-size: 14px; font-weight: 400; color: #67c23a; margin-left: 12px; vertical-align: middle;">
                        <span class="typing-dot"></span> " AI 在线预览"
                    </span>
                </h2>
                
                <div style="display: flex; align-items: center; gap: 12px;">
                    // 模型选择下拉框
                    <div class="model-selector" style="background: rgba(255,255,255,0.8); padding: 4px 12px; border-radius: 20px; border: 1px solid #dcdfe6; display: flex; align-items: center; gap: 8px;">
                        <span style="font-size: 13px; color: #909399;">"算力模型:"</span>
                        <select 
                            style="border: none; background: transparent; font-size: 13px; color: #303133; font-weight: 600; cursor: pointer; outline: none;"
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
                                            view! { <option value=move || m.id.clone()>{m.name.clone()}</option> }
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

                    <div class="security-toggle" style="display: flex; align-items: center; gap: 8px; font-size: 14px; color: #606266; background: #fff; padding: 8px 16px; border-radius: 20px; box-shadow: 0 2px 12px rgba(0,0,0,0.05);">
                        <label style="cursor: pointer; display: flex; align-items: center; gap: 6px;">
                            <input 
                                type="checkbox" 
                                prop:checked=disable_security_mask
                                on:change=move |e| set_disable_security_mask.set(event_target_checked(&e))
                            />
                            "全面脱敏"
                        </label>
                    </div>
                </div>
            </header>

            <div class="agent-container">
                <div class="agent-main">
                    <div class="chat-history" node_ref=chat_history_ref>
                        <For
                            each=move || messages.get()
                            key=|m| m.0
                            children=move |(_id, sender, content_sig)| {
                                let class = if sender == "ai" { "chat-bubble bubble-ai" } else { "chat-bubble bubble-user" };
                                
                                view! { 
                                    <div class=class>
                                        <div inner_html=move || render_markdown(&content_sig.get())></div>
                                        
                                        // 归一化逻辑：如果是 AI 且内容仍为空且正加载，直接在气泡内显示转圈
                                        {move || (sender == "ai" && content_sig.get().trim().is_empty() && loading.get()).then(|| view! {
                                            <div class="loading-content-placeholder">
                                                <span class="typing-text" style="font-size: 0.9em; opacity: 0.8;">"AI 智能审计中"</span>
                                                <div class="dot-jump">
                                                    <span class="typing-dot"></span>
                                                    <span class="typing-dot"></span>
                                                    <span class="typing-dot"></span>
                                                </div>
                                            </div>
                                        })}
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class="chat-input-area">
                        <div class="chat-input-wrapper">
                            <input 
                                type="text" 
                                placeholder="输入您的问题，例如：‘总结本月高风险案例’" 
                                prop:value=input_val
                                on:input=move |e| set_input_val.set(event_target_value(&e))
                                on:keydown=move |e| {
                                    if e.key() == "Enter" { handle_send(); }
                                }
                                disabled=loading
                            />
                            <button 
                                class="btn btn-primary" 
                                on:click=move |_| handle_send() 
                                style="border-radius: 20px; padding: 0 24px;"
                                disabled=loading
                            >
                                {move || if loading.get() { "处理中..." } else { "发送" }}
                            </button>
                        </div>
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
        </div>
    }
}
