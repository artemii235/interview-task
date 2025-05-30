pub const DEFAULT_HOST: &str = "0.0.0.0:80";

lazy_static::lazy_static! {
    pub static ref HOST: String = std::env::var("HOST")
        .ok()
        .and_then(|o| o.parse().ok())
        .unwrap_or_else(|| DEFAULT_HOST.to_owned());
}
