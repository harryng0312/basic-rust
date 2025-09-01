use env_logger::Builder;
use log::LevelFilter;

const CONFIG_FILE: &str = "config.toml";

pub fn load_config_file() -> String {
    std::fs::read_to_string(CONFIG_FILE).unwrap()
}

pub fn init_logger() {
    let mut builder = Builder::new();
    // let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
    builder
        .target(env_logger::Target::Stdout)
        // .filter_level(LevelFilter::Info)
        .filter_module("diesel", LevelFilter::Debug)
        .filter(None, LevelFilter::Info)
        .init();
}
