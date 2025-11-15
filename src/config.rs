use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub app_title: String,
    pub quiz_path: String,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_title: "Quiz App".to_string(),
            quiz_path: "Quizzes".to_string(),
            window_width: 400.0,
            window_height: 300.0,
        }
    }
}

pub fn load_config() -> AppConfig {
    let content = fs::read_to_string("Config.yaml").unwrap_or_else(|_| {
        r#"app_title: "Quiz App"
quiz_path: "Quizzes"
window_width: 400.0
window_height: 300.0
"#.to_string()
    });
    
    serde_yaml::from_str(&content).unwrap_or_else(|_| AppConfig::default())
}
