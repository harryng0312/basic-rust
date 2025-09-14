// #![allow(warnings)]
#![allow(clippy::too_many_arguments, unused_variables, dead_code)]

use crate::utils::boot::boot;

pub(crate) mod dto;
pub(crate) mod models;
pub(crate) mod persistence;
pub(crate) mod utils;

fn main() {
    boot();
    // let rec = TestRecord::new(0, String::default(), false, Default::default());
    // info!("Test Struct");
    // sleep(Duration::from_secs(5));
    // let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .worker_threads(4)
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // let rs = rt.block_on(async move {
    //     sleep(Duration::from_secs(5)).await;
    //     1
    // });
    // info!("{}", rs);
}
