use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(10);
    for i in 0..3 {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("Receiver {} got: {}", i, msg);
            }
        });
    }

    for i in 0..5 {
        tx.send(i).unwrap();
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}