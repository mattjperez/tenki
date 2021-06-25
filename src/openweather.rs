use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Stats {
    temp: f32,
    feels_like: f32,
    temp_max: f32,
    temp_min: f32,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Sys {
    #[serde(rename = "type")]
    systype: u16,
    id: u64,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Wind {
    speed: f32,
    deg: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Clouds {
    all: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OWResponse {
    main: Stats,
    weather: Vec<Weather>,
    coord: Coord,
    name: String,
    base: String,
    visibility: u64,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i64,
    id: u64,
    cod: u64,
}
