use std::collections::HashMap;
use crate::model::login::*;
use crate::model::user::*;
use crate::model::playlist::*;
use crate::util::http::CloudMusic;

lazy_static::lazy_static! {
    pub static ref CLIENT: CloudMusic = CloudMusic::default();
}

pub fn qr_login_unikey() -> QrcodeUnikey {
    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = CLIENT.post("/weapi/login/qrcode/unikey", &mut params);
    serde_json::from_str(&res).unwrap()
}

pub fn user_profile() -> UserProfile {
    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = CLIENT.post("/weapi/nuser/account/get", &mut params);
    let res: GetUserRes = serde_json::from_str(&res).unwrap();
    res.profile
}

pub fn user_playlist(uid: &str) -> Vec<PlayList> {
    let mut params= HashMap::<String, String>::new();
    params.insert("uid".to_string(), uid.to_string());
    params.insert("limit".to_string(), 30.to_string());
    params.insert("offset".to_string(), 0.to_string());
    params.insert("includeVideo".to_string(), true.to_string());
    let res = CLIENT.post("/weapi/user/playlist", &mut params);
    let res: GetPlayListsRes = serde_json::from_str(&res).unwrap();
    res.playlist
}

pub fn playlist_detail(playlist_id: &str) -> PlayList {
    let mut params= HashMap::<String, String>::new();
    params.insert("id".to_string(), playlist_id.to_string());
    params.insert("limit".to_string(), 10.to_string());
    params.insert("offset".to_string(), 0.to_string());
    let res = CLIENT.post("/weapi/v6/playlist/detail", &mut params);
    let playlist_detail: GetPlayListDetailRes = serde_json::from_str(&res).unwrap();
    playlist_detail.playlist
}