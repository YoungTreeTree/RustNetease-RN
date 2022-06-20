use std::collections::HashMap;
use std::fs;
use reqwest::{Method, Client};
use reqwest::header::{
    HeaderMap, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, COOKIE, HOST, REFERER, USER_AGENT,
};
use crate::Encrypt;
use crate::util::convert_map_to_string;

const COOKIE_PATH: &str = "cookie";

pub struct CloudMusic {
    client: Client,
    host: String
}

impl CloudMusic {
    pub fn default() -> CloudMusic {
        let client = Client::builder()
            .gzip(true)
            .cookie_store(true)
            .build()
            .unwrap();
        CloudMusic {
            client,
            host: "https://music.163.com".to_string()
        }
    }

    fn store_cookies(&self, res: &reqwest::Response) {
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

    pub async fn post(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> String {
        let csrf_token = String::new();
        params.insert("csrf_token".to_owned(), csrf_token);
        let params = Encrypt::encrypt_login(params);
        self.internal_call(Method::POST, &url, Some(params)).await
    }

    pub async fn get(
        &self,
        url: &str,
        params: &mut HashMap<String, String>,
    ) -> String {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, None).await
        } else {
            self.internal_call(Method::GET, url, None).await
        }
    }

    async fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: Option<String>,
    ) -> String {
        let mut url = (*url).to_string();
        if !url.starts_with("http") {
            url = [self.host.to_owned(), url].concat()
        }

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
            let builder = self.client.request(method, &url).headers(headers);
            let builder = if let Some(data) = payload {
                builder.body(data)
            } else {
                builder
            };
            builder.send().await
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

        let res = match res.await {
            Ok(res) => res,
            Err(error) => panic!("Problem get response body: {:?}", error)
        };
        println!("{} {:?}", url, res);
        res
    }
}