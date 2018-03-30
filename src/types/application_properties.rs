#[derive(Default, Clone, Debug)]
pub struct ApplicationPropertyFilter {
    pub key: Option<String>,
    pub key_filter: Option<String>,
    pub permission_level: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationProperty {
    pub id: String,
    pub key: String,
    pub value: String,
    pub name: String,
    pub desc: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}
