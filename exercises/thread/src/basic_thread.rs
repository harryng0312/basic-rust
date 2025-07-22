use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

use chrono::Local;
use rand::{Rng, RngCore};
use tokio::runtime::Builder;
use tokio::{task, time};

#[test]
fn test_thread_joint() {
    let mut threads = Vec::<JoinHandle<()>>::new();
    let arc_stdout = Arc::new(Mutex::new(io::stdout()));
    println!("Is parallelism:{:?}", thread::available_parallelism());
    for i in 0..20 {
        // let stdout = Arc::clone(&arc_stdout);
        let t = thread::spawn(move || {
            {
                // let mut stdlock_inner = stdout.lock().unwrap();
                // stdlock_inner.write_all(format!("ProcessId:{}\n", process::id()).as_bytes()).unwrap();
                // stdlock_inner.flush().unwrap();
                println!("Thread ID: {:?}", thread::current().id());
            }
            thread::sleep(Duration::from_millis(10_000));
        });
        println!("Started thread:{}", i);
        threads.push(t);
    }
    for jh in threads {
        jh.join().unwrap();
    }
    println!("{:=^}", "Done");
}

async fn async_task(id: usize) {
    let mut sleep_duration = 0u64;
    {
        let mut rand = rand::rng();
        sleep_duration = rand.random_range(500..20_000);
    }
    println!("Task {} started", id);
    let start = Local::now();
    tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    let end = Local::now();
    println!(
        "Task {} completed in:{}",
        id,
        (end - start).num_milliseconds()
    );
}

#[test]
fn test_async() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut handles = vec![];
        for i in 0..10 {
            handles.push(task::spawn(async_task(i)));
        }
        for handle in handles {
            handle.await.unwrap();
        }
    });
    rt.shutdown_background();
}

async fn produce(sender: Arc<Sender<String>>) {
    let mut rand = rand::thread_rng();
    // let sleep_duration = rand.gen_range(500..20_000);
    let val = rand.next_u64();
    println!("Sent:{}", val);
    sender.send(val.to_string()).unwrap_or_default();
    // time::sleep(time::Duration::from_millis(sleep_duration)).await;
}

async fn consume(receiver: Receiver<String>) {
    // 'l: loop {
    //     match receiver.recv_timeout(Duration::from_secs(5)) {
    //         Ok(val) => { println!("Received:{}", val) }
    //         err => { break 'l }
    //     }
    // }
    'l: while let val = receiver.recv_timeout(Duration::from_secs(5)) {
        match val {
            Ok(val) => {
                println!("Received:{}", val)
            }
            err => break 'l,
        }
    }
}

#[test]
fn test_prod_cons() {
    let n = 5;

    let mut rt = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(n + 1)
        .build()
        .unwrap();

    rt.block_on(async {
        let (sender, receiver) = mpsc::channel::<String>();
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
