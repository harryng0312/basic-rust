Ok 👍 mình sẽ demo một REST API nhỏ dùng Axum + Diesel + r2d2, có connection pool quản lý SQLite database.

Ví dụ: ta có một bảng users (id, name) và muốn API đọc & thêm user.

⸻

📦 Cargo.toml

[package]
name = "axum_diesel_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
diesel = { version = "2.1", features = ["sqlite", "r2d2"] }
r2d2 = "0.8"
dotenvy = "0.15"

⸻

📂 src/schema.rs

(được Diesel sinh ra bằng diesel migration run)

diesel::table! {
users (id) {
id -> Integer,
name -> Text,
}
}

⸻

📂 src/models.rs

use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Queryable, Serialize)]
pub struct user {
pub id: i32,
pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
pub name: String,
}

⸻

📂 src/main.rs

mod schema;
mod models;

use axum::{
extract::{Extension, Json},
routing::{get, post},
Router,
};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use models::{user, NewUser};
use std::net::SocketAddr;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[tokio::main]
async fn main() {
// Tạo connection pool
let manager = ConnectionManager::<SqliteConnection>::new("demo.sqlite");
let pool = r2d2::Pool::builder()
.build(manager)
.expect("Không tạo được pool");

    // Tạo router với state (pool)
    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .layer(Extension(pool));

    // Chạy server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server chạy tại http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

// GET /users -> trả về toàn bộ user
async fn list_users(Extension(pool): Extension<DbPool>) -> Json<Vec<user>> {
use crate::schema::users::dsl::*;
let conn = pool.get().expect("Lấy connection thất bại");
let results = users
.load::<user>(&conn)
.expect("Không đọc được users");
Json(results)
}

// POST /users -> thêm user mới
async fn create_user(
Extension(pool): Extension<DbPool>,
Json(new_user): Json<NewUser>,
) -> &'static str {
use crate::schema::users::dsl::*;
let conn = pool.get().expect("Lấy connection thất bại");

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .expect("Không insert được");

    "✅ user đã được thêm"

}

⸻

🔹 Cách hoạt động
• pool.get() lấy connection từ pool.
• Khi conn ra khỏi scope (kết thúc hàm), nó release về pool → tái sử dụng cho request khác.
• Nếu pool bị drop (ví dụ khi server shutdown), toàn bộ connection sẽ bị đóng.

⸻

🔹 Test API

Chạy:

cargo run

Thêm user:

curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Alice"}' \
http://127.0.0.1:3000/users

Đọc danh sách:

curl http://127.0.0.1:3000/users

⸻
Ok 👍 mình sẽ mở rộng ví dụ trên sang PostgreSQL để giống môi trường production hơn.

⸻

📦 Cargo.toml

[package]
name = "axum_diesel_postgres"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
diesel = { version = "2.1", features = ["postgres", "r2d2"] }
r2d2 = "0.8"
dotenvy = "0.15"

⸻

🔹 Chuẩn bị database

Tạo database PostgreSQL:

createdb axum_demo

Tạo file .env:

DATABASE_URL=postgres://postgres:password@localhost/axum_demo

⸻

📂 src/schema.rs

(sinh ra bằng diesel migration run)

diesel::table! {
users (id) {
id -> Int4,
name -> Varchar,
}
}

⸻

📂 src/models.rs

use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Queryable, Serialize)]
pub struct user {
pub id: i32,
pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
pub name: String,
}

⸻

📂 src/main.rs

mod schema;
mod models;

use axum::{
extract::{Extension, Json},
routing::{get, post},
Router,
};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use models::{user, NewUser};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL chưa được set");
let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .max_size(15) // số connection tối đa trong pool
        .build(manager)
        .expect("Không tạo được pool");

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server chạy tại http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

// GET /users
async fn list_users(Extension(pool): Extension<DbPool>) -> Json<Vec<user>> {
use crate::schema::users::dsl::*;
let conn = pool.get().expect("Lấy connection thất bại");
let results = users
.load::<user>(&conn)
.expect("Không đọc được users");
Json(results)
}

// POST /users
async fn create_user(
Extension(pool): Extension<DbPool>,
Json(new_user): Json<NewUser>,
) -> &'static str {
use crate::schema::users::dsl::*;
let conn = pool.get().expect("Lấy connection thất bại");

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .expect("Không insert được");

    "✅ user đã được thêm"

}

⸻

🔹 Migrations

Chạy:

diesel setup
diesel migration generate create_users

Trong file up.sql:

CREATE TABLE users (
id SERIAL PRIMARY KEY,
name VARCHAR NOT NULL
);

Trong file down.sql:

DROP TABLE users;

Rồi chạy:

diesel migration run

⸻

🔹 Test API

Chạy server:

cargo run

Thêm user:

curl -X POST -H "Content-Type: application/json" \
-d '{"name": "Bob"}' \
http://127.0.0.1:3000/users

Đọc danh sách:

curl http://127.0.0.1:3000/users

⸻

📌 Ở đây Diesel + r2d2 sẽ lo đóng/mở connection tự động:
• pool.get() → lấy connection từ pool.
• Khi conn ra khỏi scope → nó được release về pool, tái sử dụng cho request khác.
• Nếu toàn bộ pool bị drop (khi server shutdown) → tất cả connection tới Postgres sẽ đóng thật sự.

⸻


