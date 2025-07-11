use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

#[cfg(test)]
mod test {
    use log::info;
    use std::time::Duration;
    use tokio::join;
    use tokio::runtime::Builder;
    use tokio::time::{sleep, Instant};
    use utils::log::configuration::init_logger;
    #[test]
    fn test_basic_async() {
        init_logger();

        // init carrier threads
        let rt = Builder::new_multi_thread()
            .worker_threads(8)
            .enable_all()
            .build()
            .unwrap();

        // rt.block_on(
        //     async {
        //         info!("test_basic_async");
        //     }
        // );
        let task = rt.spawn(async {
            info!("test_basic_async");
            let start_time = Instant::now();
            sleep(Duration::from_secs_f32(3.5)).await;
            let end_time = Instant::now();
            info!("test_basic_async_time_cost: {:?}", end_time - start_time);
        });
        rt.block_on(task).expect("Await to end all tasks");

        info!("test_basic_async_done");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_basic_async_tokio() {
        init_logger();
        let h1 = tokio::spawn(async {
            sleep(Duration::from_secs_f32(1.5)).await;
            1 + 2
        });
        let h2 = tokio::spawn(async {
            sleep(Duration::from_secs_f32(2.5)).await;
            3 + 4
        });

        let rs = join!(h1, h2);

        assert_eq!(rs.0.unwrap(), 3);
        assert_eq!(rs.1.unwrap(), 7);
    }
}
#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let handle = task::spawn(async {
        // code này chạy trong thread blocking riêng
        sleep(Duration::from_secs_f32(2.4)).await;
        println!("Async task done");
        42
    });

    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
