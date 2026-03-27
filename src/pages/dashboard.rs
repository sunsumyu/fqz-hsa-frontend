use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="page-container">
            <header class="page-header">
                <h2>"稽核业务数据概览"</h2>
            </header>
            
            <div class="view-container">
                <div class="dashboard-grid">
                    <div class="stat-widget">
                        <div class="stat-label">"总稽查任务数"</div>
                        <div class="stat-value">"1,248"</div>
                    </div>
                    <div class="stat-widget">
                        <div class="stat-label">"待指派任务"</div>
                        <div class="stat-value">"12"</div>
                    </div>
                    <div class="stat-widget">
                        <div class="stat-label">"已违规金额"</div>
                        <div class="stat-value">"¥ 12.50M"</div>
                    </div>
                    <div class="stat-widget">
                        <div class="stat-label">"办结完成率"</div>
                        <div class="stat-value">"98.2%"</div>
                    </div>
                </div>

                <div class="card">
                    <div class="card-header" style="margin-bottom: 24px; font-weight: 500;">
                        "月度稽核任务执行趋势"
                    </div>
                    <div class="mock-chart" style="height: 300px; display: flex; align-items: flex-end; gap: 24px; padding: 20px;">
                        <div class="chart-bar" style="height: 30%; width: 40px; background: #91d5ff;"></div>
                        <div class="chart-bar" style="height: 55%; width: 40px; background: #69c0ff;"></div>
                        <div class="chart-bar" style="height: 45%; width: 40px; background: #40a9ff;"></div>
                        <div class="chart-bar" style="height: 80%; width: 40px; background: #1890ff;"></div>
                        <div class="chart-bar" style="height: 70%; width: 40px; background: #096dd9;"></div>
                        <div class="chart-bar" style="height: 95%; width: 40px; background: #0050b3;"></div>
                        <div class="chart-bar" style="height: 60%; width: 40px; background: #003a8c;"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
