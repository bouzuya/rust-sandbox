use std::env;

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
    // TODO: test load_from_env
    // TODO: test database_id
    // TODO: test project_id
}
