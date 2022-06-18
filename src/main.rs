mod util;
mod model;

use std::process::Command;
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

use qrcode_generator::QrCodeEcc;

use crate::model::login::{
    QrcodeUnikey, QrcodeCheck
};
use crate::model::user::{
    GetUserRes, UserProfile
};
use crate::model::playlist::{
    PlayList, GetPlayListrRes
};

use crate::util::Encrypt;
use crate::util::http::CloudMusic;
use std::borrow::Borrow;

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

    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = client.post("/weapi/nuser/account/get", &mut params);
    let res: GetUserRes = serde_json::from_str(&res).unwrap();
    println!("{:?}", &res);
    global_state.user_profile = Some(res.profile);

    let mut params= HashMap::<String, String>::new();
    params.insert("uid".to_string(), global_state.user_profile.clone().unwrap().userId.to_string());
    params.insert("limit".to_string(), 30.to_string());
    params.insert("offset".to_string(), 0.to_string());
    params.insert("includeVideo".to_string(), true.to_string());
    let res = client.post("/weapi/user/playlist", &mut params);
    let res: GetPlayListrRes = serde_json::from_str(&res).unwrap();
    println!("{:?}", &res);
    global_state.playlists.append(&mut res.playlist.to_owned());
    println!("{:?}", &global_state);


    let mut params= HashMap::<String, String>::new();
    let playlist: &PlayList = &(global_state.playlists)[0];
    params.insert("id".to_string(), playlist.id.to_string());
    params.insert("limit".to_string(), 10.to_string());
    params.insert("offset".to_string(), 0.to_string());
    let res = client.post("/weapi/v6/playlist/detail", &mut params);



    https://interface3.music.163.com/eapi/song/enhance/player/url


}

