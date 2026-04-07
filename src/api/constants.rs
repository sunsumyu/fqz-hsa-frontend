use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PunishStatus {
    /// 待立案 (Wait Filing)
    WaitFiling = 1000,
    /// 呈批中 (Approval in Progress)
    ApprovalInProgress = 1001,
    /// 调查中 (Investigation)
    Investigation = 1100,
    /// 强制执行中 (Forceful Execution)
    ForcefulExecution = 1105,
    /// 中止调查 (Suspended)
    Suspended = 1110,
    /// 终止调查 (Terminated)
    Terminated = 1120,
    /// 法制审核中 (Legal Review)
    LegalReview = 1200,
    /// 待告知 (Wait Notification)
    WaitNotification = 1300,
    /// 听证/申辩中 (Hearing/Appeal)
    Hearing = 1305,
    /// 处罚决定 (Decision)
    Decision = 1400,
    /// 处罚执行 (Execution)
    Execution = 1500,
    /// 结案 (Closed)
    Closed = 1600,
    /// 待结案 (Wait Close)
    WaitClose = 1700,
    /// 待预检 (Wait Pre-Audit)
    WaitPreAudit = 130,
}

impl PunishStatus {
    pub fn to_label(&self) -> &'static str {
        match self {
            Self::WaitPreAudit => "待预检",
            Self::WaitFiling => "待立案审批",
            Self::ApprovalInProgress => "呈批中",
            Self::Investigation => "调查中",
            Self::ForcefulExecution => "强制执行中",
            Self::Suspended => "中止调查",
            Self::Terminated => "终止调查",
            Self::LegalReview => "法制审核中",
            Self::WaitNotification => "待告知",
            Self::Hearing => "听证/申辩中",
            Self::Decision => "处罚决定",
            Self::Execution => "处罚执行",
            Self::Closed => "已结案",
            Self::WaitClose => "待结案归档",
        }
    }

    pub fn to_class(&self) -> &'static str {
        match self {
            Self::WaitPreAudit | Self::WaitFiling | Self::ApprovalInProgress => "status-init",
            Self::Investigation | Self::LegalReview | Self::WaitNotification | Self::Hearing => "status-process",
            Self::Decision | Self::Execution | Self::WaitClose => "status-active",
            Self::Suspended | Self::Terminated => "status-error",
            Self::Closed => "status-success",
            _ => "status-init",
        }
    }
}
