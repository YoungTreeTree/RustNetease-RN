use std::collections::HashMap;
use std::fs;
use reqwest::{ClientBuilder, Method};
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, REFERER, USER_AGENT,
};
use chrono::{DateTime, Local};
use openssl::hash::{hash, MessageDigest};
use reqwest::blocking::Client;
use tokio::task::spawn_blocking;
use crate::Encrypt;
use crate::util::convert_map_to_string;

const COOKIE_PATH: &str = "cookie";

pub struct CloudMusic {
    client: Client
}

impl CloudMusic {
    pub fn default() -> CloudMusic {
        let client = Client::builder()
            .gzip(true)
            .cookie_store(true)
            .build()
            .unwrap();

        CloudMusic {
            client
        }
    }

    fn store_cookies(&self, res: &reqwest::blocking::Response) {
        let cookies: Vec<String> = res
            .cookies()
            .into_iter()
            .map(|s| format!("{}={}", s.name().to_string(), s.value().to_string()))
            .collect();
        let mut c: String = cookies.into_iter().map(|s| format!("{}; ", s)).collect();
        c.pop();
        if c.len() > 0 {
            fs::write(COOKIE_PATH, &c).expect("Unable to write file");
        }
    }

    fn get_cookies(&self) -> String {
        match fs::read_to_string(COOKIE_PATH) {
            Ok(cookie) => cookie,
            Err(_) => "".to_string()
        }
    }

    pub fn post(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> String {
        let csrf_token = String::new();
        params.insert("csrf_token".to_owned(), csrf_token);
        let params = Encrypt::encrypt_login(params);
        self.internal_call(Method::POST, &url, Some(params))
    }

    pub fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> String {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, None)
        } else {
            self.internal_call(Method::GET, url, None)
        }
    }

    fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<String>,
    ) -> String {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert(ACCEPT, "*/*".parse().unwrap());
        headers.insert(REFERER, "https://music.163.com".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0"
                .parse()
                .unwrap(),
        );
        headers.insert(HOST, "music.163.com".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
        headers.insert(COOKIE, self.get_cookies().parse().unwrap());
        let res = {
            let builder = self.client.request(method, url).headers(headers);
            let builder = if let Some(data) = payload {
                builder.body(data)
            } else {
                builder
            };
            builder.send()
        };

        let res = match res {
            Ok(res) => {
                if url == "https://music.163.com/weapi/login/qrcode/client/login" {
                    self.store_cookies(&res);
                }
                res.text()
            },
            Err(error) => panic!("Problem get response: {:?}", error)
        };

        let res = match res {
            Ok(res) => res,
            Err(error) => panic!("Problem get response body: {:?}", error)
        };
        println!("{} {:?}", url, res);
        res
    }
}