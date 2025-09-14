use tracing::info;
use utils::log::configuration::init_logger;

fn init_http_ws() {}
pub(crate) fn boot() {
    init_logger();
    info!("Logging up!");
    init_http_ws();
    info!("Http WS up!");
    info!("!!!Started!!!");
}
