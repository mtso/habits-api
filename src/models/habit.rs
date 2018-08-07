use chrono::{DateTime, Utc};
use ksuid::Ksuid;
use rocket_contrib::{Json, JsonValue};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Habit {
    pub id: Ksuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub checks: HashSet<String>,
    pub timezone_offset: i32,
}

impl Habit {
    pub fn new(user_id: String, timezone_offset: i32) -> Self {
        Habit {
            id: Ksuid::generate(),
            user_id,
            checks: HashSet::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            timezone_offset,
        }
    }

    pub fn to_external(&self) -> Json<JsonValue> {
        Json(json!({
            "id": self.id.to_base62(),
            "checks": self.checks,
            "user_id": self.user_id,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "timezone_offset": self.timezone_offset,
        }))
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Habit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.user_id.hash(state);
    }
}
