use advanced::record;
use chrono::{Local, NaiveDate};

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
#[allow(
    unused,
    unused_mut,
    unused_variables,
    unused_imports,
    unreachable_code,
    unused_unsafe
)]
// #[record(derive="Debug, Clone, Default")]
// #[record]
#[record(derive(Debug, Clone, Default))]
// #[derive(Debug)]
pub struct User {
    id: i64,
    username: String,
    dob: NaiveDate,
}

fn main() {
    let date_now = Local::now().date_naive();
    let now = Local::now().naive_local();

    let user = User {
        id: 1,
        username: "".to_string(),
        dob: date_now,
    };
    println!("{:?}", user);
}
