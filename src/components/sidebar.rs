use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar">
            <div class="sidebar-logo">
                "全生命周期"<span>"稽核平台"</span>
            </div>
            <nav class="menu-list">
                <div class="menu-group">
                    <div class="group-title">"智能辅助"</div>
                    <A href="/agent" class="menu-item" active_class="active">
                        <i class="el-icon-cpu"></i> "智能稽核助手"
                    </A>
                </div>
                <div class="menu-group">
                    <div class="group-title">"数据中心"</div>
                    <A href="/" exact=true class="menu-item" active_class="active">
                        <i class="el-icon-pie-chart"></i> "数据统计分析"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"稽查业务"</div>
                    <A href="/clue-audit" class="menu-item" active_class="active">
                        <i class="el-icon-search"></i> "线索稽核"
                    </A>
                    <A href="/audit-query" class="menu-item" active_class="active">
                        <i class="el-icon-view"></i> "稽查进度查询"
                    </A>
                    <A href="/case-library" class="menu-item" active_class="active">
                        <i class="el-icon-discover"></i> "典型案例查询"
                    </A>
                    <A href="/confirm-finish" class="menu-item" active_class="active">
                        <i class="el-icon-circle-check"></i> "关口办结确认"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"任务管理"</div>
                    <A href="/clue-audit" class="menu-item" active_class="active">
                        <i class="el-icon-s-check"></i> "任务审核 (预检)"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"立案管理"</div>
                    <A href="/punish-filing" class="menu-item" active_class="active">
                        <i class="el-icon-folder"></i> "待立案"
                    </A>
                    <A href="/punish-investigation" class="menu-item" active_class="active">
                        <i class="el-icon-search"></i> "调查中"
                    </A>
                    <A href="/punish-legal" class="menu-item" active_class="active">
                        <i class="el-icon-coordinate"></i> "法制审核中"
                    </A>
                    <A href="/punish-notice" class="menu-item" active_class="active">
                        <i class="el-icon-bell"></i> "待通告"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"处分执行"</div>
                    <A href="/punish-decision" class="menu-item" active_class="active">
                        <i class="el-icon-circle-check"></i> "待决定"
                    </A>
                    <A href="/punish-execution" class="menu-item" active_class="active">
                        <i class="el-icon-truck"></i> "案件处罚执行"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"权利保障与移送"</div>
                    <A href="/punish-hearing" class="menu-item" active_class="active">
                        <i class="el-icon-phone-outline"></i> "听证程序处理"
                    </A>
                    <A href="/punish-appeal" class="menu-item" active_class="active">
                        <i class="el-icon-guide"></i> "复议与诉讼"
                    </A>
                    <A href="/punish-transfer" class="menu-item" active_class="active">
                        <i class="el-icon-share"></i> "刑事案件移送"
                    </A>
                </div>

                <div class="menu-group">
                    <div class="group-title">"综合管理"</div>
                    <A href="/ledger" class="menu-item" active_class="active">
                        <i class="el-icon-notebook-1"></i> "数据台账汇总"
                    </A>
                </div>

            </nav>
        </aside>
    }
}

