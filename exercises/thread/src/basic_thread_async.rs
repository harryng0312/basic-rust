use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;
use std::time::Duration;

use chrono::Local;
use rand::RngCore;

async fn produce(sender: Arc<SyncSender<String>>) {
    let mut rand = rand::rng();
    // let sleep_duration = rand.gen_range(500..20_000);
    let val = rand.next_u64();
    println!("Sent:{} at:{}", val, Local::now());
    sender.send(val.to_string()).unwrap_or_default();
    // time::sleep(time::Duration::from_millis(sleep_duration)).await;
}

async fn consume(receiver: Receiver<String>) {
    'l: loop {
        match receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(val) => {
                println!("Received:{} at:{}", val, Local::now())
            }
            err => break 'l,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::basic_thread_async::{consume, produce};
    use std::sync::{mpsc, Arc};
    use std::time::Duration;
    use tokio::runtime::Builder;
    use tokio::{task, time};

    #[test]
    fn test_prod_cons() {
        let n = 5;

        let mut rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(n + 1)
            .build()
            .unwrap();

        rt.block_on(async {
            let (sender, receiver) = mpsc::sync_channel::<String>(3);
            let sender = Arc::new(sender);
            // let receiver = Arc::new(receiver);
            let mut handles: Vec<task::JoinHandle<()>> = vec![];
            // create n producers
            for _ in 0..n {
                let sender_cloned = sender.clone();
                let prod_t = task::spawn(async {
                    time::sleep(Duration::from_millis(200)).await;
                    produce(sender_cloned).await;
                });
                handles.push(prod_t);
            }
            // create 1 consumers
            let cons_t = task::spawn({ consume(receiver) });
            handles.push(cons_t);
            // shutdown
            println!("Awaiting...");
            // time::sleep(Duration::from_millis(10_000)).await;
            for handle in handles {
                handle.await.unwrap();
            }
        });
        rt.shutdown_background();
    }
    #[test]
    fn test_async_channel() {
        let (sender, receiver) = async_channel::bounded(10);
        async_std::task::block_on(async {
            for i in 0..3 {
                let receiver = receiver.clone();
                async_std::task::spawn(async move {
                    while let Ok(msg) = receiver.recv().await {
                        println!("Receiver {} got: {}", i, msg);
                    }
                });
            }
            for i in 0..5 {
                sender.send(i).await.unwrap();
            }
            async_std::task::sleep(Duration::from_secs(1)).await;
        });
    }
}
