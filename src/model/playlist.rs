use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayList {
    pub id: i64,
    pub name: String,
    pub tracks: Option<Vec<Track>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPlayListsRes {
    pub code: i64,
    pub playlist: Vec<PlayList>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPlayListDetailRes {
    pub code: i64,
    pub playlist: PlayList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetPlayerInfoRes {
    pub code: i64,
    pub data: Vec<PlayerInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: i64,
    pub br: i64,
    pub size: i64,
    pub url: String,
    pub level: String,
    pub encodeType: String,
}