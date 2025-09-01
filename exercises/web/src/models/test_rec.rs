use chrono::NaiveDateTime;
use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable};
use macros::record;
use serde::{Deserialize, Serialize};

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

#[derive(Queryable, Identifiable, Insertable, AsChangeset)]
// #[derive(Debug, Serialize, Queryable, Identifiable, Insertable, AsChangeset)]
#[record(derive(Debug, Serialize, Deserialize))]
// #[table_name = "test_recs"]
#[diesel(table_name=test_recs)]
// #[primary_key(id_)]
#[diesel(primary_key(id))]
pub struct TestRecord {
    // #[column_name = "id_"]
    id: i64,
    name: String,
    available: bool,
    created_at: NaiveDateTime,
}
