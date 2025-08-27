// use diesel::table;
//
// table! {
//     #[sql_name = "user_"]
//     user (id_) {
//         #[sql_name = "id_"]
//         id -> BigInt,
//         created_date -> TimeStamp,
//         modified_date -> TimeStamp,
//         dob -> Date,
//         passwd -> Varchar,
//         passwd_enc_method -> Varchar,
//         screenname -> Varchar
//         #[sql_name = "status_"]
//         status -> SmallInt,
//         username -> Varchar,
//         org_id -> BigInt,
//         org_treepath -> Varchar
//     }
// }