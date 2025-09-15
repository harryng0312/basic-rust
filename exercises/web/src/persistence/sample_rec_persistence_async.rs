use crate::models::sample_rec::SampleRecord;
use crate::persistence::common::get_async_connection;
use anyhow::anyhow;
use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use tokio::pin;
use tokio_postgres::types::ToSql;
use tokio_postgres::GenericClient;
use tokio_stream::StreamExt;
use utils::error::app_error::AppResult;

pub async fn find(page_no: u32, page_size: u32) -> AppResult<Vec<SampleRecord>> {
    let conn = get_async_connection().await?;
    let mut result: Vec<SampleRecord> = vec![];

    let offset_val = (page_no * page_size) as i64;
    let page_size = page_size as i64;
    let sql = "select id_, name_, available, created_at from test_rec order by created_at desc limit $2 offset $1";
    let rows = conn.query_raw(sql, &[offset_val, page_size]).await?;
    pin!(rows);
    while let Some(row) = rows.next().await {
        if let Ok(row) = row {
            let rec = SampleRecord::new(
                row.get("id_"),
                row.get("name_"),
                row.get("available"),
                row.get::<_, NaiveDateTime>("created_at"),
            );
            result.push(rec);
        }
    }
    Ok(result)
}
pub async fn find_by_id(_id: i64) -> AppResult<Option<SampleRecord>> {
    let conn = get_async_connection().await?;
    let sql = "select id_, name_, available, created_at from test_rec where id_ = $1";
    let row = conn.query_opt(sql, &[&_id]).await?;
    match row {
        Some(row) => Ok(Some(SampleRecord::new(
            row.get("id_"),
            row.get("name_"),
            row.get("available"),
            row.get::<_, NaiveDateTime>("created_at"),
        ))),
        None => Ok(None),
    }
}
pub async fn insert(val: &SampleRecord) -> AppResult<()> {
    let conn = get_async_connection().await?;
    let sql = "insert into test_rec (id_, name_, available, created_at) values ($1, $2, $3, $4)";
    let _ = conn
        .execute(sql, &[&val.id, &val.name, &val.available, &val.created_at])
        .await?;
    Ok(())
}
pub async fn insert_batch(vals: &[SampleRecord]) -> AppResult<()> {
    let mut conn = get_async_connection().await?;
    let tx = conn.transaction().await?;
    // let mut sql_batch: Vec<String> = vec![];
    let sql: &str =
        "insert into test_rec (id_, name_, available, created_at) values ($1, $2, $3, $4)";
    let stmt = tx.prepare(sql).await?;
    for (idx, val) in vals.iter().enumerate() {
        let params: &[&(dyn ToSql + Sync)] = &[&val.id, &val.name, &val.available, &val.created_at];
        let exec = tx
            .execute(
                &stmt,
                &[&val.id, &val.name, &val.available, &val.created_at],
            )
            .await?;
        // let sql = format!(
        //     "insert into test_rec (id_, name_, available, created_at) values (${}::INT8, ${}::TEXT, ${}::BOOL, ${}::TIMESTAMP)",
        //     idx * params.len() + 1,
        //     idx * params.len() + 2,
        //     idx * params.len() + 3,
        //     idx * params.len() + 4
        // );
        // let portal = tx.bind(&stmt, params).await?;
        // sql_batch.push(String::from(sql));
    }
    // let exec = tx.batch_execute(sql_batch.join(";").as_str()).await;
    let exec = tx.commit().await;
    match exec {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}
pub async fn update(val: &SampleRecord) -> AppResult<()> {
    todo!()
}
pub async fn delete(_id: i64) -> AppResult<()> {
    todo!()
}
