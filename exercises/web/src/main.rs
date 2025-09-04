// #![allow(warnings)]
#![allow(clippy::too_many_arguments, unused_variables, dead_code)]

use std::thread::sleep;
use std::time::Duration;
use tracing::info;
use ::utils::log::configuration::init_logger;

pub(crate) mod dto;
pub(crate) mod models;
pub(crate) mod persistence;
pub(crate) mod utils;

record! {
    TestStruct {
        id: i64,
    }
}

fn main() {
    init_logger();
    // let rec = TestRecord::new(0, String::default(), false, Default::default());
    info!("Test Struct");
    // sleep(Duration::from_secs(5));
}
