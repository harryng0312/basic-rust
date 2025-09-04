use crate::models::test_rec::test_recs::dsl::test_recs;
use crate::models::test_rec::test_recs::id;
use crate::models::test_rec::TestRecord;
use crate::persistence::common::get_connection;
use anyhow::anyhow;
use diesel::dsl::insert_into;
use diesel::result::Error;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use utils::error::app_error::AppResult;

fn find(page_no: u32, page_size: u32) -> AppResult<Vec<TestRecord>> {
    let mut conn = get_connection()?;
    let offset_val = page_no * page_size;
    let rs = test_recs
        .order(id.desc())
        .offset(offset_val as i64)
        .limit(page_size as i64)
        .load::<TestRecord>(&mut conn)?;

    Ok(rs)
}
fn find_by_id(_id: i64) -> AppResult<Option<TestRecord>> {
    Err(anyhow!("Test Record with id {} not found", _id))
}

fn insert(val: &TestRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    insert_into(test_recs).values(val).execute(&mut conn)?;
    Ok(())
}

fn insert_batch(vals: &Vec<TestRecord>) -> AppResult<()> {
    let mut conn = get_connection()?;
    conn.transaction::<(), Error, _>(|connection| {
        insert_into(test_recs).values(vals).execute(connection)?;
        Ok(())
    })?;
    let trans = ();
    Ok(())
}

fn update(val: &TestRecord) -> AppResult<()> {
    let mut conn = get_connection()?;
    let upd = diesel::update(test_recs.filter(id.eq(val.id())))
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
    use crate::models::test_rec::TestRecord;
    use crate::persistence::test_rec_persistence::{find, insert_batch};
    use chrono::Local;
    use tracing::info;
    use tokio::runtime::Runtime;
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

    // #[test]
    fn test_insert() {
        init_logger();
        let mut ls_test_recs: Vec<TestRecord> = vec![];
        for i in 1..=10 {
            // let _val = insert(&TestRecord {
            //     id: i,
            //     name: format!("name of {}", i),
            //     available: i % 2 == 0,
            //     created_at: Local::now().naive_local(),
            // })
            // .expect(format!("insert failed at {} step", i).as_str());
            let test_rec = TestRecord::new(
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
