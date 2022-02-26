use once_cell::sync::Lazy;
use std::env;

pub static ADDRESS: Lazy<String> = Lazy::new(|| env::var("ADDRESS").expect("ADDRESS is set"));
pub static ALLOWED_ORIGIN: Lazy<Vec<String>> =
    Lazy::new(|| get_list_env(env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN is set")));
pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL is set"));

// ----------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------
fn get_list_env(s: String) -> Vec<String> {
    s.split(",").map(|s| s.to_string()).collect()
}
