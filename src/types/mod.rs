mod application_properties;
pub use self::application_properties::*;

mod application_roles;
pub use self::application_roles::*;

mod attachments;
pub use self::attachments::*;

mod auditing;
pub use self::auditing::*;

use chrono;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Paginated<T> {
    #[serde(rename = "startAt")]
    pub start_at: u64,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    pub total: u64,
    #[serde(rename = "isLast")]
    pub is_last: bool,
    pub values: Vec<T>,
}
