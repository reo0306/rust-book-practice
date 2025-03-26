use std::fs;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
    pub debug: bool,
    pub allowed_ips: Vec<String>,
}

impl Config {
    pub fn load_from_file(path: &str) -> Option<Config> {
        let json = fs::read_to_string(path).ok()?;
        let config: Config = serde_json::from_str(&json).ok()?;

        Some(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_load_from_file() {
        let result = Config::load_from_file("config.json");
        assert_eq!(result, Some(Config {
            app_name: "MyApp".to_string(),
            port: 8080,
            debug: true,
            allowed_ips: vec!["192.168.0.1".to_string(), "192.168.0.2".to_string()],
        }));
    }

    #[test]
    fn it_load_from_file_none() {
        let result = Config::load_from_file("config1.json");
        assert_eq!(result, None);
    }
}
