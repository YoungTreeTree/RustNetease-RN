use std::collections::HashMap;
use crate::model::login::*;
use crate::model::user::*;
use crate::model::playlist::*;
use crate::util::http::CloudMusic;

lazy_static::lazy_static! {
    pub static ref CLIENT: CloudMusic = CloudMusic::default();
}

pub async fn qr_login_unikey() -> QrcodeUnikey {
    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = CLIENT.post("/weapi/login/qrcode/unikey", &mut params);
    serde_json::from_str(&res.await).unwrap()
}

pub async fn user_profile() -> UserProfile {
    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = CLIENT.post("/weapi/nuser/account/get", &mut params);
    let res: GetUserRes = serde_json::from_str(&res.await).unwrap();
    res.profile
}

pub async fn user_playlist(uid: &str) -> Vec<PlayList> {
    let mut params= HashMap::<String, String>::new();
    params.insert("uid".to_string(), uid.to_string());
    params.insert("limit".to_string(), 30.to_string());
    params.insert("offset".to_string(), 0.to_string());
    params.insert("includeVideo".to_string(), true.to_string());
    let res = CLIENT.post("/weapi/user/playlist", &mut params);
    let res: GetPlayListsRes = serde_json::from_str(&res.await).unwrap();
    res.playlist
}

pub async fn playlist_detail(playlist_id: &str) -> PlayList {
    let mut params= HashMap::<String, String>::new();
    params.insert("id".to_string(), playlist_id.to_string());
    params.insert("n".to_string(), 100000.to_string());
    params.insert("s".to_string(), 0.to_string());
    let res = CLIENT.post("/weapi/v6/playlist/detail", &mut params);
    let playlist_detail: GetPlayListDetailRes = serde_json::from_str(&res.await).unwrap();
    playlist_detail.playlist
}

pub async fn player_info(track_id: &str) -> PlayerInfo {
    let mut params= HashMap::<String, String>::new();
    params.insert("ids".to_string(), format!("[{}]", track_id));
    params.insert("br".to_string(), "999000".to_string());
    let res = CLIENT.post("/weapi/song/enhance/player/url", &mut params);
    let s = &res.await;
    //println!("{}", s);
    let res: GetPlayerInfoRes = serde_json::from_str(s).unwrap();
    res.data[0].to_owned()
}