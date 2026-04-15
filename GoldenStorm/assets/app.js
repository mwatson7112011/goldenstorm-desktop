// ===============================
// GoldenStorm - app.js
// Full Personality Engine + Hybrid IPC
// ===============================

// Current UI state
let currentWeather = null;
let currentAlert = null;
let currentPersona = "Serious";
let chaosMode = false;

// ===============================
// RUST → JS EVENT HANDLERS
// Called by Rust using evaluate_script()
// ===============================

// Rust pushes new weather data
window.receiveWeatherUpdate = function (weather) {
    currentWeather = weather;
    updateWeatherCard(weather);

    // Initialize or update radar (NEW)
    initRadar(weather.lat, weather.lon);

    // Auto-generate personality output if not in Serious mode
    if (currentPersona !== "Serious") {
        requestRoast();
    }
};

// Rust pushes new alert data
window.receiveAlertUpdate = function (alert) {
    currentAlert = alert;
    updateAlertBanner(alert);

    // If it's extreme, Rust will also trigger alarm UI
};

// Rust triggers emergency alarm UI
window.triggerEmergencyAlarm = function (alert) {
    showEmergencyOverlay(alert);
};

// ===============================
// JS → RUST IPC CALLS
// ===============================

// Ask Rust to generate personality output
function requestRoast() {
    if (!currentWeather) return;

    window.__TAURI__.invoke("generate_personality", {
        persona: currentPersona,
        chaos: chaosMode,
        weather: currentWeather
    }).then(roast => {
        updateRoastOutput(roast);
    }).catch(err => {
        console.error("Personality engine error:", err);
    });
}

// User selects a persona
function setPersona(persona) {
    currentPersona = persona;
    document.getElementById("persona-label").innerText = persona;

    if (persona === "Serious") {
        hideRoastOutput();
    } else {
        requestRoast();
    }
}

// Toggle chaos mode
function toggleChaos() {
    chaosMode = !chaosMode;
    document.getElementById("chaos-indicator").innerText =
        chaosMode ? "CHAOS MODE: ON" : "CHAOS MODE: OFF";

    if (currentPersona !== "Serious") {
        requestRoast();
    }
}

// Manual refresh button
function manualRefresh() {
    window.__TAURI__.invoke("manual_refresh")
        .catch(err => console.error("Manual refresh failed:", err));
}

// ===============================
// UI UPDATE FUNCTIONS
// ===============================

function updateWeatherCard(weather) {
    document.getElementById("temp").innerText = `${weather.temp}°F`;
    document.getElementById("humidity").innerText = `${weather.humidity}%`;
    document.getElementById("wind").innerText = `${weather.wind_speed} mph`;
    document.getElementById("pressure").innerText = `${weather.pressure} mb`;
    document.getElementById("city").innerText = weather.city;
}

function updateAlertBanner(alert) {
    const banner = document.getElementById("alert-banner");

    if (!alert) {
        banner.style.display = "none";
        return;
    }

    banner.style.display = "block";
    banner.innerText = `${alert.event} — ${alert.severity}`;
}

function updateRoastOutput(text) {
    const box = document.getElementById("roast-box");
    box.style.display = "block";
    box.innerText = text;
}

function hideRoastOutput() {
    const box = document.getElementById("roast-box");
    box.style.display = "none";
}

// ===============================
// EMERGENCY OVERLAY
// ===============================

function showEmergencyOverlay(alert) {
    const overlay = document.getElementById("emergency-overlay");
    const msg = document.getElementById("emergency-message");

    overlay.style.display = "flex";
    msg.innerText = `${alert.event}\n\n${alert.description}`;
}

function closeEmergencyOverlay() {
    document.getElementById("emergency-overlay").style.display = "none";
}

// ===============================
// Expose functions to HTML
// ===============================

window.setPersona = setPersona;
window.toggleChaos = toggleChaos;
window.manualRefresh = manualRefresh;
window.closeEmergencyOverlay = closeEmergencyOverlay;

// ===============================
// OPEN-METEO RADAR (NEW)
// ===============================

let radarMap = null;
let radarLayer = null;

function initRadar(lat, lon) {
    if (!radarMap) {
        radarMap = L.map("radar-map").setView([lat, lon], 7);
    }

    // Remove old layer if exists
    if (radarLayer) {
        radarMap.removeLayer(radarLayer);
    }

    // Open-Meteo radar tiles
    radarLayer = L.tileLayer(
        "https://tile.open-meteo.com/v1/radar/{z}/{x}/{y}.png",
        {
            tileSize: 256,
            opacity: 0.75,
            attribution: "© Open-Meteo Radar"
        }
    );

    radarLayer.addTo(radarMap);
}
