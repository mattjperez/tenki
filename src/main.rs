use dotenv::dotenv;
use serde_json;
use std::collections::HashMap;
use std::env;

mod localhost;
mod openweather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load .env variables to environment
    dotenv().ok();

    let api_key = env::var("OWM_API_KEY")?;

    let local = localhost::Local::new().await;
    println!("{:?}", local);

    let weather_url = &format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
        local.info.city, local.info.country, api_key
    );
    println!("{}", weather_url);

    let res = reqwest::get(weather_url).await?;

    let j = res.json::<openweather::OWResponse>().await?;
    println!("{:?}", j);

    Ok(())
}
