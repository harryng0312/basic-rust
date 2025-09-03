use chrono::NaiveDate;
// use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use macros::record;

#[record(derive(Debug))]
// #[derive(Debug, Serialize)]
pub struct DtoTest {
    id: i64,
    name: String,
    dob: NaiveDate,
}

// impl DtoTest {
//     pub fn new (id: i64, name: String, dob: NaiveDate) -> Self {
//         Self { id, name, dob }
//     }
// }
