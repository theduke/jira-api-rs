#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarUrl {
    pub _16x16: String,
    pub _24x24: String,
    pub _32x32: String,
    pub _48x48: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarSystem {
    pub id: String,
    #[serde(rename = "isDeletable")]
    pub is_deletable: bool,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
    #[serde(rename = "isSystemAvatar")]
    pub is_system_avatar: bool,
    pub owner: String,
    pub selected: bool,
    pub urls: AvatarUrl,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarResponse {
    pub system: Vec<AvatarSystem>,
}
