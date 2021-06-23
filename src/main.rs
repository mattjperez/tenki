use dotenv::dotenv;
use serde_json;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Default)]
struct Local {
    ip_addr: String,
    city: String,
    country: String,
    postal: String,
}

impl Local {
    fn new() -> Self {
        let mut local = Local::default();
        local.set_ip();
        local.set_local_stats();
        local
    }

    async fn set_ip(&mut self) {
        let resp = reqwest::get("https://httpbin.org/ip")
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();
        let ip = resp.get("origin").unwrap();
        println!("{}", &ip);
        self.ip_addr = ip.to_owned();
    }

    async fn set_local_stats(&mut self) {
        let url = format!("https://ipinfo.io/{}", self.ip_addr);
        let r = reqwest::get(url)
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();

        self.city = r.get("city").unwrap().to_owned();
        self.country = r.get("country").unwrap().to_owned();
        self.postal = r.get("postal").unwrap().to_owned();
    }
}

/*
async fn get_user_ip() -> Result<HashMap<String, String>, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();
    println!("{:#?}", &resp);
    Ok(resp)
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env variables to environment
    dotenv().ok();

    let api_key = env::var("OWM_API_KEY")?;

    let local = Local::new();
    println!("{:?}", local);
    /*
    let ip = get_user_ip().await?;
    let ip = ip.get("origin").unwrap();

    let input = format!("https://ipinfo.io/{}", ip);
    let loc = reqwest::get(input)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let city = loc.get("city").unwrap();
    let country = loc.get("country").unwrap();
    */

    let weather_url = &format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
        local.city, local.country, api_key
    );
    println!("{}", weather_url);

    let weather_res = reqwest::get(weather_url)
        .await?
        .json::<HashMap<String, serde_json::Value>>()
        .await?;

    let stats = weather_res.get("main");
    println!("{:#?}", stats);

    let weather = weather_res.get("weather");
    println!("{:#?}", weather);

    Ok(())
}
