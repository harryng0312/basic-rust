use crate::models::user::users::dsl::users;
use crate::models::user::User;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use log::{error, info};
use once_cell::sync::Lazy;
use std::env;
use utils::error::app_error::AppResult;

type RawDbConnection = PgConnection;
type DbConnectionPool = Pool<ConnectionManager<RawDbConnection>>;

pub type DbConnection = PooledConnection<ConnectionManager<RawDbConnection>>;

static DB_CONNECTION_POOL: Lazy<DbConnectionPool> =
    Lazy::new(|| create_conn_pool().expect("Could not create DB connection pool"));

fn create_conn_pool() -> AppResult<DbConnectionPool> {
    // dotenv().ok();
    let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());
    dotenvy::from_filename(format!(".env.{run_env}")).ok(); // success
    let database_url = env::var("DATABASE_URL").map_err(|e| e)?;
    let manager = ConnectionManager::<RawDbConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(15) // max_pool_size
        .build(manager)
        .expect("Could not create connection pool");

    Ok(pool)
}

pub fn get_connection() -> AppResult<DbConnection> {
    let conn_pool = DB_CONNECTION_POOL.get()?;
    // match conn_pool {
    //     Ok(conn) => {
    //         Ok(conn)
    //     }
    //     Err(e) => {
    //         error!("{}", e);
    //         Err(e.into())
    //     }
    // }
    Ok(conn_pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::users::dsl::users;
    use crate::models::user::User;
    use diesel::{QueryDsl, RunQueryDsl};
    use log::{error, info};
    use utils::log::configuration::init_logger;

    #[test]
    fn test_get_conn_pool() -> AppResult<()> {
        init_logger();
        let conn_pool = DB_CONNECTION_POOL.get();

        match conn_pool {
            Ok(mut conn) => {
                info!("Get connection from pool successfully!");
                let _rs = users
                    .limit(3)
                    // .get_results(&mut conn)?;
                    .load::<User>(&mut conn)?;
                for u in _rs {
                    info!("{:?}", u);
                }

                Ok(())
            }
            Err(e) => {
                error!("{}", e);
                Err(e.into())
            }
        }
    }
}
