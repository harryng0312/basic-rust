#[allow(
    unused,
    unused_mut,
    unused_variables,
    unused_imports,
    unreachable_code,
    unused_unsafe,
    dead_code
)]
use chrono::NaiveDate;
use log::info;
use macros::with;
use macros::{record, sum};
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
#[record(derive(Debug, Clone, Default))]
#[derive(Debug, Clone, Default)]
pub struct User {
    id: i64,
    username: String,
    dob: NaiveDate,
}
fn log_before(fn_name: &str, fn_params: &[&dyn std::fmt::Debug]) {
    info!("before fn: {}", fn_name);
}
fn log_after(fn_name: &str, fn_result: &dyn std::fmt::Debug, fn_params: &[&dyn std::fmt::Debug]) {
    info!("after fn: {} result:{:?}", fn_name, fn_result);
}
// #[with(before(log_before), after(log_after))]
#[with(before(log_before), after(log_after))]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    init_logger();

    // let date_now = Local::now().date_naive();
    // let now = Local::now().naive_local();
    // let sum = sum!(200u64, 20, 10, 30);
    // info!("sum: {} type {}", sum, type_of(&sum));
    info!("sum {}", sum(1, 2));
}
