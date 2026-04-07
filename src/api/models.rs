use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WrapperResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PageVO<T> {
    pub condition: T,
    pub page_num: i32,
    pub page_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub data: Vec<T>,
    pub record_counts: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTask {
    pub id: Option<i32>,
    pub task_id: Option<i32>,
    pub main_task_code: Option<String>,
    pub source: Option<String>,
    pub data_id: Option<String>,
    pub audit_no: Option<String>,
    pub audit_object_type: Option<String>,
    pub audit_object_code: Option<String>,
    pub inspection_no: Option<String>,
    pub inspection_name: Option<String>,
    pub inspection_status: Option<i32>,
    pub audit_desc: Option<String>,
    pub assign_time: Option<String>,
    pub expire_time: Option<String>,
    pub create_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewInspTaskDTO {
    pub id: i32,
    pub inspection_no: String,
    pub inspection_name: String,
    pub inspection_status: i32,
    pub source_type: i32,
    pub task_id: i32,
    pub region_code: Option<String>,
    pub status_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksReq {
    pub inspection_no: Option<String>,
    pub inspection_name: Option<String>,
    pub inspection_status: Option<i32>,
}

// Administrative Punishment Models
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdmPunishTask {
    pub id: Option<i32>,
    pub punish_no: Option<String>,
    pub audit_task_no: Option<String>,
    pub target_name: Option<String>,
    pub punish_status: Option<i32>, // 1000, 1100, etc.
    pub filing_user: Option<String>,
    pub filing_time: Option<String>,
    pub violation_amount: Option<f64>,
}

// Dynamic Form Models
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNoteTemplateVO {
    pub id: Option<i32>,
    pub note_category: Option<String>,
    pub note_template_name: Option<String>,
    pub note_gen_name: Option<String>,
    pub note_tag: Option<String>,
    #[serde(default)]
    pub attrs: Vec<InspectionTasksNoteAttrVO>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNoteAttrVO {
    pub id: Option<i32>,
    pub template_id: Option<i32>,
    pub field_type: Option<i32>,
    pub field_name: Option<String>,
    pub field_attr: Option<String>,
    pub field_class: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNoteAttrValVO {
    pub id: Option<i32>,
    pub inspection_id: Option<i32>,
    pub template_id: Option<i32>,
    pub field_type: Option<i32>,
    pub field_name: Option<String>,
    pub field_attr: Option<String>,
    pub field_class: Option<String>,
    pub field_value: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNoteAttrListVO {
    pub template_id: i32,
    pub field_name: Option<String>,
    pub task_attr_list: Vec<InspectionTasksNoteAttrValVO>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNotePunishSubmitReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNotePunishSubmitSubReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
    pub legal_audit: i32, // 是否法制审核
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsInquiredReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
    pub compulsory_measure: i32, // 是否采取行政强制措施
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsAppealReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
    pub audit_conclusion: i32, 
    pub audit_opinion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsRecheckReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
    pub recheck: i32,  // 是否复议: 1 是 0 否
    pub transfer: i32, // 是否移交: 1 是 0 否
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksLedgerVO {
    pub id: i32,
    pub case_no: String,
    pub hospital_name: String,
    pub case_origin: String, // 案源类别
    pub total_amount: f64,   // 涉案金额
    pub status: String,      // 当前环节
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypicalCaseVO {
    pub id: i32,
    pub title: String,
    pub violation_type: String, // 违规类型
    pub background: String,     // 案件背景
    pub method: String,         // 违规手段
    pub result: String,         // 查处结果
    pub lessons: String,        // 案例启示
    pub image_url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyValVO {
    pub key: String,
    pub val: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsResultUploadReq {
    pub inspection_id: i32,
    pub template_id: i32,
    pub reqs: Vec<InspectionTasksNoteAttrValVO>,
    pub legal_audit: i32,
    pub transfor: i32, // Note: Typo 'transfor' matches Java DTO
    pub notice: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksNoteAttrValReq { // For batch add
    pub id: Option<i32>,
    pub inspection_id: i32,
    pub template_id: i32,
    pub field_name: String,
    pub field_attr: String,
    pub field_class: Option<String>,
    pub field_value: String,
    pub field_type: i32,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchAddReq {
    pub inspection_id: i32,
    pub list: Vec<InspectionTasksNoteAttrValReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspPrecheckReq {
    pub inspection_id: Option<i32>,
    pub case_repeat_flag: Option<i32>,
    pub case_repeat_id: Option<i32>,
    pub event: Option<String>,
    pub checked_reason: Option<String>,
    pub punish_method: Option<i32>,
    pub result: Option<InspectionTasksResultReq>,
    pub punish_submit: Option<InspectionTasksNotePunishSubmitReq>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionReq {
    pub inspection_id: Option<i32>,
    pub opinion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InspectionTasksResultReq {
    pub id: Option<i32>,
    pub inspection_id: Option<i32>,
    pub fixmedins_code: String,
    pub fixmedins_name: String,
    pub violation_found: Option<i32>,
    pub basis_for_penalty: Option<String>,
    pub penalty_recommendation: Option<String>,
    pub investigating_personnel: Option<String>,
}
