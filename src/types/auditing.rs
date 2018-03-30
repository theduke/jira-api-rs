use super::DateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedAuditRecords {
    offset: u64,
    limit: u64,
    total: u64,
    records: Vec<AuditRecord>,
}

#[derive(Default)]
pub struct AuditingFilter {
    pub filter: Option<String>,
    pub from: Option<DateTime>,
    pub offset: Option<u64>,
    pub to: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AssociatedItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "parentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "typeName")]
    pub type_name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChangedValue {
    #[serde(rename = "changedFrom")]
    pub changed_from: Option<String>,
    #[serde(rename = "changedTo")]
    pub changed_to: Option<String>,
    #[serde(rename = "fieldName")]
    pub field_name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObjectItem {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "parentName")]
    pub parent_name: Option<String>,
    #[serde(rename = "typeName")]
    pub type_name: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuditRecord {
    #[serde(default, rename = "associatedItems")]
    pub associated_items: Vec<AssociatedItem>,
    #[serde(rename = "authorKey")]
    pub author_key: Option<String>,
    pub category: String,
    #[serde(default, rename = "changedValues")]
    pub changed_values: Vec<ChangedValue>,
    pub created: String,
    pub description: Option<String>,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    pub id: i64,
    #[serde(rename = "objectItem")]
    pub object_item: ObjectItem,
    #[serde(rename = "remoteAddress")]
    pub remote_address: Option<String>,
    pub summary: String,
}
