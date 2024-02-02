use vrchatapi::apis::configuration::Configuration;

pub fn get_vrchat_api_client(username: &str, password: &str) -> Configuration {
    let mut config = Configuration::default();
    config.basic_auth = Some((username.to_string(), Some(password.to_string())));
    config
}
