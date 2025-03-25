use std::collections::HashMap;
use std::fs;

pub fn load_env(file_path: &str) -> Option<HashMap<String, String>> {
    let env: String = fs::read_to_string(file_path).ok()?;

    let rows: Vec<&str> = env.as_str().lines().collect();

    let mut result: HashMap<String, String> = HashMap::new();
    for row in rows {
        if let Some((key, value))  = row.trim().split_once('=') {
            result.insert(key.to_string(), value.to_string());
        }
    }

    Some(result)
}

pub fn get_env_value(env: &HashMap<String, String>, key: &str) -> Option<String> {
    env.get(key).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_load_env() {
        let result = load_env(".env");
        assert_eq!(result, Some(HashMap::from([
            ("APP_NAME".to_string(), "MyApp".to_string()),
            ("APP_PORT".to_string(), "8080".to_string()),
            ("DEBUG".to_string(), "true".to_string())
        ])));
    }

    #[test]
    fn it_load_env_none() {
        let result = load_env("env");
        assert_eq!(result, None);
    }

    #[test]
    fn it_get_env_value() {
        let env = load_env(".env");
        let result = get_env_value(&env.unwrap(), "APP_NAME");
        assert_eq!(result, Some("MyApp".to_string()));
    }

    #[test]
    fn it_get_env_value_none() {
        let env = load_env(".env");
        let result = get_env_value(&env.unwrap(), "env");
        assert_eq!(result, None);
    }
}
