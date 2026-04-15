// weather_models.rs

use serde::{Deserialize, Serialize};

// ======================================================
// INTERNAL MODELS (used by your app + personality engine)
// ======================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherStatus {
    pub temp: f64,
    pub humidity: i32,
    pub pressure: f64,
    pub wind_speed: f64,
    pub city: String,
    pub current_alert: Option<WeatherAlert>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherAlert {
    pub id: String,
    pub event: String,        // "Tornado Warning"
    pub severity: AlertSeverity,
    pub description: String,  // Full NWS text
    pub instruction: String,  // Safety steps
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    Unknown,
    Minor,
    Moderate,
    Severe,   // Severe Thunderstorm Warning
    Extreme,  // Tornado/Hurricane Warning → triggers alarm
}

// ======================================================
// RAW NWS API MODELS (exactly match NWS JSON structure)
// ======================================================

// ---------- NWS POINTS ENDPOINT ----------
#[derive(Deserialize, Debug)]
pub struct NwsPointResponse {
    pub properties: NwsPointProperties,
}

#[derive(Deserialize, Debug)]
pub struct NwsPointProperties {
    pub gridId: String,
    pub gridX: u32,
    pub gridY: u32,
}

// ---------- NWS FORECAST GRID DATA ----------
#[derive(Deserialize, Debug)]
pub struct NwsForecastResponse {
    pub properties: NwsForecastProperties,
}

#[derive(Deserialize, Debug)]
pub struct NwsForecastProperties {
    pub temperature: NwsValue,
    pub relativeHumidity: NwsValue,
    pub windSpeed: NwsValue,
    pub barometricPressure: Option<NwsValue>,
}

#[derive(Deserialize, Debug)]
pub struct NwsValue {
    pub value: Option<f64>,
}

// ---------- NWS ALERTS ----------
#[derive(Deserialize, Debug, Clone)]
pub struct NwsAlertResponse {
    pub features: Vec<NwsAlertFeature>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NwsAlertFeature {
    pub id: String,
    pub properties: NwsAlertProperties,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NwsAlertProperties {
    pub event: Option<String>,
    pub severity: Option<String>,
    pub urgency: Option<String>,
    pub certainty: Option<String>,
    pub description: Option<String>,
    pub instruction: Option<String>,
}
