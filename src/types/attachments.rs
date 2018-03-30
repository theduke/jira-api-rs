#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarUrls {
    #[serde(rename = "48x48")]
    pub s48_48: String,
    #[serde(rename = "32x32")]
    pub s32_32: String,
    #[serde(rename = "24x24")]
    pub s24_24: String,
    #[serde(rename = "16x16")]
    pub s16_16: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    #[serde(rename = "self")]
    pub _self: String,
    pub key: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: AvatarUrls,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    pub id: u64,
    #[serde(rename = "self")]
    pub _self: String,
    pub filename: String,
    pub author: Author,
    pub created: String,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub content: String,
    pub thumbnail: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AttachmentMeta {
    pub enabled: bool,
    #[serde(rename = "uploadLimit")]
    pub upload_limit: u64,
}
