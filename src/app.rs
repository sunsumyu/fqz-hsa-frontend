use leptos::*;
use leptos_router::*;
use crate::pages::dashboard::DashboardPage;
use crate::pages::clue_audit::ClueAuditPage;
use crate::pages::audit_query::AuditQueryPage;
use crate::pages::confirm_finish::ConfirmFinishPage;
use crate::pages::punish_filing::PunishFilingPage;
use crate::pages::punish_investigation::PunishInvestigationPage;
use crate::pages::punish_decision::PunishDecisionPage;
use crate::pages::punish_execution::PunishExecutionPage;
use crate::pages::punish_legal::PunishLegalPage;
use crate::pages::punish_notice::PunishNoticePage;
use crate::pages::ledger::LedgerPage;
use crate::pages::case_library::CaseLibraryPage;
use crate::components::sidebar::Sidebar;
use crate::components::template_editor::TemplateEditor;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app-layout">
                <Sidebar />
                <main class="content-main">
                    <Routes>
                        <Route path="" view=DashboardPage/>
                        <Route path="/clue-audit" view=ClueAuditPage/>
                        <Route path="/audit-query" view=AuditQueryPage/>
                        <Route path="/confirm-finish" view=ConfirmFinishPage/>
                        <Route path="/punish-filing" view=PunishFilingPage/>
                        <Route path="/punish-investigation" view=PunishInvestigationPage/>
                        <Route path="/punish-legal" view=PunishLegalPage/>
                        <Route path="/punish-notice" view=PunishNoticePage/>
                        <Route path="/punish-decision" view=PunishDecisionPage/>
                        <Route path="/punish-execution" view=PunishExecutionPage/>
                        <Route path="/ledger" view=LedgerPage/>
                        <Route path="/case-library" view=CaseLibraryPage/>
                        <Route path="/document-edit/:id" view=AuditPage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}



#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="home-hero">
            <h1>"全生命周期稽核平台"</h1>
            <p>"欢迎使用反欺诈医保监管平台 - 稽查子系统"</p>
        </div>
    }
}

#[component]
fn AuditPage() -> impl IntoView {
    view! {
        <div class="page-container" style="padding: 0;">
            <TemplateEditor 
                inspection_id=1001 
                note_category="FILINGCASE_TO_INQUIRING".to_string() 
            />
        </div>
    }
}
#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - 页面未找到"</h1>
            <p>"抱歉，您访问的页面不存在。"</p>
            <A href="/">"返回首页"</A>
        </div>
    }
}
