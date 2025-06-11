use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub topic_id: Uuid,
    pub sender: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CommentRequest {
    pub topic_id: Uuid,
    pub sender: String,
    pub text: String,
}
