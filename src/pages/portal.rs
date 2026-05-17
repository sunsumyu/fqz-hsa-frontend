use leptos::*;
use leptos_router::A;

#[component]
pub fn PortalPage() -> impl IntoView {
    view! {
        <div class="portal-container">
            <style>
                "
                .portal-container {
                    background: #020617;
                    color: #f8fafc;
                    min-height: 100vh;
                    padding: 40px;
                    font-family: 'Outfit', sans-serif;
                }
                .welcome-header { margin-bottom: 40px; }
                .portal-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 30px;
                }
                .portal-card {
                    background: rgba(30, 41, 59, 0.7);
                    backdrop-filter: blur(10px);
                    border: 1px solid #334155;
                    border-radius: 16px;
                    padding: 30px;
                    transition: 0.3s;
                    cursor: pointer;
                    text-decoration: none;
                    color: inherit;
                    display: block;
                }
                .portal-card:hover {
                    transform: translateY(-5px);
                    border-color: #38bdf8;
                    box-shadow: 0 10px 30px -10px rgba(56, 189, 248, 0.3);
                }
                .icon-box {
                    width: 50px;
                    height: 50px;
                    background: #1e293b;
                    border-radius: 12px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    margin-bottom: 20px;
                    font-size: 1.5rem;
                }
                .card-tag {
                    font-size: 0.75rem;
                    background: #0ea5e9;
                    color: white;
                    padding: 2px 8px;
                    border-radius: 4px;
                    margin-bottom: 10px;
                    display: inline-block;
                }
                .roadmap-container {
                    margin-top: 50px;
                    background: #0f172a;
                    border-radius: 16px;
                    padding: 30px;
                    border: 1px solid #1e293b;
                }
                .roadmap-grid {
                    display: flex;
                    gap: 20px;
                    margin-top: 20px;
                }
                .roadmap-step {
                    flex: 1;
                    padding: 15px;
                    border-radius: 4px;
                }
                .status-footer {
                    margin-top: 30px;
                    padding: 20px;
                    background: #020617;
                    border-radius: 12px;
                    border: 1px dashed #334155;
                }
                "
            </style>

            <div class="welcome-header">
                <h1 style="font-size: 2.5rem; margin: 0; background: linear-gradient(to right, #38bdf8, #818cf8); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">
                    "欢迎回来, 首席审计官"
                </h1>
                <p style="color: #94a3b8; margin-top: 10px;">"这是您对 HSA-Agent 全链路性能与成本的掌控中心。"</p>
            </div>

            <div class="portal-grid">
                <A href="/" class="portal-card">
                    <span class="card-tag">"LIVE"</span>
                    <div class="icon-box">"📊"</div>
                    <h3>"性能监控中心"</h3>
                    <p style="color: #94a3b8;">"查看 Benchmark 实时评分、耗时分布及上下文节省率。"</p>
                </A>

                <A href="/agent" class="portal-card">
                    <div class="icon-box">"🤖"</div>
                    <h3>"智能审计助手"</h3>
                    <p style="color: #94a3b8;">"与 HSA-Agent 协作，执行复杂的医疗欺诈建模与线索发现。"</p>
                </A>

                <A href="/palace" class="portal-card">
                    <div class="icon-box">"🧠"</div>
                    <h3>"记忆宫殿 (3D)"</h3>
                    <p style="color: #94a3b8;">"可视化审计证据链条，探索实体间的隐秘关联。"</p>
                </A>

                <div class="portal-card" on:click=|_| { /* TODO */ }>
                    <span class="card-tag">"Memory V4"</span>
                    <div class="icon-box">"🛡️"</div>
                    <h3>"租户隔离管控"</h3>
                    <p style="color: #94a3b8;">"管理多租户物理分片隔离状态，确保数据合规与安全。"</p>
                </div>
            </div>

            <div class="roadmap-container">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <h3 style="margin: 0; display: flex; align-items: center;">
                        "🏗️ 系统架构演进路线 (V4 Roadmap)"
                    </h3>
                </div>
                
                <div class="roadmap-grid">
                    <div class="roadmap-step" style="background: rgba(56, 189, 248, 0.1); border-left: 4px solid #0ea5e9;">
                        <h4 style="margin: 0 0 10px 0; color: #0ea5e9;">"Phase 1: 基础设施 (Done)"</h4>
                        <p style="font-size: 0.85rem; color: #94a3b8; margin: 0;">"Hierarchical Memory Hub, SQL Guardian"</p>
                    </div>
                    <div class="roadmap-step" style="background: rgba(168, 85, 247, 0.1); border-left: 4px solid #a855f7;">
                        <h4 style="margin: 0 0 10px 0; color: #a855f7;">"Phase 2: 企业级加固 (Active)"</h4>
                        <p style="font-size: 0.85rem; color: #94a3b8; margin: 0;">"Physical Sharding, Distributed Locking, Async Pipeline"</p>
                    </div>
                    <div class="roadmap-step" style="background: rgba(30, 41, 59, 0.5); border-left: 4px solid #475569;">
                        <h4 style="margin: 0 0 10px 0; color: #475569;">"Phase 3: 极致透明 (Planned)"</h4>
                        <p style="font-size: 0.85rem; color: #94a3b8; margin: 0;">"Multi-modal Recall, Real-time Governance"</p>
                    </div>
                </div>
            </div>

            <div class="status-footer">
                <h4 style="margin: 0; color: #38bdf8;">"🎯 记忆中枢实时状态"</h4>
                <p style="color: #94a3b8; font-size: 0.9rem; margin-top: 5px;">
                    "当前架构："{ "Physical-Sharded-V4" } " | "
                    "物理租户："{ "Active" } " | "
                    "审计节点："{ "Neo4j Cluster Ready" } " | "
                    "语义对齐率："{ "99.8%" }
                </p>
            </div>
        </div>
    }
}
