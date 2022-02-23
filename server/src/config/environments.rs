use once_cell::sync::Lazy;
use std::env;

pub static ADDRESS: Lazy<String> = Lazy::new(|| env::var("ADDRESS").expect("ADDRESS is set"));
pub static DATABASE_URL: Lazy<String> =
    Lazy::new(|| env::var("DATABASE_URL").expect("DATABASE_URL is set"));
