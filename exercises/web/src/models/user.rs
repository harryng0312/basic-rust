use chrono::{NaiveDate, NaiveDateTime};
use diesel::{table, Identifiable, Insertable, Queryable};
use macros::record;

table! {
    #[sql_name="user_"]
    users (id) {
        #[sql_name="id_"]
        id -> BigInt,

        created_date -> Timestamp,
        modified_date -> Timestamp,

        dob -> Date,
        passwd -> VarChar,
        passwd_enc_method -> VarChar,
        screenname -> VarChar,

        #[sql_name="status_"]
        status -> SmallInt,
        username -> VarChar,

        org_id -> BigInt,
        org_treepath -> VarChar,
    }
}
// #[derive(Queryable, Serialize, Identifiable, Insertable, AsChangeset, Debug)]
#[derive(Queryable, Identifiable, Insertable)]
#[record]
// #[table_name = "user_"]
#[diesel(table_name=users)]
// #[primary_key(id_)]
#[diesel(primary_key(id))]
pub struct User {
    // #[column_name = "id_"]
    id: i64,
    created_date: NaiveDateTime,
    modified_date: NaiveDateTime,
    dob: NaiveDate,
    passwd: String,
    passwd_enc_method: String,
    screenname: String,
    // #[column_name = "status_"]
    status: i16,
    username: String,
    org_id: i64,
    org_treepath: String,
}
