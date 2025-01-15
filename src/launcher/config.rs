pub struct Config {
    pub username: String,
    pub resolution: (u32, u32),
}

impl Config {
    pub fn new(username: &str, resolution: (u32, u32)) -> Self {
        Config {
            username: username.to_string(),
            resolution,
        }
    }
}