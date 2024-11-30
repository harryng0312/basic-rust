use env_logger::Builder;
use log::LevelFilter;

pub const CONFIG_FILE: &str = "config.toml";

pub fn load_config_file() -> String {
    std::fs::read_to_string(CONFIG_FILE).unwrap()
}

pub fn init_logger() {
    let mut builder = Builder::new();
    builder
        .target(env_logger::Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();
}