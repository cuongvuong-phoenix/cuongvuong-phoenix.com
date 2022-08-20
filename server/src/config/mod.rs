mod environments;

pub use environments::*;

pub fn load_env() {
    let profile = if cfg!(test) {
        "test"
    } else if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    };

    dotenvy::from_filename(format!(".env.{}.local", profile)).ok();
    dotenvy::from_filename(format!(".env.local")).ok();
    dotenvy::from_filename(format!(".env.{}", profile)).ok();
    dotenvy::dotenv().ok();
}
