use serde_json;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Info {
    pub(crate) city: String,
    pub(crate) country: String,
    pub(crate) postal: u32,
}

impl Info {
    async fn new(ip_addr: &str) -> Self {
        let url = format!("https://ipinfo.io/{}", &ip_addr.clone());
        let r = reqwest::get(url)
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();

        Info {
            city: r.get("city").unwrap().to_owned(),
            country: r.get("country").unwrap().to_owned(),
            postal: r.get("postal").unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Local {
    pub(crate) ip_addr: String,
    pub(crate) info: Info,
}

impl Local {
    pub(crate) async fn new() -> Self {
        let ip_addr: String = get_ip().await;
        let info = Info::new(&ip_addr).await;
        Local { ip_addr, info }
    }
}

pub(crate) async fn get_ip() -> String {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();
    resp.get("origin").unwrap().to_string()
}
