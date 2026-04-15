use crate::backend::weather_models::{ApiAlert, ApiWeatherState};
use reqwest::Client;
use serde::Deserialize;
use chrono::Local;

// Optional import — not required, but allowed

const USER_AGENT: &str = "GoldenStormAgent/1.0 (contact: example@example.com)";

pub struct WeatherApiClient {
    client: Client,
}

impl WeatherApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Fetch current weather using Open-Meteo (free, no key)
    pub async fn fetch_weather(&self) -> Result<ApiWeatherState, Box<dyn std::error::Error>> {
        // Hardcoded for now — agent config system can override later
        let lat = 30.1588;
        let lon = -85.6602;

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code",
            lat, lon
        );

        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let data: OpenMeteoResponse = resp.json().await?;

        let temp = data.current.temperature_2m;
        let code = data.current.weather_code;
        let condition = map_weather_code(code);

        Ok(ApiWeatherState {
            temperature: temp as f32,
            condition,
            last_updated: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Fetch active alerts using NWS
    pub async fn fetch_alerts(&self) -> Result<Vec<ApiAlert>, Box<dyn std::error::Error>> {
        // Same lat/lon as above
        let lat = 30.1588;
        let lon = -85.6602;

        let url = format!(
            "https://api.weather.gov/alerts/active?point={:.4},{:.4}",
            lat, lon
        );

        let resp = self.client.get(&url).send().await?.error_for_status()?;
        let raw: NwsAlertResponse = resp.json().await?;

        let mut alerts = Vec::new();

        for feature in raw.features {
            let p = feature.properties;

            alerts.push(ApiAlert {
                severity: p.severity.unwrap_or_else(|| "Unknown".into()),
                headline: p.headline.unwrap_or_else(|| "Weather Alert".into()),
                description: p.description.unwrap_or_default(),
                expires_at: p.expires.unwrap_or(None),
            });
        }

        Ok(alerts)
    }
}

// -----------------------------
// Open-Meteo Models
// -----------------------------

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current: OpenMeteoCurrent,
}

#[derive(Deserialize)]
struct OpenMeteoCurrent {
    temperature_2m: f64,
    weather_code: i32,
}

// -----------------------------
// NWS Alert Models
// -----------------------------

#[derive(Deserialize)]
struct NwsAlertResponse {
    features: Vec<NwsAlertFeature>,
}

#[derive(Deserialize)]
struct NwsAlertFeature {
    properties: NwsAlertProperties,
}

#[derive(Deserialize)]
struct NwsAlertProperties {
    severity: Option<String>,
    headline: Option<String>,
    description: Option<String>,
    #[serde(rename = "expires")]
    expires: Option<Option<String>>,
}

// -----------------------------
// Weather Code Mapping
// -----------------------------

fn map_weather_code(code: i32) -> String {
    match code {
        0 => "Clear".into(),
        1 | 2 | 3 => "Partly Cloudy".into(),
        45 | 48 => "Fog".into(),
        51 | 53 | 55 => "Drizzle".into(),
        61 | 63 | 65 => "Rain".into(),
        71 | 73 | 75 => "Snow".into(),
        95 => "Thunderstorm".into(),
        96 | 99 => "Severe Thunderstorm".into(),
        _ => "Unknown".into(),
    }
}
