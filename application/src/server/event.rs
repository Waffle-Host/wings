use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Event {
    pub uuid: Uuid,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
