#[cfg(test)]
mod tests {
    use chrono::Local;
    use chrono::NaiveDateTime;
    use macros::{record, Crud};

    use tracing::info;
    use utils::error::app_error::AppResult;
    use utils::log::configuration::init_logger;
    use web::models::sample_rec::SampleRecord;
    use web::persistence::sample_rec_persistence_async::{find, find_by_id, insert_batch};

    #[tokio::test]
    async fn test_find() {
        init_logger();
        let n = 5;
        for i in 0..n {
            let mut _test_recs = find(i, 10).await.unwrap();
            for _test_rec in _test_recs.iter_mut() {
                info!("{:?}", _test_rec);
            }
        }
    }

    #[tokio::test]
    async fn test_find_by_id() {
        init_logger();
        let test_rec = find_by_id(1i64).await.expect("TODO: panic message");
        if let Some(test_rec) = test_rec {
            info!("Found: {:?}", test_rec);
        } else {
            info!("no test rec found");
        }
    }

    #[tokio::test]
    async fn test_insert_batch() {
        init_logger();
        let mut ls_test_recs: Vec<SampleRecord> = vec![];
        for i in 11..=20 {
            let test_rec = SampleRecord::new(
                i,
                format!("name of {}", i),
                false,
                Local::now().naive_local(),
            );
            ls_test_recs.push(test_rec);
        }
        let exec = insert_batch(&ls_test_recs).await;
        match exec {
            Ok(_) => info!("Insert successful"),
            Err(e) => info!("Insert failed:{}", e),
        }
    }

    // record! {
    // #[derive(Crud(table_name="test_rec"))]
    #[derive(Crud)]
    #[crud(table_name = "test_rec", primary_key(id))]
    #[record]
    struct Foo {
        #[column(name = "id_")]
        id: i64,
        #[column(name = "name_")]
        name: String,
        dob: NaiveDateTime,
    }
}
