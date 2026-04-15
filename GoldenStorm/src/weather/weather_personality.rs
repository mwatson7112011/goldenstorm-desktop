use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use chrono::Timelike;
use serde::Deserialize;

// ---------- CORE DATA ----------

pub struct WeatherStatus {
    pub temp: f64,
    pub humidity: i32,
    pub pressure: f64,
    pub code: i32,
    pub wind_speed: f64,
    pub home_location: String,
    pub target_location: String,
    pub alert: Option<String>,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
pub enum ChaosLevel {
    Normal,
    Chaos,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
pub enum PersonaMode {
    Serious,
    StarWars,
    Spock,
    Roast,
    Sheldon,
    Apocalypse,
}

// ---------- UTIL ----------

fn choose<'a>(bank: &'a [&str], rng: &mut ThreadRng) -> &'a str {
    bank.choose(rng).copied().unwrap_or("...")
}

// ---------- WORD BANKS ----------

// Serious mode
const SERIOUS_REMARKS: &[&str] = &[
    "Conditions are stable. Suggesting standard outdoor preparations.",
    "Barometric shift detected. Individuals with joint sensitivity should monitor symptoms.",
    "High humidity. Ensure proper ventilation and hydration.",
    "Severe weather alert in effect. Please follow local safety protocols.",
];

// Temperature descriptors
const TEMP_COLD_NORMAL: &[&str] = &[
    "bone-shattering cold",
    "arctic nonsense",
    "frost-gremlin-approved chill",
];

const TEMP_HOT_NORMAL: &[&str] = &[
    "skin-melting heat",
    "Florida-level scorch",
    "sweat-summoning misery",
];

const TEMP_MILD_NORMAL: &[&str] = &[
    "pleasantly boring air",
    "statistically average conditions",
    "the meteorological equivalent of beige",
];

const TEMP_COLD_CHAOS: &[&str] = &[
    "cold enough to make penguins file HR complaints",
    "so cold your thoughts are forming icicles",
];

const TEMP_HOT_CHAOS: &[&str] = &[
    "so hot your DNA is reconsidering its life choices",
    "lava-adjacent suffering",
];

const TEMP_MILD_CHAOS: &[&str] = &[
    "so mild it’s suspicious",
    "emotionally neutral weather",
];

// Humidity
const HUMID_HIGH_NORMAL: &[&str] = &[
    "thick enough to chew.",
    "basically gravy.",
    "air with the consistency of warm, wet pudding.",
];

const HUMID_HIGH_CHAOS: &[&str] = &[
    "moist enough to qualify as soup.",
    "humidity so high your lungs need snorkels.",
    "the atmosphere has entered its stew phase.",
    "thick enough to chew with a fork.",
];

// Wind
const WIND_STRONG_NORMAL: &[&str] = &[
    "strong enough to yeet a lawn chair.",
    "perfect for involuntary hang-gliding.",
    "capable of rearranging your face.",
];

const WIND_STRONG_CHAOS: &[&str] = &[
    "wind so aggressive it’s basically a personality test.",
    "gusts strong enough to uninstall your roof.",
    "nature’s way of shuffling humans like loose papers.",
];

// Time-of-day moods
const MOOD_MORNING: &[&str] = &[
    "too early for this nonsense.",
    "the sun is judging you.",
    "your coffee needs coffee.",
];

const MOOD_AFTERNOON: &[&str] = &[
    "the afternoon slump is real.",
    "the sun is bored and so am I.",
    "peak 'why am I still here' hours.",
];

const MOOD_NIGHT: &[&str] = &[
    "nighttime chaos energy.",
    "the moon is silently judging you.",
    "perfect conditions for overthinking everything.",
];

const MOOD_MORNING_CHAOS: &[&str] = &[
    "the sun rose and immediately regretted it.",
    "this morning is sponsored by poor life choices.",
];

const MOOD_AFTERNOON_CHAOS: &[&str] = &[
    "the afternoon where time stops but suffering continues.",
    "emotionally it’s 3:17 PM forever.",
];

const MOOD_NIGHT_CHAOS: &[&str] = &[
    "the kind of night where side quests happen.",
    "the sky is dark and so is your browser history.",
];

// Star Wars
const SITH_HEAT: &[&str] = &[
    "I find your lack of air conditioning disturbing. Mustafar is cooler than this.",
    "The sun is a cruel master. I can feel your anger. It gives you focus... and a migraine.",
    "You underestimate the power of a triple-digit heatwave! It is useless to resist the indoors.",
];

const STARWARS_GENERAL: &[&str] = &[
    "I find your lack of meteorological awareness disturbing.",
    "The forecast is strong with this one.",
    "In my experience, there is no such thing as 'perfect weather'.",
];

// Sheldon
const SHELDON_REMARKS: &[&str] = &[
    "Bazinga! You thought it was 'nice' out, but you ignored the localized adiabatic cooling.",
    "I’ve occupied 'my spot' on the couch because the current dew point is physically repulsive.",
    "I'm not crazy; my mother had me tested. But staying out in this wind? Certifiably insane.",
    "It's a common misconception that 75 degrees is 'pleasant.' Not with this nitrogen ratio.",
];

// Apocalypse / ultimate chaos
const APOCALYPSE_STING: &[&str] = &[
    "The sun is actively trying to delete your soul. You're just poorly-rendered bacon now.",
    "Humidity is at 99%. The air is a hot, wet lung-transplant rejection. Go lie in a freezer.",
    "Barometric pressure is so low the earth is trying to suck your organs out through your pores.",
    "It’s so hot even the devil is calling Panama City to ask for the thermostat settings.",
];

// ---------- CORE GENERATORS ----------

fn generate_location_snark(status: &WeatherStatus, rng: &mut ThreadRng) -> String {
    let home = status.home_location.to_lowercase().trim().to_string();
    let target = status.target_location.to_lowercase().trim().to_string();

    if target != home {
        format!(
            "Looking at {}? Brave choice. It's a different flavor of atmospheric failure.",
            status.target_location
        )
    } else if home.contains("panama city") {
        "You're in Panama City. The air is basically salt-flavored soup. Breathe it in.".into()
    } else if home.contains("warner robins") {
        "Middle Georgia. Still here. Still humid. Still a sauna for your lungs.".into()
    } else {
        "You're home. The weather is exactly as emotionally confusing as you remember.".into()
    }
}

fn generate_alert_snark(status: &WeatherStatus, _rng: &mut ThreadRng) -> Option<String> {
    let alert_msg = status.alert.as_ref()?;
    let a = alert_msg.to_lowercase();

    if a.contains("tornado") {
        Some("Tornado watch. Grab a lawn chair. If you get sucked up, try to land on a developer who writes better code than me.".into())
    } else if a.contains("hurricane") || a.contains("tropical") {
        Some("Hurricane incoming. The ocean is just trying to come inside for a drink. Don't be a bad host.".into())
    } else {
        Some("The NWS is yelling again. Just nod and stay inside.".into())
    }
}

fn generate_temp_snark(temp: f64, chaos: ChaosLevel, rng: &mut ThreadRng) -> String {
    let bank = match chaos {
        ChaosLevel::Normal => {
            if temp < 45.0 {
                TEMP_COLD_NORMAL
            } else if temp > 85.0 {
                TEMP_HOT_NORMAL
            } else {
                TEMP_MILD_NORMAL
            }
        }
        ChaosLevel::Chaos => {
            if temp < 45.0 {
                TEMP_COLD_CHAOS
            } else if temp > 85.0 {
                TEMP_HOT_CHAOS
            } else {
                TEMP_MILD_CHAOS
            }
        }
    };

    format!("Temperature {:.0}° — {}.", temp, choose(bank, rng))
}

fn generate_wind_snark(wind: f64, chaos: ChaosLevel, rng: &mut ThreadRng) -> String {
    if wind < 10.0 {
        return match chaos {
            ChaosLevel::Normal => "Wind is calm enough that even your hair won’t complain.".into(),
            ChaosLevel::Chaos => "The wind is suspiciously calm. Something is plotting.".into(),
        };
    }

    let bank = match chaos {
        ChaosLevel::Normal => WIND_STRONG_NORMAL,
        ChaosLevel::Chaos => WIND_STRONG_CHAOS,
    };

    format!("Wind {:.0} mph — {}.", wind, choose(bank, rng))
}

fn generate_humidity_snark(h: i32, chaos: ChaosLevel, rng: &mut ThreadRng) -> String {
    if h < 50 {
        return match chaos {
            ChaosLevel::Normal => "Humidity is tolerable. A rare blessing.".into(),
            ChaosLevel::Chaos => "Humidity is low, which feels like a clerical error in this region.".into(),
        };
    }

    let bank = match chaos {
        ChaosLevel::Normal => HUMID_HIGH_NORMAL,
        ChaosLevel::Chaos => HUMID_HIGH_CHAOS,
    };

    format!("Humidity {}% — {}", h, choose(bank, rng))
}

fn generate_time_mood(chaos: ChaosLevel, rng: &mut ThreadRng) -> String {
    let hour = chrono::Local::now().hour();

    let base_bank = match hour {
        5..=11 => MOOD_MORNING,
        12..=17 => MOOD_AFTERNOON,
        _ => MOOD_NIGHT,
    };

    let chaos_bank = match hour {
        5..=11 => MOOD_MORNING_CHAOS,
        12..=17 => MOOD_AFTERNOON_CHAOS,
        _ => MOOD_NIGHT_CHAOS,
    };

    let bank = match chaos {
        ChaosLevel::Normal => base_bank,
        ChaosLevel::Chaos => {
            if rng.gen_bool(0.5) { base_bank } else { chaos_bank }
        }
    };

    choose(bank, rng).to_string()
}

// ---------- SERIOUS MODE ----------

fn generate_serious_report(status: &WeatherStatus, rng: &mut ThreadRng) -> String {
    let base = format!(
        "Temperature: {:.0}°. Humidity: {}%. Wind: {:.0} mph. Pressure: {:.1} hPa.",
        status.temp, status.humidity, status.wind_speed, status.pressure
    );

    match generate_alert_snark(status, rng) {
        Some(alert) => format!("{base} ALERT: {alert}"),
        None => base,
    }
}

// ---------- PERSONA STINGERS ----------

fn generate_persona_stinger(
    status: &WeatherStatus,
    persona: PersonaMode,
    rng: &mut ThreadRng,
) -> String {
    match persona {
        PersonaMode::StarWars => {
            if status.temp > 85.0 {
                choose(SITH_HEAT, rng).to_string()
            } else {
                choose(STARWARS_GENERAL, rng).to_string()
            }
        }
        PersonaMode::Sheldon => choose(SHELDON_REMARKS, rng).to_string(),
        PersonaMode::Spock => "Logic suggests this weather is suboptimal for carbon-based life forms.".to_string(),
        PersonaMode::Roast => "I've seen better conditions in a dumpster fire.".to_string(),
        PersonaMode::Apocalypse => choose(APOCALYPSE_STING, rng).to_string(),
        _ => String::new(),
    }
}

// ---------- APOCALYPSE MODE ----------

fn generate_apocalypse_block(status: &WeatherStatus, rng: &mut ThreadRng) -> String {
    let mut fragments = Vec::new();

    fragments.push("CORE MELTDOWN DETECTED:".to_string());
    fragments.push(choose(APOCALYPSE_STING, rng).to_string());

    if status.humidity > 80 {
        fragments.push(format!(
            "Humidity is {}% — {}",
            status.humidity,
            choose(HUMID_HIGH_CHAOS, rng)
        ));
    }

    if status.pressure < 1005.0 {
        fragments.push(
            "Barometric pressure is so low your joints are filing formal complaints.".to_string(),
        );
    }

    fragments.push("WESA ALL GONNA DIE-SA!".to_string());

    fragments.join(" ")
}

// ---------- PUBLIC ENTRY POINT ----------

pub fn generate_remark(
    status: &WeatherStatus,
    chaos: ChaosLevel,
    persona: PersonaMode,
) -> String {
    let mut rng = rand::thread_rng();

    // 1. Serious mode
    if persona == PersonaMode::Serious {
        return generate_serious_report(status, &mut rng);
    }

    // 2. Apocalypse mode
    if persona == PersonaMode::Apocalypse {
        return generate_apocalypse_block(status, &mut rng);
    }

    // 3. Standard procedural snark
    let mut fragments = Vec::new();

    // Location
    fragments.push(generate_location_snark(status, &mut rng));

    // Alert
    if let Some(alert_text) = generate_alert_snark(status, &mut rng) {
        fragments.push(alert_text);
    }

    // Core weather-driven snark
    fragments.push(generate_temp_snark(status.temp, chaos, &mut rng));
    fragments.push(generate_wind_snark(status.wind_speed, chaos, &mut rng));
    fragments.push(generate_humidity_snark(status.humidity, chaos, &mut rng));

    // Time-of-day mood
    fragments.push(generate_time_mood(chaos, &mut rng));

    // Persona stinger
    let stinger = generate_persona_stinger(status, persona, &mut rng);
    if !stinger.is_empty() {
        fragments.push(stinger);
    }

    // Pressure aches
    if status.pressure < 1005.0 {
        fragments.push(
            "Your arthritis is likely playing a drum solo on your nervous system.".to_string(),
        );
    }

    fragments.join(" ")
}
