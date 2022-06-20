use reqwest::header::{HeaderMap, CACHE_CONTROL, PRAGMA, UPGRADE_INSECURE_REQUESTS, ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use reqwest::Method;
use std::fs::File;
use std::io::Write;

pub async fn fetch_data(url: &str, file: &mut File) {
    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(PRAGMA, "no-cache".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip,deflate".parse().unwrap());
    headers.insert(
        USER_AGENT,
        "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0".parse().unwrap(),
    );
    let client = reqwest::Client::builder()
        .build().expect("builder error");
    let builder = client.request(Method::GET, url).headers(headers);
    let mut res = builder.send().await.unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        file.write(&chunk[..]).unwrap();
    }
    file.sync_all();
}
