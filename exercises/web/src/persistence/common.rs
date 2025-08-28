use log::info;
use once_cell::sync::Lazy;
use std::env;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;
use utils::error::app_error::AppResult;

type RawDbConnection = diesel::PgConnection;
type DbConnectionPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<RawDbConnection>>;
pub type DbConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<RawDbConnection>>;

type AsyncDbConnectionPool = bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>;
pub type AsyncDbConnection =
    bb8::PooledConnection<'static, bb8_postgres::PostgresConnectionManager<NoTls>>;
static DB_CONNECTION_POOL: Lazy<DbConnectionPool> =
    Lazy::new(|| create_conn_pool().expect("Could not create DB connection pool"));

// static ASYNC_DB_CONNECTION_POOL: OnceCell<AsyncDbConnectionPool> = OnceCell::const_new();
static ASYNC_DB_CONNECTION_POOL: OnceCell<AsyncDbConnectionPool> = OnceCell::const_new();

fn create_conn_pool() -> AppResult<DbConnectionPool> {
    // dotenv().ok();
    let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());
    dotenvy::from_filename(format!(".env.{run_env}")).ok(); // success
    let database_url = env::var("DATABASE_URL").map_err(|e| e)?;
    let manager = diesel::r2d2::ConnectionManager::<RawDbConnection>::new(database_url);

    let pool = diesel::r2d2::Pool::builder()
        .max_size(15) // max_pool_size
        .build(manager)
        .expect("Could not create connection pool");

    Ok(pool)
}

pub async fn create_async_conn_pool() -> AppResult<AsyncDbConnectionPool> {
    let manager = bb8_postgres::PostgresConnectionManager::new(
        "host=localhost user=postgres password=123 dbname=mydb".parse()?,
        NoTls,
    );
    Ok(bb8::Pool::builder().max_size(15).build(manager).await?)
}

pub fn get_connection() -> AppResult<DbConnection> {
    let conn_pool = DB_CONNECTION_POOL.get()?;
    Ok(conn_pool)
}

pub async fn get_async_connection() -> AppResult<AsyncDbConnection> {
    // Ok(ASYNC_DB_CONNECTION_POOL
    //     .get_or_init(|| async { create_async_conn_pool().await })
    //     .await?)
    Ok(ASYNC_DB_CONNECTION_POOL
        .get_or_init(|| async {
            create_async_conn_pool()
                .await
                .expect("Could not create DB async connection pool")
        })
        .await
        .get()
        .await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_rec::test_recs::dsl::test_recs;
    use crate::models::test_rec::TestRecord;
    use crate::models::user::users::dsl::users;
    use crate::models::user::User;
    use chrono::NaiveDateTime;
    use diesel::{QueryDsl, RunQueryDsl};
    use log::{error, info};
    use utils::log::configuration::init_logger;

    #[test]
    fn test_get_conn_pool() -> AppResult<()> {
        init_logger();
        let mut conn = get_connection()?;
        info!("Get connection from pool successfully!");
        let _rs = test_recs
            .limit(3)
            // .get_results(&mut conn)?;
            .load::<TestRecord>(&mut conn)?;
        for u in _rs {
            info!("{:?}", u);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_async_connection() -> AppResult<()> {
        init_logger();
        let conn = get_async_connection().await?;
        info!("Start getting async connection from pool ...");
        let rows = conn
            .query_opt(
                "SELECT id_, name_, available, created_at FROM test_rec where id_ = $1",
                &[],
            )
            .await?;
        rows.map(|row| {
            let test_rec: TestRecord = TestRecord {
                id: row.get("id_"),
                name: row.get("name_"),
                available: row.get("available"),
                created_at: row.get::<_, NaiveDateTime>("created_at"),
            };
        });
        Ok(())
    }
}
