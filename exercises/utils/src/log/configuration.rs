use std::env;
use std::sync::OnceLock;
use tracing::subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

const CONFIG_FILE: &str = "config.toml";
// static INIT: Once = Once::new();
// static GUARD: OnceCell<tracing_appender::non_blocking::WorkerGuard> = OnceCell::new();
static GUARDS: OnceLock<Vec<WorkerGuard>> = OnceLock::new();

pub fn load_config_file() -> String {
    std::fs::read_to_string(CONFIG_FILE).unwrap()
}

// pub fn init_logger() {
//     let mut builder = Builder::new();
//     // let mut builder = Builder::from_env(Env::default().default_filter_or("debug"));
//     builder
//         .target(env_logger::Target::Stdout)
//         // .filter_level(LevelFilter::Info)
//         .filter_module("diesel", LevelFilter::Debug)
//         .filter(None, LevelFilter::Info)
//         .init();
// }

pub fn init_logger() {
    let run_env = env::var("ENV").unwrap_or(String::from("dev"));
    dotenvy::from_filename(format!(".env.{run_env}")).ok(); // success
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("./logs"));
    let log_file = env::var("LOG_FILE").unwrap_or(String::from("not_config"));

    let mut layers: Vec<Box<dyn tracing_subscriber::Layer<Registry> + Send + Sync>> = vec![];
    let mut guards: Vec<WorkerGuard> = vec![];

    let (stdout, _guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(_guard);

    let env_filter = std::env::var("RUST_LOG")
        .ok()
        .map(EnvFilter::new)
        .unwrap_or_else(|| EnvFilter::new("info"));
    layers.push(Box::new(env_filter));
    let fmt_stdout = fmt::layer()
        .with_target(false)
        .with_thread_names(true)
        .with_line_number(true)
        .with_file(true)
        // for test
        .with_writer(stdout)
        .with_test_writer();

    layers.push(Box::new(fmt_stdout));

    if cfg!(test) {
        println!("Run with `RUST_LOG={}=debug` test", run_env);
    }

    if run_env.to_lowercase() == "test" {
        // do nothing
    } else {
        let file_appender = rolling::Builder::new()
            .rotation(Rotation::DAILY)
            .filename_prefix(log_file)
            .filename_suffix("log")
            .max_log_files(5)
            .build(log_path)
            .expect("build rolling appender");
        let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
        guards.push(_guard);
        let fmt_file = fmt::layer()
            .with_ansi(false)
            .with_target(true)
            .with_thread_names(true)
            .with_line_number(true)
            .with_file(true)
            .with_writer(file_writer);
        layers.push(Box::new(fmt_file));
    }
    // tracing_subscriber::registry()
    //     .with(env_filter)
    //     .with(fmt_stdout)
    //     .with(fmt_file)
    //     .init();
    let _ = GUARDS.get_or_init(|| guards);
    tracing_subscriber::registry().with(layers).init();
}
