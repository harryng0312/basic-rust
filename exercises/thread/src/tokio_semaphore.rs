use std::sync::Arc;
use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use futures::{stream::FuturesUnordered, StreamExt};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_semaphore() {
    let semaphore = Arc::new(Semaphore::new(4)); // Giới hạn 4 concurrent jobs
    let (tx, mut rx) = mpsc::channel::<u32>(100);

    // Producer
    tokio::spawn({
        let tx = tx.clone();
        async move {
            for i in 0..10 {
                tx.send(i).await.unwrap();
                println!("Produced job {}", i);
            }
        }
    });

    // Worker pool
    let mut workers = FuturesUnordered::new();

    while let Some(job) = rx.recv().await {
        let permit = semaphore.clone().acquire_owned().await.unwrap(); // ✅ Đúng ở đây
        workers.push(tokio::spawn(handle_job(job, permit)));
    }

    // Đợi tất cả workers xong
    while let Some(result) = workers.next().await {
        if let Ok(msg) = result {
            println!("Result: {}", msg);
        }
    }
}

async fn handle_job(job: u32, _permit: OwnedSemaphorePermit) -> String {
    println!("Processing job {}", job);
    sleep(Duration::from_millis(500)).await;
    format!("Job {} done", job)
}