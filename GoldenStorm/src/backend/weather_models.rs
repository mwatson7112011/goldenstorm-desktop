use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiWeatherState {
    pub temperature: f32,
    pub condition: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAlert {
    pub severity: String,
    pub headline: String,
    pub description: String,
    pub expires_at: Option<String>,
}
