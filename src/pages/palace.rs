use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = initMemPalace)]
    fn init_mem_palace(id: &str);

    #[wasm_bindgen(js_name = initiateAuditSequence)]
    fn initiate_audit_sequence();

    #[wasm_bindgen(js_name = closeDataViewer)]
    fn close_data_viewer();

    // [V4.9.6] 新增：将后端图谱数据注入到 JS 3D 场景
    #[wasm_bindgen(js_name = loadEvidenceGraph)]
    fn load_evidence_graph(graph_json: &str);
}

#[component]
pub fn PalacePage() -> impl IntoView {
    // 从后端拉取证据图谱数据
    let graph_res = create_resource(|| (), |_| async {
        crate::api::client::get_palace_graph().await
    });

    // 挂载时初始化 3D 场景
    create_effect(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            leptos::logging::log!("MemPalace 3D Page Mount: Triggering JS init...");
            init_mem_palace("canvas-container");
        }
    });

    // 当图谱数据加载完成后，注入到 JS 场景
    create_effect(move |_| {
        if let Some(Ok(graph)) = graph_res.get() {
            #[cfg(target_arch = "wasm32")]
            {
                if let Ok(json_str) = serde_json::to_string(&graph) {
                    leptos::logging::log!("MemPalace: Injecting evidence graph...");
                    load_evidence_graph(&json_str);
                }
            }
        }
    });

    view! {
        <div class="palace-wrapper" style="width: 100%; height: 100%; background: #020617; position: relative; overflow: hidden; flex: 1; display: flex; flex-direction: column;">

            <style>
                {r#"
                .header-bar { position: absolute; top: 0; left: 0; width: 100%; padding: 20px; display: flex; justify-content: space-between; align-items: center; pointer-events: none; z-index: 100; background: linear-gradient(to bottom, rgba(2,6,23,0.8), transparent); }
                .step-panel { position: absolute; bottom: 40px; left: 40px; width: 320px; pointer-events: auto; z-index: 110; }
                .graph-legend-panel { position: absolute; bottom: 40px; right: 40px; width: 240px; pointer-events: auto; z-index: 110; }
                .hud-card { background: rgba(15, 23, 42, 0.9); border: 1px solid rgba(56, 189, 248, 0.3); padding: 20px; border-radius: 4px; backdrop-filter: blur(16px); box-shadow: 0 20px 50px rgba(0,0,0,0.5); }
                .step-item { display: flex; align-items: flex-start; gap: 12px; margin-bottom: 12px; opacity: 0.4; transition: all 0.5s; font-size: 11px; }
                .step-item.active { opacity: 1; transform: translateX(10px); }
                .step-num { background: #38bdf8; color: #000; width: 18px; height: 18px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold; flex-shrink: 0; }
                #data-viewer { position: fixed; top: 50%; right: 40px; transform: translateY(-50%); width: 450px; display: none; z-index: 200; pointer-events: auto; }
                .scroll-paper { background: #fdf6e3; border: 12px solid #5d3a1a; padding: 30px; color: #432b12; box-shadow: 0 0 100px rgba(0,0,0,0.8); position: relative; }
                .code-block { background: #002b36; color: #268bd2; padding: 12px; border-radius: 4px; font-family: 'JetBrains Mono', monospace; font-size: 12px; margin: 15px 0; border: 1px solid #073642; }
                .btn-start { background: linear-gradient(135deg, #0ea5e9, #2563eb); color: white; padding: 14px; width: 100%; border-radius: 4px; font-weight: bold; text-transform: uppercase; letter-spacing: 2px; box-shadow: 0 4px 15px rgba(37, 99, 235, 0.4); cursor: pointer; border: none; }
                .btn-start:hover { filter: brightness(1.2); }
                "#}
            </style>

            <div id="canvas-container" style="width: 100%; height: 100%;"></div>

            <div class="header-bar">
                <div>
                    <h1 class="text-2xl font-bold tracking-tighter text-sky-400">"MEMPALACE 3D" <span class="text-xs font-normal text-slate-500 ml-2 italic">"Architecture v12.1"</span></h1>
                    <p class="text-[9px] text-slate-400 uppercase tracking-[0.4em]">"Audit Intelligence & Spatial Memory Engine"</p>
                </div>
                // 图谱状态指示器 HUD
                <div style="display: flex; align-items: center; gap: 12px;">
                    {move || graph_res.get().map(|res| match res {
                        Ok(ref g) if g.total > 0 => view! {
                            <div style="background: rgba(15,23,42,0.8); border: 1px solid rgba(34,197,94,0.4); padding: 8px 16px; border-radius: 4px; display: flex; align-items: center; gap: 10px; pointer-events: none;">
                                <div style="width: 8px; height: 8px; border-radius: 50%; background: #22c55e; box-shadow: 0 0 8px #22c55e;"></div>
                                <div>
                                    <p style="font-size: 9px; color: #64748b; text-transform: uppercase; margin: 0;">"证据图谱"</p>
                                    <p style="font-size: 11px; font-family: monospace; color: #38bdf8; margin: 0;">{format!("{} 节点 | {} 连接", g.total, g.edges.len())}</p>
                                </div>
                            </div>
                        }.into_view(),
                        _ => view! {
                            <div style="background: rgba(15,23,42,0.8); border: 1px solid rgba(100,116,139,0.3); padding: 8px 16px; border-radius: 4px; pointer-events: none;">
                                <p style="font-size: 10px; color: #64748b; margin: 0;">"请先发起稽核推演以加载证据图谱"</p>
                            </div>
                        }.into_view(),
                    })}
                    <div class="flex items-center gap-4 bg-slate-900/50 p-3 rounded border border-slate-800">
                        <div class="text-right">
                            <p class="text-[9px] text-slate-500 uppercase">"API Endpoint"</p>
                            <p class="text-[11px] font-mono text-sky-300">"127.0.0.1:18082"</p>
                        </div>
                        <div id="status-orb" class="w-3 h-3 rounded-full bg-slate-700"></div>
                    </div>
                </div>
            </div>

            // 左下角：推演步骤面板
            <div class="step-panel">
                <div class="hud-card">
                    <h2 class="text-xs font-bold text-sky-500 mb-4 uppercase tracking-widest border-b border-sky-500/20 pb-2">"检索执行序列"</h2>
                    <div id="step-v-1" class="step-item"><span class="step-num">"1"</span> <div><b>"发起查询"</b>": 激活推理主脑核心"</div></div>
                    <div id="step-v-2" class="step-item"><span class="step-num">"2"</span> <div><b>"锁定侧翼"</b>": 走向 [ClickHouse 结算档案部]"</div></div>
                    <div id="step-v-3" class="step-item"><span class="step-num">"3"</span> <div><b>"穿过大厅"</b>": 身份验证与主题隔离"</div></div>
                    <div id="step-v-4" class="step-item"><span class="step-num">"4"</span> <div><b>"进入房间"</b>": 锁定目标书架数据"</div></div>
                    <div id="step-v-5" class="step-item"><span class="step-num">"5"</span> <div><b>"开柜取档"</b>": 提取 AAAK 摘要与明细数据"</div></div>
                    <div id="step-v-6" class="step-item"><span class="step-num">"6"</span> <div><b>"隧道联想"</b>": 跨项目穿梭至 MySQL 契约馆"</div></div>

                    <button class="btn-start mt-4" on:click=move |_| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            initiate_audit_sequence();
                        }
                    }>"🚶 开启全真架构检索"</button>
                </div>
            </div>

            // 右下角：图谱图例面板
            <div class="graph-legend-panel">
                <div class="hud-card">
                    <h2 class="text-xs font-bold text-sky-500 mb-3 uppercase tracking-widest border-b border-sky-500/20 pb-2">"证据节点图例"</h2>
                    <div style="display: flex; flex-direction: column; gap: 8px; font-size: 11px; color: #94a3b8;">
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #ef4444; flex-shrink: 0;"></div><span>"医疗机构"</span></div>
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #f59e0b; flex-shrink: 0;"></div><span>"患者信息"</span></div>
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #3b82f6; flex-shrink: 0;"></div><span>"政策规则"</span></div>
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #8b5cf6; flex-shrink: 0;"></div><span>"结算记录"</span></div>
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #22c55e; flex-shrink: 0;"></div><span>"涉案金额"</span></div>
                        <div style="display: flex; align-items: center; gap: 8px;"><div style="width: 10px; height: 10px; border-radius: 50%; background: #64748b; flex-shrink: 0;"></div><span>"通用发现"</span></div>
                    </div>
                    {move || graph_res.get().map(|res| match res {
                        Ok(ref g) if g.total > 0 => view! {
                            <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid rgba(56,189,248,0.2);">
                                <p style="font-size: 10px; color: #38bdf8; font-family: monospace; margin: 0;">{format!("已加载 {} 个证据节点", g.total)}</p>
                            </div>
                        }.into_view(),
                        Ok(ref g) => view! {
                            <div style="margin-top: 12px; padding-top: 12px; border-top: 1px solid rgba(56,189,248,0.2);">
                                <p style="font-size: 10px; color: #64748b; margin: 0;">{g.message.clone().unwrap_or("暂无图谱数据".to_string())}</p>
                            </div>
                        }.into_view(),
                        Err(_) => view! { <div></div> }.into_view(),
                    })}
                </div>
            </div>

            <div id="data-viewer">
                <div class="scroll-paper">
                    <div class="flex justify-between items-center mb-4 border-b border-amber-900/20 pb-2">
                        <span class="text-[10px] font-bold uppercase tracking-widest">"MemPalace Archive Data"</span>
                        <button on:click=move |_| {
                            #[cfg(target_arch = "wasm32")]
                            { close_data_viewer(); }
                        } class="text-amber-900 opacity-50 hover:opacity-100">"✕"</button>
                    </div>
                    <h3 id="paper-title" class="text-xl font-bold mb-1">"档案标题"</h3>
                    <p id="paper-meta" class="text-[10px] text-amber-900/60 uppercase mb-4">"Location: Wing_A / Room_01 / Closet_01"</p>
                    <div class="mb-4">
                        <p class="text-[11px] font-bold uppercase text-amber-900/80">"AAAK 压缩摘要 (快速索引):"</p>
                        <div id="aaak-content" class="code-block">"--"</div>
                    </div>
                    <div>
                        <p class="text-[11px] font-bold uppercase text-amber-900/80">"Verbatim 完整记录 (一个字都不丢):"</p>
                        <div id="raw-content" class="text-sm leading-relaxed min-h-[100px] italic">
                            "正在从 127.0.0.1 调取真实卷宗..."
                        </div>
                    </div>
                    <div class="mt-6 pt-4 border-t border-amber-900/10 text-[9px] text-amber-900/40 text-center uppercase tracking-widest">
                        "Verification: Signed by AI Agent Auditor"
                    </div>
                </div>
            </div>
        </div>
    }
}
