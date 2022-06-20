mod util;
mod model;
mod api;
mod downloader;
mod player;

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
use downloader::fetch_data;
use std::error::Error;
use std::thread::sleep;

#[derive(Clone, Debug)]
struct GlobalState {
    pub user_profile: Option<UserProfile>,
    pub playlists: Vec<PlayList>
}

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use crate::player::Task;
use tokio::runtime::Builder;



#[tokio::main]
pub async fn main(){
    /*a();
    sleep(Duration::from_secs(300));*/
    let music_player = player::MusicPlayer::default();
    let mut global_state = GlobalState { user_profile: None , playlists: vec!()};
    let client = CloudMusic::default();
    let needLogin = false;
    if needLogin {
        let mut params= HashMap::<String, String>::new();
        params.insert("type".to_string(), "1".to_string());
        let res = client.post("/weapi/login/qrcode/unikey", &mut params).await;
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
            let res = client.post("/weapi/login/qrcode/client/login", &mut params).await;
            let qr_check: QrcodeCheck = serde_json::from_str(&res).unwrap();
            println!("check[{}], message: {:?}", i, &qr_check.message);
            if qr_check.code == 803 {
                break
            }
        }
    }

    let user_profile = user_profile().await;
    global_state.user_profile = Some(user_profile);

    let mut playlist = user_playlist(global_state.user_profile.clone().unwrap().userId.to_string().borrow()).await;
    global_state.playlists.append(&mut playlist);
    println!("{:?}", &global_state);

    let playlist_detail = playlist_detail(&global_state.playlists[0].id.to_string()).await;

    let first_song = &playlist_detail.tracks.unwrap()[0];
    println!("first song {} {}", first_song.name, first_song.id);

    let player_info = player_info(&first_song.id.borrow().to_string()).await;
    println!("song url {:?}", player_info.url);

    //fetch_data(&player_info.url).await;

    let mut d = Task{
        key: [player_info.md5.clone().borrow(), ".flac".borrow()].concat(),
        url: player_info.url.clone()
    };
    music_player.spawn_task(d).await;
    sleep(Duration::from_secs(20));
}
