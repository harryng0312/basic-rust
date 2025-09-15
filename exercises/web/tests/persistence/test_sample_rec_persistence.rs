#[cfg(test)]
mod tests {
    use chrono::Local;
    use tokio::runtime::Runtime;
    use tracing::info;
    use utils::log::configuration::init_logger;
    use web::models::sample_rec::SampleRecord;
    use web::persistence::sample_rec_persistence::{find, find_by_id, insert_batch};

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

    #[test]
    fn test_find_by_id() {
        init_logger();
        let test_rec = find_by_id(1i64).expect("TODO: panic message");
        if let Some(test_rec) = test_rec {
            info!("Found: {:?}", test_rec);
        } else {
            info!("no test rec found");
        }
    }

    #[test]
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
