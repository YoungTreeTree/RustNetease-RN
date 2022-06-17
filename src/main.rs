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
use crate::util::Encrypt;
use crate::util::http::CloudMusic;

fn main(){
    let client = CloudMusic::default();
    let needLogin = false;
    if needLogin {
        let mut params= HashMap::<String, String>::new();
        params.insert("type".to_string(), "1".to_string());
        let res = client.post("https://music.163.com/weapi/login/qrcode/unikey", &mut params);
        let qrkey: QrcodeUnikey = serde_json::from_str(&res).unwrap();

        // Encode some data into bits.
        let code = format!("https://music.163.com/login?codekey={}", &qrkey.unikey);
        qrcode_generator::to_png_to_file(code, QrCodeEcc::Low, 1024, "qrcode.png").unwrap();
        Command::new("sh").arg("-c").arg("xdg-open tests/data/qrcode.png").output().expect("sh exec error!");


        for i in 1..20 {
            thread::sleep(Duration::from_secs(2));
            let mut params= HashMap::<String, String>::new();
            params.insert("type".to_string(), "1".to_string());
            params.insert("key".to_string(), qrkey.unikey.clone());
            let res = client.post("https://music.163.com/weapi/login/qrcode/client/login", &mut params);
            let qr_check: QrcodeCheck = serde_json::from_str(&res).unwrap();
            println!("check[{}], message: {:?}", i, &qr_check.message);
            if qr_check.code == 803 {
                break
            }
        }
    }

    let mut params= HashMap::<String, String>::new();
    params.insert("type".to_string(), "1".to_string());
    let res = client.post("https://music.163.com/api/nuser/account/get", &mut params);

}

