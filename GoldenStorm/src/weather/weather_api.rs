// src/weather/weather_api.rs

use crate::weather::weather_models::*;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

// NWS requires a User-Agent header
const USER_AGENT: &str = "GoldenStorm/1.0 (contact: example@example.com)";

// =========================
// PUBLIC API SURFACE
// =========================

pub struct WeatherApi {
    client: Client,
}

impl WeatherApi {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Try to detect location via IP (ipinfo.io).
    /// Returns (lat, lon, city_name) if successful.
    pub async fn geolocate_ip(&self) -> Result<Option<(f64, f64, String)>, Box<dyn Error>> {
        let resp = self
            .client
            .get("https://ipinfo.io/json")
            .send()
            .await?
            .error_for_status()?;

        let data: IpInfoResponse = resp.json().await?;
        if let Some(loc) = data.loc {
            let parts: Vec<&str> = loc.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(lat), Ok(lon)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    let city = data.city.unwrap_or_else(|| "Unknown".to_string());
                    return Ok(Some((lat, lon, city)));
                }
            }
        }

        Ok(None)
    }

    /// Geocode a city name or ZIP using Nominatim (OpenStreetMap).
    /// Returns (lat, lon, display_name) if successful.
    pub async fn geocode_city(&self, query: &str) -> Result<Option<(f64, f64, String)>, Box<dyn Error>> {
        let url = format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
            urlencoding::encode(query)
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .error_for_status()?;

        let results: Vec<NominatimResult> = resp.json().await?;
        if let Some(first) = results.first() {
            let lat = first.lat.parse::<f64>()?;
            let lon = first.lon.parse::<f64>()?;
            let name = first.display_name.clone();
            return Ok(Some((lat, lon, name)));
        }

        Ok(None)
    }

    /// Build a full WeatherStatus for a given lat/lon + city label.
    pub async fn build_weather_status(
        &self,
        lat: f64,
        lon: f64,
        city_label: String,
    ) -> Result<WeatherStatus, Box<dyn Error>> {
        // 1. Get NWS point metadata (grid + office)
        let point = self.get_point_data(lat, lon).await?;

        // 2. Get forecast for that grid
        let forecast = self
            .get_grid_weather(&point.gridId, point.gridX, point.gridY)
            .await?;

        // 3. Get active alerts for that point
        let alert = self.fetch_active_alerts(lat, lon).await?;

        // 4. Map forecast → WeatherStatus
        let temp = forecast.temperature.value.unwrap_or(0.0);
        let humidity = forecast.relativeHumidity.value.unwrap_or(0.0) as i32;
        let pressure = forecast
            .barometricPressure
            .as_ref()
            .and_then(|v| v.value)
            .unwrap_or(1013.0);

        let wind_speed = forecast.windSpeed.value.unwrap_or(0.0);

        Ok(WeatherStatus {
            temp,
            humidity,
            pressure,
            wind_speed,
            city: city_label,
            current_alert: alert,
        })
    }

    /// Fetch the most relevant active alert for this point, if any.
    pub async fn fetch_active_alerts(
        &self,
        lat: f64,
        lon: f64,
    ) -> Result<Option<WeatherAlert>, Box<dyn Error>> {
        let url = format!(
            "https://api.weather.gov/alerts/active?point={:.4},{:.4}",
            lat, lon
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let raw: NwsAlertResponse = resp.json().await?;

        // Pick the highest severity alert, if any
        let mut best: Option<WeatherAlert> = None;

        for feature in raw.features {
            let p = feature.properties;

            let event = p
                .event
                .unwrap_or_else(|| "Unknown Alert".to_string());
            let description = p.description.unwrap_or_default();
            let instruction = p
                .instruction
                .unwrap_or_else(|| "No specific instructions provided.".to_string());

            let severity = classify_severity(
                p.severity.as_deref(),
                p.urgency.as_deref(),
                p.certainty.as_deref(),
                &event,
            );

            let alert = WeatherAlert {
                id: feature.id,
                event,
                severity,
                description,
                instruction,
            };

            best = match best {
                None => Some(alert),
                Some(existing) => {
                    if alert.severity > existing.severity {
                        Some(alert)
                    } else {
                        Some(existing)
                    }
                }
            };
        }

        Ok(best)
    }

    // =========================
    // INTERNAL NWS HELPERS
    // =========================

    async fn get_point_data(&self, lat: f64, lon: f64) -> Result<NwsPointProperties, Box<dyn Error>> {
        let url = format!("https://api.weather.gov/points/{:.4},{:.4}", lat, lon);

        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let data: NwsPointResponse = resp.json().await?;
        Ok(data.properties)
    }

    async fn get_grid_weather(
        &self,
        grid_id: &str,
        x: u32,
        y: u32,
    ) -> Result<NwsForecastProperties, Box<dyn Error>> {
        let url = format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast",
            grid_id, x, y
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let data: NwsForecastResponse = resp.json().await?;
        Ok(data.properties)
    }
}

// =========================
// IMPROVED SEVERITY CLASSIFIER
// =========================

fn classify_severity(
    severity_raw: Option<&str>,
    urgency_raw: Option<&str>,
    _certainty_raw: Option<&str>,
    event: &str,
) -> AlertSeverity {
    let event_lc = event.to_lowercase();

    // TIER 1: EMINENT DANGER (Trigger Hardware Alarm)
    if event_lc.contains("tornado warning")
        || event_lc.contains("hurricane warning")
        || event_lc.contains("extreme wind warning")
    {
        return AlertSeverity::Extreme;
    }

    // TIER 2: SIGNIFICANT THREAT (Loud Notification)
    if event_lc.contains("severe thunderstorm warning")
        || event_lc.contains("flash flood warning")
    {
        return AlertSeverity::Severe;
    }

    // TIER 3: NWS STANDARD MAPPING
    match severity_raw.unwrap_or("Unknown") {
        "Extreme" if urgency_raw == Some("Immediate") => AlertSeverity::Extreme,
        "Extreme" | "Severe" => AlertSeverity::Severe,
        "Moderate" => AlertSeverity::Moderate,
        _ => AlertSeverity::Minor,
    }
}

// =========================
// EXTERNAL API MODELS
// =========================

#[derive(Deserialize, Debug)]
struct IpInfoResponse {
    city: Option<String>,
    loc: Option<String>, // "lat,lon"
}

#[derive(Deserialize, Debug)]
struct NominatimResult {
    lat: String,
    lon: String,
    display_name: String,
}
