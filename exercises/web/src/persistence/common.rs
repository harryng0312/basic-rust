use once_cell::sync::Lazy;
use std::env;
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;
use utils::error::app_error::AppResult;

type RawDbConnection = diesel::PgConnection;
type DbConnectionPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<RawDbConnection>>;
pub(crate) type DbConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<RawDbConnection>>;

type AsyncDbConnectionPool = bb8::Pool<bb8_postgres::PostgresConnectionManager<NoTls>>;
pub(crate) type AsyncDbConnection =
    bb8::PooledConnection<'static, bb8_postgres::PostgresConnectionManager<NoTls>>;
static DB_CONNECTION_POOL: Lazy<DbConnectionPool> =
    Lazy::new(|| create_conn_pool().expect("Could not create DB connection pool"));

// static ASYNC_DB_CONNECTION_POOL: OnceCell<AsyncDbConnectionPool> = OnceCell::const_new();
static ASYNC_DB_CONNECTION_POOL: OnceCell<AsyncDbConnectionPool> = OnceCell::const_new();

fn create_conn_pool() -> AppResult<DbConnectionPool> {
    // dotenv().ok();
    let run_env = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
    dotenvy::from_filename(format!(".env.{run_env}")).ok(); // success
                                                            // let database_url = env::var("DATABASE_URL").map_err(|e| e)?;
    let db_address = env::var("DB_ADDRESS")?;
    let db_name = env::var("DB_NAME")?;
    let db_username = env::var("DB_USERNAME")?;
    let db_password = env::var("DB_PASSWORD")?;
    let database_url = format!("postgres://{db_username}:{db_password}@{db_address}/{db_name}");
    let manager = diesel::r2d2::ConnectionManager::<RawDbConnection>::new(database_url);

    let pool = diesel::r2d2::Pool::builder()
        .max_size(15) // max_pool_size
        .build(manager)
        .expect("Could not create connection pool");

    Ok(pool)
}

pub async fn create_async_conn_pool() -> AppResult<AsyncDbConnectionPool> {
    let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());
    dotenvy::from_filename(format!(".env.{run_env}")).ok(); // success
                                                            // let database_url = env::var("DATABASE_URL").map_err(|e| e)?;
    let db_address = env::var("DB_ADDRESS")?;
    let db_name = env::var("DB_NAME")?;
    let db_username = env::var("DB_USERNAME")?;
    let db_password = env::var("DB_PASSWORD")?;
    let db_min_pool_size = env::var("DB_MIN_POOL_SIZE")
        .unwrap_or("5".to_string())
        .parse::<u32>()?;
    let db_max_pool_size = env::var("DB_MAX_POOL_SIZE")
        .unwrap_or("5".to_string())
        .parse::<u32>()?;
    let manager = bb8_postgres::PostgresConnectionManager::new(
        format!("host={db_address} user={db_username} password={db_password} dbname={db_name}")
            .parse()?,
        NoTls,
    );
    Ok(bb8::Pool::builder()
        .min_idle(db_min_pool_size)
        .max_size(db_max_pool_size)
        .build(manager)
        .await?)
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
