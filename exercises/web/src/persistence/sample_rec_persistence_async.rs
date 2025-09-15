use crate::models::sample_rec::SampleRecord;
use crate::persistence::common::get_async_connection;
use chrono::NaiveDateTime;
use tokio::pin;
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
    todo!()
}
pub async fn insert(val: &SampleRecord) -> AppResult<()> {
    todo!()
}
pub async fn insert_batch(vals: &Vec<SampleRecord>) -> AppResult<()> {
    todo!()
}
pub async fn update(val: &SampleRecord) -> AppResult<()> {
    todo!()
}
pub async fn delete(_id: i64) -> AppResult<()> {
    todo!()
}
