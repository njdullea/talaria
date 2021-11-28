use std::env;

// To setup environment variables, execute this function.
pub fn setup_local_env() {
    dotenv::dotenv().ok();
}

// To get an environment variable, call this function. The environment variables must be setup.
pub fn get_env_var(key: &str) -> Option<String> {
    let key: Option<String> = match env::var(key) {
        Ok(api_key) => Some(api_key),
        Err(_) => {
            panic!("Could not find api key!");
            // None
        }
    };

    return key;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_gets_env_var() {
        super::setup_local_env();
        let test_key = super::get_env_var("TEST_KEY").unwrap();
        assert_eq!(test_key, "TEST_KEY");
    }
}
