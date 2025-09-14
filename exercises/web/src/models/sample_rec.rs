// use macros::record;
use crate::record;
use chrono::NaiveDateTime;
use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable};

table! {
    #[sql_name="test_rec"]
    sample_recs (id) {
        #[sql_name="id_"]
        id -> BigInt,
        #[sql_name="name_"]
        name -> VarChar,
        available -> Bool,
        created_at -> Timestamp,
    }
}

// #[derive(Queryable, Identifiable, Insertable, AsChangeset)]
// #[derive(Debug, Serialize, Queryable, Identifiable, Insertable, AsChangeset)]
// #[record(derive(Debug, Serialize, Deserialize))]
// #[table_name = "test_recs"]
// #[primary_key(id_)]

// #[derive(Queryable, Identifiable, Insertable, AsChangeset)]
// #[record(derive(Debug, Serialize, Deserialize))]
// #[diesel(table_name=test_recs)]
// #[diesel(primary_key(id))]
// pub struct TestRecord {
//     // #[column_name = "id_"]
//     id: i64,
//     name: String,
//     available: bool,
//     created_at: NaiveDateTime,
// }

record! {
    #[derive(Queryable, Identifiable, Insertable, AsChangeset)]
    #[diesel(table_name=sample_recs)]
    #[diesel(primary_key(id))]
    SampleRecord {
        id : i64,
        name : String,
        available : bool,
        created_at : NaiveDateTime,
    }
}
