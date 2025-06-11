pub const DEFAULT_HOST: &str = "0.0.0.0:80";
pub const DEFAULT_MONGO: &str = "mongodb://mongodb:27017";
pub const DEFAULT_REDIS: &str = "redis://redis:6379";

lazy_static::lazy_static! {
    pub static ref HOST: String = std::env::var("HOST")
        .ok()
        .and_then(|o| o.parse().ok())
        .unwrap_or_else(|| DEFAULT_HOST.to_owned());
    pub static ref MONGO_URL: String = std::env::var("MONGO_URL")
        .ok()
        .and_then(|o| o.parse().ok())
        .unwrap_or_else(|| DEFAULT_MONGO.to_owned());
    pub static ref REDIS_URL: String = std::env::var("REDIS_URL")
        .ok()
        .and_then(|o| o.parse().ok())
        .unwrap_or_else(|| DEFAULT_REDIS.to_owned());
}
