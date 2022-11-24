use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    database_id: String,
    project_id: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        let project_id = env::var("PROJECT_ID").expect("PROJECT_ID is defined");
        let database_id = "(default)".to_owned();
        Self {
            project_id,
            database_id,
        }
    }

    pub fn database_id(&self) -> &str {
        &self.database_id
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::config::Config;

    #[test]
    fn test() {
        env::set_var("PROJECT_ID", "project_id1");
        let config = Config::load_from_env();
        assert_eq!(config.project_id(), "project_id1");
        assert_eq!(config.database_id(), "(default)");
    }
}
