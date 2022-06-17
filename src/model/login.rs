use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QrcodeUnikey {
    pub unikey: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QrcodeCheck {
    pub message: String,
    pub code: i32,
}