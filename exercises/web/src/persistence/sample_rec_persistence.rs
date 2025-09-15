use crate::models::sample_rec::sample_recs::{dsl::sample_recs, id};
use crate::models::sample_rec::SampleRecord;
use crate::persistence::common::get_connection;
use anyhow::anyhow;
use diesel::dsl::insert_into;
use diesel::result::Error;
use diesel::OptionalExtension;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use utils::error::app_error::AppResult;

pub fn find(page_no: u32, page_size: u32) -> AppResult<Vec<SampleRecord>> {
    let mut conn = get_connection()?;
    let offset_val = page_no * page_size;
    let rs = sample_recs
        .order(id.desc())
        .offset(offset_val as i64)
        .limit(page_size as i64)
        .load::<SampleRecord>(&mut conn)?;

    Ok(rs)
}
pub fn find_by_id(_id: i64) -> AppResult<Option<SampleRecord>> {
    let mut conn = get_connection()?;
    let rs = sample_recs
        .filter(id.eq(_id))
        .first::<SampleRecord>(&mut conn)
        .optional();
    match rs {
        Ok(Some(rs)) => Ok(Some(rs)),  // found
        Ok(None) => Ok(None),          // not found
        Err(err) => Err(anyhow!(err)), // system error
    }
}
pub fn insert(val: &SampleRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    insert_into(sample_recs).values(val).execute(&mut conn)?;
    Ok(())
}
pub fn insert_batch(vals: &[SampleRecord]) -> AppResult<()> {
    let mut conn = get_connection()?;
    conn.transaction::<(), Error, _>(|connection| {
        insert_into(sample_recs).values(vals).execute(connection)?;
        Ok(())
    })?;
    let trans = ();
    Ok(())
}
pub fn update(val: &SampleRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    let upd = diesel::update(sample_recs.filter(id.eq(val.id())))
        .set(val)
        .execute(&mut conn)?;
    Ok(())
}
pub fn delete(_id: i64) -> AppResult<()> {
    let mut conn = get_connection()?;
    let rs = diesel::delete(sample_recs.filter(id.eq(_id))).execute(&mut conn)?;
    Ok(())
}
