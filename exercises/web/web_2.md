Dưới đây là ví dụ Rust + Diesel minh họa cách xây dựng trait repository chung, implement cho UserRepo và PostRepo, có hỗ trợ transaction và batch insert, dùng một template implement chung.

⸻

1. Schema (schema.rs)

diesel::table! {
users (id) {
id -> Int4,
name -> Varchar,
created_at -> Timestamp,
}
}

diesel::table! {
posts (id) {
id -> Int4,
user_id -> Int4,
title -> Varchar,
body -> Text,
created_at -> Timestamp,
}
}


⸻

2. Models (models.rs)

use chrono::NaiveDateTime;
use crate::schema::{users, posts};

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
pub id: i32,
pub name: String,
pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
pub name: &'a str,
pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
pub id: i32,
pub user_id: i32,
pub title: String,
pub body: String,
pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
pub user_id: i32,
pub title: &'a str,
pub body: &'a str,
pub created_at: NaiveDateTime,
}


⸻

3. Trait repository chung

use diesel::prelude::*;
use diesel::result::Error;

/// Generic CRUD trait
pub trait Repository<T> {
fn find_all(conn: &mut PgConnection) -> Result<Vec<T>, Error>;
fn find_by_id(conn: &mut PgConnection, id: i32) -> Result<T, Error>;
fn insert_batch<'a>(conn: &mut PgConnection, objs: &[T]) -> Result<usize, Error>;
}

	•	insert_batch cho phép batch insert nhiều object cùng lúc
	•	Có thể mở rộng thêm update, delete nếu cần

⸻

4. Macro để implement chung

Chúng ta có thể dùng macro để tránh viết lặp:

macro_rules! impl_repository {
($repo:ident, $model:ty, $table:ident) => {
pub struct $repo;

        impl Repository<$model> for $repo {
            fn find_all(conn: &mut PgConnection) -> Result<Vec<$model>, Error> {
                use crate::schema::$table::dsl::*;
                $table.load(conn)
            }

            fn find_by_id(conn: &mut PgConnection, id_val: i32) -> Result<$model, Error> {
                use crate::schema::$table::dsl::*;
                $table.find(id_val).get_result(conn)
            }

            fn insert_batch<'a>(conn: &mut PgConnection, objs: &[$model]) -> Result<usize, Error> {
                use crate::schema::$table::dsl::*;
                conn.transaction::<_, Error, _>(|| {
                    let mut total = 0;
                    for obj in objs {
                        total += diesel::insert_into($table).values(obj).execute(conn)?;
                    }
                    Ok(total)
                })
            }
        }
    };
}

	•	Macro giúp reuse cho nhiều bảng
	•	transaction tự động commit/rollback khi batch insert

⸻

5. Implement cho User và Post

use crate::models::{User, Post};

impl_repository!(UserRepo, User, users);
impl_repository!(PostRepo, Post, posts);

	•	Bây giờ cả UserRepo và PostRepo đều có các hàm find_all, find_by_id, insert_batch

⸻

6. Sử dụng

fn main() -> Result<(), Box<dyn std::error::Error>> {
let pool = establish_pool(); // R2D2 pool
let conn = &mut pool.get()?;

    use chrono::Local;

    let now = Local::now().naive_local();

    // Batch insert users
    let users = vec![
        User { id: 0, name: "Alice".into(), created_at: now },
        User { id: 0, name: "Bob".into(), created_at: now },
    ];
    let inserted = UserRepo::insert_batch(conn, &users)?;
    println!("Inserted {} users", inserted);

    // Batch insert posts
    let posts = vec![
        Post { id: 0, user_id: 1, title: "Post1".into(), body: "Hello".into(), created_at: now },
        Post { id: 0, user_id: 2, title: "Post2".into(), body: "World".into(), created_at: now },
    ];
    let inserted_posts = PostRepo::insert_batch(conn, &posts)?;
    println!("Inserted {} posts", inserted_posts);

    // Query
    let all_users = UserRepo::find_all(conn)?;
    println!("All users: {:?}", all_users);

    let post = PostRepo::find_by_id(conn, 1)?;
    println!("Post 1: {:?}", post);

    Ok(())
}


⸻

✅ Tóm tắt
1.	Trait Repository định nghĩa CRUD chung
2.	Macro impl_repository! giúp reuse code cho nhiều bảng
3.	Transaction được áp dụng cho batch insert, tự rollback nếu có lỗi
4.	Sử dụng chung cho UserRepo và PostRepo mà không cần viết lại logic

⸻

Mình có thể viết thêm phiên bản hỗ trợ NewUser / NewPost để insert mà không cần id, đúng chuẩn insert vào auto-increment primary key của PostgreSQL, nếu bạn muốn.

Bạn có muốn mình làm không?