use chrono::{NaiveDate, NaiveDateTime};
use diesel::{table, AsChangeset, Identifiable, Insertable, Queryable};

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
#[derive(Debug, Queryable, Identifiable, Insertable)]
// #[table_name = "user_"]
#[diesel(table_name=users)]
// #[primary_key(id_)]
#[diesel(primary_key(id))]
pub struct User {
    // #[column_name = "id_"]
    pub id: i64,
    pub created_date: NaiveDateTime,
    pub modified_date: NaiveDateTime,
    pub dob: NaiveDate,
    pub passwd: String,
    pub passwd_enc_method: String,
    pub screenname: String,
    // #[column_name = "status_"]
    pub status: i16,
    pub username: String,
    pub org_id: i64,
    pub org_treepath: String,
}
