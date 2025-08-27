use chrono::{NaiveDateTime};
use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;

table! {
    #[sql_name="test_rec"]
    test_recs (id) {
        #[sql_name="id_"]
        id -> BigInt,
        #[sql_name="name_"]
        name -> VarChar,
        available -> Bool,
        created_at -> Timestamp,
    }
}
// #[derive(Queryable, Serialize, Identifiable, Insertable, AsChangeset, Debug)]
#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, AsChangeset)]
// #[table_name = "user_"]
#[diesel(table_name=test_recs)]
// #[primary_key(id_)]
#[diesel(primary_key(id))]
pub struct TestRecord {
    // #[column_name = "id_"]
    pub id: i64,
    pub name: String,
    pub available: bool,
    pub created_at: NaiveDateTime,
}
