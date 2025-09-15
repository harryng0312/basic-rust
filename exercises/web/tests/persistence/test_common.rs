#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, Utc};
    use diesel::{QueryDsl, RunQueryDsl};
    use tokio::pin;
    use tokio_postgres::RowStream;
    use tokio_stream::StreamExt;
    use tracing::info;
    use utils::error::app_error::AppResult;
    use utils::log::configuration::init_logger;
    use web::models::sample_rec::sample_recs::dsl::sample_recs;
    use web::models::sample_rec::SampleRecord;
    use web::persistence::common::{get_async_connection, get_connection};

    #[test]
    fn test_get_conn_pool() -> AppResult<()> {
        init_logger();
        let mut conn = get_connection()?;
        info!("Get connection from pool successfully!");
        let _rs = sample_recs
            .limit(3)
            // .get_results(&mut conn)?;
            .load::<SampleRecord>(&mut conn)?;
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
        let sql = "SELECT id_, name_, available, created_at FROM test_rec where created_at < $1 order by id_ DESC limit $2";
        let now = Utc::now().naive_utc();
        let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&now, &5i64];
        let rows: RowStream = conn.query_raw(sql, params).await?;
        pin!(rows);
        while let Some(row) = rows.next().await {
            if let Ok(row) = row {
                let test_rec = SampleRecord::new(
                    row.get("id_"),
                    row.get("name_"),
                    row.get("available"),
                    row.get::<_, NaiveDateTime>("created_at"),
                );
                info!("{:#?}", test_rec);
            }
        }
        Ok(())
    }
}
