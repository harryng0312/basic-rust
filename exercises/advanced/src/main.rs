#[allow(
    unused,
    unused_mut,
    unused_variables,
    unused_imports,
    unreachable_code,
    unused_unsafe,
    dead_code
)]
use advanced::{sum, record};
// use chrono::{Local};
use log::info;
use utils::log::configuration::init_logger;

// record! {serde,
//     User {
//     id: i64 = 0i64,
//     username: String = String::new(),
//     dob: NaiveDate = Local::now().date_naive(),
// }}
// record! {
//     User1 {
//     id: i64,
//     username: String,
//     dob: NaiveDate,
// }}

// #[record(derive="Debug, Clone, Default")]
// #[record]
// #[record(derive(Debug, Clone, Default))]
// #[derive(Debug)]
// pub struct User {
//     id: i64,
//     username: String,
//     dob: NaiveDate,
// }

fn main() {
    init_logger();
    // let date_now = Local::now().date_naive();
    // let now = Local::now().naive_local();
    let sum = sum!(200, 20);
    info!("sum: {}", sum);


}
