mod util;
mod model;
mod api;

use std::borrow::Borrow;
use std::process::Command;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

use qrcode_generator::QrCodeEcc;
use crate::api::*;

use crate::model::login::*;
use crate::model::user::*;
use crate::model::playlist::*;

use crate::util::Encrypt;
use crate::util::http::CloudMusic;

#[derive(Clone, Debug)]
struct GlobalState {
    pub user_profile: Option<UserProfile>,
    pub playlists: Vec<PlayList>
}

fn main(){
    let mut global_state = GlobalState { user_profile: None , playlists: vec!()};
    let client = CloudMusic::default();
    let needLogin = false;
    if needLogin {
        let mut params= HashMap::<String, String>::new();
        params.insert("type".to_string(), "1".to_string());
        let res = client.post("/weapi/login/qrcode/unikey", &mut params);
        let qrkey: QrcodeUnikey = serde_json::from_str(&res).unwrap();

        // Encode some data into bits.
        let code = format!("/login?codekey={}", &qrkey.unikey);
        qrcode_generator::to_png_to_file(code, QrCodeEcc::Low, 1024, "qrcode.png").unwrap();
        Command::new("sh").arg("-c").arg("xdg-open tests/data/qrcode.png").output().expect("sh exec error!");


        for i in 1..20 {
            thread::sleep(Duration::from_secs(2));
            let mut params= HashMap::<String, String>::new();
            params.insert("type".to_string(), "1".to_string());
            params.insert("key".to_string(), qrkey.unikey.clone());
            let res = client.post("/weapi/login/qrcode/client/login", &mut params);
            let qr_check: QrcodeCheck = serde_json::from_str(&res).unwrap();
            println!("check[{}], message: {:?}", i, &qr_check.message);
            if qr_check.code == 803 {
                break
            }
        }
    }

    let user_profile = user_profile();
    global_state.user_profile = Some(user_profile);

    let mut playlist = user_playlist(global_state.user_profile.clone().unwrap().userId.to_string().borrow());
    global_state.playlists.append(&mut playlist);
    println!("{:?}", &global_state);

    let playlist_detail = playlist_detail(&global_state.playlists[0].id.to_string());

    let first_song = &playlist_detail.tracks.unwrap()[0];
    println!("first song {} {}", first_song.name, first_song.id);

    let mut params= HashMap::<String, String>::new();
    params.insert("ids".to_string(), format!("[{}]", first_song.id));
    params.insert("br".to_string(), "999000".to_string());
    let res = client.post("/weapi/song/enhance/player/url", &mut params);
    let res: GetPlayerInfoRes = serde_json::from_str(&res).unwrap();
    let player_info = &res.data[0];
    println!("song url {:?}", player_info);


}

