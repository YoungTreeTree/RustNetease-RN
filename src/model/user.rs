use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub userId: i64,
    pub nickname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserRes {
    pub code: i64,
    pub profile: UserProfile,
}