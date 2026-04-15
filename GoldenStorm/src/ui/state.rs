use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct UiState {
    pub persona: Arc<Mutex<String>>,
    pub chaos_mode: Arc<Mutex<bool>>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            persona: Arc::new(Mutex::new("Serious".to_string())),
            chaos_mode: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_persona(&self, p: &str) {
        let mut persona = self.persona.lock().unwrap();
        *persona = p.to_string();
    }

    pub fn set_chaos(&self, enabled: bool) {
        let mut chaos = self.chaos_mode.lock().unwrap();
        *chaos = enabled;
    }

    pub fn get_persona(&self) -> String {
        self.persona.lock().unwrap().clone()
    }

    pub fn get_chaos(&self) -> bool {
        *self.chaos_mode.lock().unwrap()
    }
}
