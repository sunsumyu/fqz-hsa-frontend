use leptos::*;
use crate::api::models::TypicalCaseVO;

#[component]
pub fn CaseLibraryPage() -> impl IntoView {
    let mock_cases = vec![
        TypicalCaseVO {
            id: 1,
            title: "某医院诱导住院、虚假住院案".to_string(),
            violation_type: "欺诈骗保".to_string(),
            background: "通过大数据模型筛选，发现该院病历记录存在高度一致性...".to_string(),
            method: "由于监管漏洞，通过虚构病情、代刷医保卡方式...".to_string(),
            result: "追回医保基金 120 万元，处以 5 倍罚款，吊销协议。".to_string(),
            lessons: "加强对民营医疗机构的随机飞检力度，建立长效监控机制。".to_string(),
            image_url: None,
        },
        TypicalCaseVO {
            id: 2,
            title: "零售药房串换药品套取医保基金案".to_string(),
            violation_type: "串换项目".to_string(),
            background: "群众举报反映该药店可使用医保卡购买生活用品...".to_string(),
            method: "在结算系统将日用品伪装成甲类药品进行虚假申报。".to_string(),
            result: "解除医保定点服务协议，约谈负责人，清退违规款项。".to_string(),
            lessons: "药店进销存系统应与医保结算系统实现强一致性比对。".to_string(),
            image_url: None,
        },
    ];

    view! {
        <div class="case-library-container high-fidelity">
            <header class="library-header">
                <h2>"典型案例查询"</h2>
                <div class="search-options">
                    <input type="text" placeholder="搜索案例标题、违规关键词..." />
                    <select class="form-select">
                        <option>"全部分类"</option>
                        <option>"欺诈骗保"</option>
                        <option>"违规用药"</option>
                        <option>"串换项目"</option>
                    </select>
                </div>
            </header>

            <div class="case-grid">
                {mock_cases.into_iter().map(|case| {
                    view! {
                        <div class="case-card">
                            <div class="card-cover">
                                <i class="el-icon-picture-outline" style="font-size: 40px; color: #dcdfe6;"></i>
                            </div>
                            <div class="card-content">
                                <span class="badge">{case.violation_type}</span>
                                <h4 class="card-title">{case.title}</h4>
                                <p class="card-excerpt">{case.background}</p>
                                <div class="card-footer">
                                    <button class="btn btn-outline-primary btn-sm">"查看详情"</button>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
