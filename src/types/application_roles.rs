#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationRole {
    pub key: String,
    pub groups: Vec<String>,
    pub name: String,
    #[serde(rename = "defaultGroups")]
    pub default_groups: Vec<String>,
    #[serde(rename = "selectedByDefault")]
    pub selected_by_default: bool,
    pub defined: bool,
    #[serde(rename = "numberOfSeats")]
    pub number_of_seats: u64,
    #[serde(rename = "remainingSeats")]
    pub remaining_seats: u64,
    #[serde(rename = "userCount")]
    pub user_count: u64,
    #[serde(rename = "userCountDescription")]
    pub user_count_description: String,
    #[serde(rename = "hasUnlimitedSeats")]
    pub has_unlimited_seats: bool,
    pub platform: bool,
}
