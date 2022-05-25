use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

fn main() {
    let config: Config = Config::figment().extract().unwrap();

    println!("{:#?}", config);
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub secret: String,
}

impl Config {
    pub fn figment() -> Figment {
        Figment::new()
            .merge(Env::prefixed("APP_").global())
            .merge(Toml::file("App.toml"))
    }
}
