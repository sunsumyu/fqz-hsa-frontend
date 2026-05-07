use leptos::*;

#[component]
pub fn MemoryAuditorPage() -> impl IntoView {
    let raw_session = crate::api::client::get_or_create_session_id();
    // 强制转换为响应式 signal，即使它的初始值目前不打算经常变动
    let (session_id, set_session_id) = create_signal(raw_session);

    let timeline_res = create_resource(
        move || session_id.get(),
        |sid| async move { crate::api::client::get_memory_timeline(&sid).await },
    );
    
    let conflicts_res = create_resource(
        || (),
        |_| async { crate::api::client::get_conflicts().await },
    );

    view! {
        <div class="page-container" style="padding: 24px; background: #f0f2f5; min-height: 100vh; display: flex; flex-direction: column;">
            <header class="page-header" style="background: transparent; border: none; padding: 0 0 20px 0; display: flex; justify-content: space-between; align-items: center;">
                <div>
                    <h2 style="font-size: 22px; font-weight: 700; color: #1a1a1a; margin: 0;">"记忆时间轴 (Memory Timeline)"</h2>
                    <p style="font-size: 13px; color: #909399; margin: 4px 0 0 0;">"追溯与审计 Agent 内部认知的演进过程"</p>
                </div>
                <div style="display: flex; align-items: center; gap: 12px; background: white; padding: 8px 16px; border-radius: 6px; box-shadow: 0 1px 4px rgba(0,0,0,0.05);">
                    <span style="font-size: 12px; color: #606266; font-weight: 600;">"Current Session"</span>
                    <input
                        type="text"
                        on:input=move |ev| set_session_id.set(event_target_value(&ev))
                        prop:value=move || session_id.get()
                        style="border: 1px solid #dcdfe6; border-radius: 4px; padding: 4px 8px; font-family: monospace; font-size: 11px; width: 200px; color: #909399;"
                    />
                    <button 
                        on:click=move |_| {
                            timeline_res.refetch();
                            conflicts_res.refetch();
                        }
                        style="background: #e6f7ff; color: #1890ff; border: 1px solid #91d5ff; border-radius: 4px; padding: 4px 10px; cursor: pointer; font-size: 12px;"
                    >
                        "拉取轨迹"
                    </button>
                </div>
            </header>

            // ==== 冲突警告横幅 ====
            <Suspense fallback=move || view!{<div></div>}>
                {move || conflicts_res.get().map(|res| match res {
                    Ok(c) if !c.conflicts.is_empty() => {
                        view! {
                            <div style="background: #fef0f0; border: 1px solid #fde2e2; border-radius: 8px; padding: 16px 24px; margin-bottom: 24px; box-shadow: 0 4px 12px rgba(245,108,108,0.1);">
                                <div style="display: flex; align-items: center; gap: 8px; color: #f56c6c; font-weight: 600; font-size: 16px; margin-bottom: 12px;">
                                    <span>"⚠️"</span>
                                    <span>{format!("警告：系统检测到 {} 处逻辑冲突", c.conflicts.len())}</span>
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 12px;">
                                    {c.conflicts.into_iter().map(|item| {
                                        let item_a = item.item_a.clone();
                                        let item_b = item.item_b.clone();
                                        view! {
                                            <div style="background: white; border: 1px solid #ebeef5; border-radius: 6px; padding: 16px; display: flex; flex-direction: column; gap: 12px;">
                                                <div style="color: #606266; font-size: 13px; font-weight: 600;">{item.description}</div>
                                                <div style="display: flex; gap: 16px;">
                                                    <div style="flex: 1; background: #fafafa; border: 1px dashed #dcdfe6; padding: 12px; border-radius: 4px; display: flex; flex-direction: column; gap: 12px;">
                                                        <span style="font-size: 12px; color: #909399;">"证据 A / 结论 A:"</span>
                                                        <div style="font-family: monospace; font-size: 12px; color: #303133;">{item_a.clone()}</div>
                                                        <button 
                                                            on:click={
                                                                let a = item_a.clone();
                                                                let b = item_b.clone();
                                                                move |_| {
                                                                    leptos::spawn_local({
                                                                        let a = a.clone(); let b = b.clone();
                                                                        async move {
                                                                            let _ = crate::api::client::resolve_conflict(&a, &b, true, false).await;
                                                                            conflicts_res.refetch();
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                            style="align-self: flex-start; padding: 6px 12px; background: white; border: 1px solid #dcdfe6; border-radius: 4px; font-size: 12px; cursor: pointer; color: #606266;"
                                                            onMouseOver="this.style.borderColor='#409eff'; this.style.color='#409eff'"
                                                            onMouseOut="this.style.borderColor='#dcdfe6'; this.style.color='#606266'"
                                                        >
                                                            "保留证据 A (废弃 B)"
                                                        </button>
                                                    </div>
                                                    <div style="flex: 1; background: #fafafa; border: 1px dashed #dcdfe6; padding: 12px; border-radius: 4px; display: flex; flex-direction: column; gap: 12px;">
                                                        <span style="font-size: 12px; color: #909399;">"证据 B / 结论 B:"</span>
                                                        <div style="font-family: monospace; font-size: 12px; color: #303133;">{item_b.clone()}</div>
                                                        <button 
                                                            on:click={
                                                                let a = item_a.clone();
                                                                let b = item_b.clone();
                                                                move |_| {
                                                                    leptos::spawn_local({
                                                                        let a = a.clone(); let b = b.clone();
                                                                        async move {
                                                                            let _ = crate::api::client::resolve_conflict(&a, &b, false, true).await;
                                                                            conflicts_res.refetch();
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                            style="align-self: flex-start; padding: 6px 12px; background: white; border: 1px solid #dcdfe6; border-radius: 4px; font-size: 12px; cursor: pointer; color: #606266;"
                                                            onMouseOver="this.style.borderColor='#409eff'; this.style.color='#409eff'"
                                                            onMouseOut="this.style.borderColor='#dcdfe6'; this.style.color='#606266'"
                                                        >
                                                            "保留证据 B (废弃 A)"
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_view()
                    },
                    _ => view! { <div></div> }.into_view()
                })}
            </Suspense>

            <div class="card" style="flex: 1; display: flex; padding: 0; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08); border-radius: 8px;">
                <Suspense fallback=move || view! { <div style="padding: 40px; color: #909399; text-align: center; width: 100%;">"从时间长河中打捞记忆切片..."</div> }>
                    {move || timeline_res.get().map(|res| match res {
                        Ok(tl) => {
                            let events = tl.events;
                            if events.is_empty() {
                                return view! {
                                    <div style="padding: 60px; text-align: center; width: 100%;">
                                        <div style="font-size: 48px; margin-bottom: 16px;">"⏳"</div>
                                        <h3 style="color: #303133; font-weight: 600;">"当前会话暂无记忆锚点"</h3>
                                        <p style="color: #909399; font-size: 13px;">"（Agent 必须执行带有工具调用的推理，才能凝结为时间切片）"</p>
                                    </div>
                                }.into_view();
                            }
                            view! {
                                // 左侧时间滑轨
                                <div style="width: 380px; background: #fafafa; border-right: 1px solid #ebeef5; overflow-y: auto; padding: 24px;">
                                    <div style="margin-bottom: 24px; font-size: 14px; font-weight: 600; color: #303133; display: flex; justify-content: space-between;">
                                        <span>"演化时间轴"</span>
                                        <span style="color: #1890ff; background: #e6f7ff; padding: 2px 8px; border-radius: 10px; font-size: 11px;">{format!("{} 切片", events.len())}</span>
                                    </div>
                                    <div style="position: relative; padding-left: 12px; margin-left: 8px; border-left: 2px solid #e4e7ed;">
                                        {events.clone().into_iter().enumerate().map(|(idx, ev)| {
                                            view! {
                                                <div style="position: relative; margin-bottom: 24px; cursor: pointer; transition: all 0.2s; background: white; padding: 12px; border: 1px solid #ebeef5; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.02); left: -10px;">
                                                    <div style="position: absolute; left: -15px; top: 16px; width: 10px; height: 10px; background: white; border: 2px solid #1890ff; border-radius: 50%;"></div>
                                                    <div style="font-size: 11px; color: #909399; font-family: monospace; display: flex; justify-content: space-between;">
                                                        <span>{format!("#{}", idx+1)}</span>
                                                        <span>{ev.ts.split('T').nth(1).unwrap_or("").split('.').next().unwrap_or("").to_string()}</span>
                                                    </div>
                                                    <div style="font-size: 13px; font-weight: 600; color: #303133; margin-top: 4px; display: flex; gap: 8px; align-items: center;">
                                                        <span style="background: #f0f9eb; color: #67c23a; border: 1px solid #e1f3d8; font-size: 10px; padding: 1px 4px; border-radius: 4px;">{ev.node.clone()}</span>
                                                        <span>{ev.event.clone()}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                        // 结束锚点
                                        <div style="position: relative; left: -10px;">
                                            <div style="position: absolute; left: -15px; top: 0; width: 10px; height: 10px; background: #e4e7ed; border-radius: 50%;"></div>
                                            <div style="font-size: 11px; color: #c0c4cc; margin-left: 12px;">"当前端点"</div>
                                        </div>
                                    </div>
                                </div>
                                // 右侧记忆快照详情
                                <div style="flex: 1; padding: 32px; background: white; overflow-y: auto;">
                                    <div style="margin-bottom: 24px;">
                                        <h3 style="font-size: 18px; color: #1a1a1a; margin: 0 0 8px 0;">"记忆快照解析区"</h3>
                                        <p style="color: #909399; font-size: 13px; margin: 0;">"点击左侧节点，或拖动时间轴，可在此回溯事发现场的真实认知结构。"</p>
                                    </div>

                                    {events.clone().into_iter().enumerate().map(|(idx, ev)| {
                                        view! {
                                            <div style="margin-bottom: 32px; border: 1px solid #ebeef5; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.05);">
                                                <div style="background: #f5f7fa; padding: 12px 16px; border-bottom: 1px solid #ebeef5; display: flex; justify-content: space-between; align-items: center;">
                                                    <span style="font-weight: 600; font-size: 14px; color: #303133;">{format!("切片快照 #{}", idx+1)}</span>
                                                    <span style="color: #909399; font-size: 11px; font-family: monospace;">{ev.ts}</span>
                                                </div>
                                                <div style="padding: 16px;">
                                                    <div style="margin-bottom: 12px;">
                                                        <span style="font-size: 12px; font-weight: 600; color: #606266; display: block; margin-bottom: 4px;">"执行动作："</span>
                                                        <div style="background: #ecf5ff; color: #409eff; padding: 8px 12px; border-radius: 4px; font-size: 13px;">{ev.event}</div>
                                                    </div>
                                                    <div>
                                                        <span style="font-size: 12px; font-weight: 600; color: #606266; display: block; margin-bottom: 4px;">"产生结论 (Findings / Memory Delta)："</span>
                                                        <pre style="background: #282c34; color: #abb2bf; padding: 12px; border-radius: 4px; font-size: 12px; font-family: 'JetBrains Mono', monospace; white-space: pre-wrap; overflow-x: auto; margin: 0;">
                                                            {ev.finding}
                                                        </pre>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        },
                        Err(e) => view! { 
                            <div style="padding: 40px; color: #f5222d; text-align: center; width: 100%;">
                                <div style="font-size: 24px; margin-bottom: 8px;">"❌"</div>
                                {format!("无法解析时间轴扭曲: {}", e)}
                            </div> 
                        }.into_view()
                    })}
                </Suspense>
            </div>
        </div>
    }
}
