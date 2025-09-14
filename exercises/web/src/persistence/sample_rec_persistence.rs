use crate::models::sample_rec::sample_recs::dsl::sample_recs;
use crate::models::sample_rec::sample_recs::id;
use crate::models::sample_rec::SampleRecord;
use crate::persistence::common::get_connection;
use anyhow::anyhow;
use diesel::dsl::insert_into;
use diesel::result::Error;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::info;
use utils::error::app_error::AppResult;

fn find(page_no: u32, page_size: u32) -> AppResult<Vec<SampleRecord>> {
    let mut conn = get_connection()?;
    let offset_val = page_no * page_size;
    let rs = sample_recs
        .order(id.desc())
        .offset(offset_val as i64)
        .limit(page_size as i64)
        .load::<SampleRecord>(&mut conn)?;

    Ok(rs)
}
fn find_by_id(_id: i64) -> AppResult<Option<SampleRecord>> {
    Err(anyhow!("Test Record with id {} not found", _id))
}

fn insert(val: &SampleRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    insert_into(sample_recs).values(val).execute(&mut conn)?;
    Ok(())
}

fn insert_batch(vals: &Vec<SampleRecord>) -> AppResult<()> {
    let mut conn = get_connection()?;
    conn.transaction::<(), Error, _>(|connection| {
        insert_into(sample_recs).values(vals).execute(connection)?;
        Ok(())
    })?;
    let trans = ();
    Ok(())
}

fn update(val: &SampleRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    let upd = diesel::update(sample_recs.filter(id.eq(val.id())))
        .set(val)
        .execute(&mut conn)?;
    info!("Updated {} records", upd);
    Ok(())
}
fn delete(_id: u64) -> AppResult<()> {
    Err(anyhow!("Test Record {} not found", _id))
}

#[cfg(test)]
mod tests {
    use crate::models::sample_rec::SampleRecord;
    use crate::persistence::sample_rec_persistence::{find, insert_batch};
    use chrono::Local;
    use tokio::runtime::Runtime;
    use tracing::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_find() {
        init_logger();
        let n = 100;
        for i in 0..n {
            let mut _test_recs = find(0, 10).unwrap();
            for _test_rec in _test_recs.iter_mut() {
                info!("{:?}", _test_rec);
            }
        }
    }

    // #[tests]
    fn test_insert() {
        init_logger();
        let mut ls_test_recs: Vec<SampleRecord> = vec![];
        for i in 1..=10 {
            // let _val = insert(&TestRecord {
            //     id: i,
            //     name: format!("name of {}", i),
            //     available: i % 2 == 0,
            //     created_at: Local::now().naive_local(),
            // })
            // .expect(format!("insert failed at {} step", i).as_str());
            let test_rec = SampleRecord::new(
                i,
                format!("name of {}", i),
                false,
                Local::now().naive_local(),
            );
            ls_test_recs.push(test_rec);
        }
        insert_batch(&ls_test_recs).unwrap();

        let rt = Runtime::new().unwrap();

        // let result = rt.block_on(async {
        //     let conn = get_async_connection().await?;
        //     ()
        // });

        info!("Insert successful");
    }
}
