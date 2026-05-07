use leptos::*;
use leptos_router::A;

#[component]
pub fn DashboardPage() -> impl IntoView {
    // 实时拉取后端指标
    let metrics_res = create_resource(|| (), |_| async { crate::api::client::get_metrics().await });
    let trace_res = create_resource(|| (), |_| async { crate::api::client::get_trace().await });

    // 自动刷新：每 8 秒重新拉取
    set_interval(
        move || {
            metrics_res.refetch();
            trace_res.refetch();
        },
        std::time::Duration::from_secs(8),
    );

    // [V5.3.0] 算力体检 Action
    let (probing, set_probing) = create_signal(false);
    let on_probe = move |_| {
        set_probing.set(true);
        spawn_local(async move {
            let _ = crate::api::client::probe_models().await;
            set_probing.set(false);
            metrics_res.refetch();
        });
    };

    view! {
        <div class="page-container" style="padding: 24px; background: #f0f2f5;">
            <header class="page-header" style="background: transparent; border: none; padding: 0 0 20px 0; display: flex; justify-content: space-between; align-items: center;">
                <div>
                    <h2 style="font-size: 22px; font-weight: 700; color: #1a1a1a; margin: 0;">"稽核业务数据概览"</h2>
                    <p style="font-size: 13px; color: #909399; margin: 4px 0 0 0;">"实时算力消耗与推演轨迹监控"</p>
                </div>
                <div style="display: flex; align-items: center; gap: 16px;">
                    // [V4.9.6] Memory Timeline 跳转按钮
                    <A href="/memory-auditor" attr:style="display: inline-flex; align-items: center; gap: 8px; padding: 8px 16px; background: white; color: #606266; border: 1px solid #dcdfe6; border-radius: 6px; font-size: 13px; font-weight: 600; text-decoration: none; box-shadow: 0 1px 4px rgba(0,0,0,0.05); transition: all 0.2s;">
                        <span>"⏳"</span>
                        <span>"记忆时间轴"</span>
                    </A>
                    // MemPalace 跳转按钮
                    <A href="/palace" attr:style="display: inline-flex; align-items: center; gap: 8px; padding: 8px 16px; background: linear-gradient(135deg, #0ea5e9, #3b82f6); color: white; border-radius: 6px; font-size: 13px; font-weight: 600; text-decoration: none; box-shadow: 0 2px 8px rgba(14,165,233,0.4);">
                        <span>"🧠"</span>
                        <span>"查看证据图谱"</span>
                    </A>

                    // [V5.3.0] 算力自检按钮
                    <button 
                        on:click=on_probe
                        disabled=probing
                        style="display: inline-flex; align-items: center; gap: 8px; padding: 8px 16px; background: #fff; color: #67c23a; border: 1px solid #67c23a; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer; transition: all 0.2s;"
                        class=move || if probing.get() { "btn-loading" } else { "" }
                    >
                        <span>{move || if probing.get() { "⌛" } else { "💊" }}</span>
                        <span>{move || if probing.get() { "体检中..." } else { "算力自检" }}</span>
                    </button>

                    <div style="display: flex; align-items: center; gap: 8px;">
                        <span class="typing-dot" style="background: #67c23a; width: 6px; height: 6px; box-shadow: 0 0 8px #67c23a;"></span>
                        <span style="font-size: 12px; color: #67c23a; font-weight: 600;">"LIVE"</span>
                    </div>
                </div>
            </header>

            // ---- 第一行：总览卡片（含今日调用次数 RPD）----
            <Suspense fallback=move || view! { <div style="color: #909399;">"加载指标中..."</div> }>
                {move || metrics_res.get().map(|res| match res {
                    Ok(m) => {
                        let total_daily = m.total_daily_tokens;
                        let total_lifetime = m.total_lifetime_tokens;
                        let total_cost: f64 = m.models.iter().map(|x| x.estimated_cost).sum();
                        let model_count = m.models.len();
                        let total_requests = m.total_daily_requests;

                        view! {
                            <div class="dashboard-grid">
                                <div class="stat-widget" style="border-left: 4px solid #1890ff;">
                                    <div class="stat-label">"今日 Token 消耗"</div>
                                    <div class="stat-value" style="color: #1890ff;">{format_tokens(total_daily)}</div>
                                </div>
                                <div class="stat-widget" style="border-left: 4px solid #52c41a;">
                                    <div class="stat-label">"累计 Token 总量"</div>
                                    <div class="stat-value" style="color: #52c41a;">{format_tokens(total_lifetime)}</div>
                                </div>
                                <div class="stat-widget" style="border-left: 4px solid #faad14;">
                                    <div class="stat-label">"今日预估成本"</div>
                                    <div class="stat-value" style="color: #faad14;">{format!("¥ {:.4}", total_cost)}</div>
                                </div>
                                <div class="stat-widget" style="border-left: 4px solid #13c2c2;">
                                    <div class="stat-label">"今日调用次数 (RPD)"</div>
                                    <div class="stat-value" style="color: #13c2c2;">{total_requests.to_string()}</div>
                                </div>
                                <div class="stat-widget" style="border-left: 4px solid #722ed1;">
                                    <div class="stat-label">"并网模型数"</div>
                                    <div class="stat-value" style="color: #722ed1;">{model_count.to_string()}</div>
                                </div>
                            </div>
                        }.into_view()
                    },
                    Err(e) => view! { <div style="color: #f5222d;">{format!("指标加载失败: {}", e)}</div> }.into_view()
                })}
            </Suspense>

            <div style="display: grid; grid-template-columns: 2fr 1fr; gap: 24px;">
                // ---- 左侧：模型用量表（含 RPD / RPM 维度）----
                <div class="card" style="padding: 0;">
                    <div style="padding: 16px 24px; border-bottom: 1px solid #f0f0f0; font-weight: 600; font-size: 15px; display: flex; justify-content: space-between; align-items: center;">
                        <span>"模型算力消耗明细"</span>
                        <span style="font-size: 11px; color: #909399;">"每 8 秒自动刷新"</span>
                    </div>
                    <Suspense fallback=move || view! { <div style="padding: 24px; color: #909399;">"加载中..."</div> }>
                        {move || metrics_res.get().map(|res| match res {
                            Ok(m) => {
                                view! {
                                    <table class="data-table">
                                        <thead>
                                            <tr>
                                                <th>"模型"</th>
                                                <th>"供应商"</th>
                                                <th>"今日用量"</th>
                                                <th>"配额"</th>
                                                <th>"使用率"</th>
                                                <th>"调用(RPD)"</th>
                                                <th>"RPM"</th>
                                                <th>"预估成本"</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {m.models.into_iter().map(|mdl| {
                                                let pct = mdl.usage_pct;
                                                let bar_color = if pct > 80.0 { "#f5222d" } else if pct > 50.0 { "#faad14" } else { "#52c41a" };
                                                let rpm_pct = if mdl.rpm_limit > 0 { mdl.current_rpm as f64 / mdl.rpm_limit as f64 * 100.0 } else { 0.0 };
                                                let rpm_color = if rpm_pct > 80.0 { "#f5222d" } else if rpm_pct > 50.0 { "#faad14" } else { "#52c41a" };
                                                let rpd_pct = if mdl.rpd_limit > 0 { mdl.daily_requests as f64 / mdl.rpd_limit as f64 * 100.0 } else { 0.0 };
                                                let rpd_color = if rpd_pct > 80.0 { "#f5222d" } else { "#606266" };
                                                view! {
                                                    <tr>
                                                        <td style="font-weight: 600;">{mdl.name.clone()}</td>
                                                        <td><span class="status-badge status-init">{mdl.provider.clone()}</span></td>
                                                        <td>{format_tokens(mdl.daily_used)}</td>
                                                        <td style="color: #909399;">{format_tokens(mdl.daily_quota)}</td>
                                                        <td>
                                                            <div style="display: flex; align-items: center; gap: 8px;">
                                                                <div style="flex: 1; height: 8px; background: #f0f0f0; border-radius: 4px; overflow: hidden;">
                                                                    <div style={format!("width: {}%; height: 100%; background: {}; border-radius: 4px; transition: width 0.6s ease;", pct.min(100.0), bar_color)}></div>
                                                                </div>
                                                                <span style="font-size: 12px; font-weight: 600; min-width: 40px;">{format!("{:.1}%", pct)}</span>
                                                            </div>
                                                        </td>
                                                        // 今日调用 RPD
                                                        <td><span style={format!("font-size: 12px; font-weight: 600; color: {};", rpd_color)}>{format!("{}/{}", mdl.daily_requests, mdl.rpd_limit)}</span></td>
                                                        // 当前 RPM 指示灯
                                                        <td>
                                                            <div style="display: flex; align-items: center; gap: 6px;">
                                                                <div style={format!("width: 8px; height: 8px; border-radius: 50%; background: {}; box-shadow: 0 0 4px {};", rpm_color, rpm_color)}></div>
                                                                <span style="font-size: 12px;">{format!("{}/{}", mdl.current_rpm, mdl.rpm_limit)}</span>
                                                            </div>
                                                        </td>
                                                        <td style="color: #faad14; font-weight: 500;">{format!("¥{:.4}", mdl.estimated_cost)}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                }.into_view()
                            },
                            Err(_) => view! { <div style="padding: 24px; color: #f5222d;">"加载失败"</div> }.into_view()
                        })}
                    </Suspense>
                </div>

                // ---- 右侧：推演轨迹面板 ----
                <div class="card" style="padding: 0;">
                    <div style="padding: 16px 24px; border-bottom: 1px solid #f0f0f0; font-weight: 600; font-size: 15px;">
                        "最近推演轨迹"
                    </div>
                    <Suspense fallback=move || view! { <div style="padding: 24px; color: #909399;">"等待推演数据..."</div> }>
                        {move || trace_res.get().map(|res| match res {
                            Ok(t) => {
                                if t.total_nodes == 0 {
                                    return view! { <div style="padding: 24px; color: #909399; text-align: center;">"暂无推演记录，请先发起一次稽核查询"</div> }.into_view();
                                }
                                let total_ms = t.total_ms;
                                view! {
                                    <div style="padding: 16px 24px;">
                                        <div style="display: flex; justify-content: space-between; margin-bottom: 16px; font-size: 13px;">
                                            <span style="color: #606266;">"执行节点: " <strong>{t.total_nodes.to_string()}</strong></span>
                                            <span style="color: #1890ff; font-weight: 600;">{format!("总耗时: {}ms", total_ms)}</span>
                                        </div>
                                        <div style="display: flex; flex-direction: column; gap: 8px;">
                                            {t.nodes.into_iter().map(|n| {
                                                let dur = n.duration_ms.unwrap_or(0);
                                                let pct = if total_ms > 0 { dur as f64 / total_ms as f64 * 100.0 } else { 0.0 };
                                                let (color, icon) = match n.node.as_str() {
                                                    "SUPERVISOR" => ("#1890ff", "🎯"),
                                                    "DATA_EXPERT" => ("#f59e0b", "🔍"),
                                                    "AUDITOR" => ("#ef4444", "⚖️"),
                                                    "FINANCIAL_EXPERT" => ("#22c55e", "💰"),
                                                    "TOOLS" | "TOOL" => ("#8b5cf6", "🔧"),
                                                    _ => ("#64748b", "⚙️"),
                                                };
                                                let status_badge = match n.status.as_str() {
                                                    "success" | "completed" => "✓",
                                                    "running" => "⏳",
                                                    "breaker" => "🛑",
                                                    _ => "?",
                                                };
                                                view! {
                                                    <div style="display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: #fafafa; border-radius: 6px; border-left: 3px solid; border-color: {color};">
                                                        <span style="font-size: 16px;">{icon}</span>
                                                        <div style="flex: 1;">
                                                            <div style="font-size: 13px; font-weight: 600; color: #303133;">{n.node.clone()}</div>
                                                            <div style="height: 4px; background: #e5e7eb; border-radius: 2px; margin-top: 4px;">
                                                                <div style={format!("width: {}%; height: 100%; background: {}; border-radius: 2px; transition: width 0.4s;", pct.min(100.0), color)}></div>
                                                            </div>
                                                        </div>
                                                        <span style="font-size: 12px; color: #909399; font-family: monospace; min-width: 60px; text-align: right;">{format!("{}ms", dur)}</span>
                                                        <span style="font-size: 14px;">{status_badge}</span>
                                                    </div>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_view()
                            },
                            Err(_) => view! { <div style="padding: 24px; color: #f5222d;">"轨迹加载失败"</div> }.into_view()
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

fn format_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.2}M t", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k t", n as f64 / 1_000.0)
    } else {
        format!("{} t", n)
    }
}
