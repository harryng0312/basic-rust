use std::ops::Range;
use std::time::Duration;

// use std::time::Duration;
use async_std::io;
use chrono::Local;
use rand::Rng;

async fn async_gen_sleep_duration<T>(range: Range<T>) -> T
where
    T: PartialOrd + rand::distributions::uniform::SampleUniform,
{
    let mut rand = rand::thread_rng();
    rand.gen_range(range)
}
async fn async_produce(idx: i32, sender: async_channel::Sender<String>) {
    let sleep_duration = async_gen_sleep_duration(500..2_000).await;
    async_std::task::sleep(Duration::from_millis(sleep_duration)).await;
    // loop {
    let mut _stdout = io::stdout();
    let val = Local::now().to_string();
    if sender.send(val.clone()).await.is_ok() {
        println!("Sent[{}]:{}", idx, val);
    }
    // match sender.send(val.clone()).await {
    //     Ok(..) => {
    //         println!("Sent[{}]:{}", idx, val);
    //         // _stdout.write_all(format!("Sent:{} at:{}\n", val, Local::now()).as_bytes());
    //         // _stdout.flush();
    //         // sender.close();
    //         // break;
    //     }
    //     Err(e) => {}
    // };
    // }
    // time::sleep(time::Duration::from_millis(sleep_duration)).await;
}

async fn async_consume(idx: i32, receiver: async_channel::Receiver<String>) {
    let mut sleep_duration = async_gen_sleep_duration(500..2_000).await;
    async_std::task::sleep(Duration::from_millis(sleep_duration)).await;
    while let Ok(val) = receiver.recv().await {
        println!("Received[{}]:{}", idx, val);
        // match receiver.recv().await {
        // match receiver.recv().await {
        //     Ok(val) => { println!("Received[{}]:{}", idx, val) }
        //     Err(e) => { break 'l }
        // }
    }
    // receiver.close();
}
#[test]
fn test_prod_cons_async_std() {
    let n = 15;
    let m = 15;
    // let mut p_results: Vec<_> = vec![];
    // let mut c_results: Vec<_> = vec![];
    let (sender, receiver) = async_channel::bounded::<String>(5);
    async_std::task::block_on(async {
        let p_results: Vec<_> = (0..n)
            .map(|x| { async_std::task::spawn({ async_produce(x, sender.clone()) }) })
            .collect();
        let c_results: Vec<_> = (0..m)
            .map(|x1| { async_std::task::spawn({ async_consume(x1, receiver.clone()) }) })
            .collect();
        for p_rs in p_results {
            p_rs.await;
        }
        println!("Rest of channel:{}", sender.len());
        sender.close();
        for c_rs in c_results {
            c_rs.await;
        }
        receiver.close();
    });
}
