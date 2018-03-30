#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Key {
    pub key: String,
    pub _self: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentPropertiesResponse {
    pub keys: Vec<Key>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentPropertyValue {
    #[serde(rename = "hipchat.room.id")]
    pub hipchat_room_id: String,
    #[serde(rename = "support.time")]
    pub support_time: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentProperty {
    pub key: String,
    pub value: CommentPropertyValue,
}
