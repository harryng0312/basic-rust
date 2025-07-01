// use std::{sync::Arc, task::{Context, Poll}};
// use futures::{stream::{FuturesUnordered, StreamExt}, Future};
// use tokio::{sync::mpsc, task, time::{sleep, Duration}};
// use tokio_util::sync::PollSemaphore;
// use tokio_stream::wrappers::ReceiverStream;
//
// #[tokio::main]
// async fn main() {
//     let (tx, rx) = mpsc::channel::<u32>(100);
//     let semaphore = Arc::new(PollSemaphore::new(3)); // tối đa 3 task chạy cùng lúc
//
//     // Gửi job
//     tokio::spawn(async move {
//         for i in 0..10 {
//             tx.send(i).await.unwrap();
//             println!("Produced job {}", i);
//             sleep(Duration::from_millis(50)).await;
//         }
//     });
//
//     let mut tasks = FuturesUnordered::new();
//     let mut stream = ReceiverStream::new(rx);
//
//     while let Some(job) = stream.next().await {
//         let sem = semaphore.clone();
//         let fut = async move {
//             // Dùng polling theo kiểu future adapter
//             PermitFuture { semaphore: sem.clone() }.await;
//             process_job(job).await
//         };
//         tasks.push(task::spawn(fut));
//     }
//
//     // Đợi tất cả xong
//     while let Some(Ok(result)) = tasks.next().await {
//         println!("✅ Result: {}", result);
//     }
//
//     println!("🎉 All done");
// }
//
// async fn process_job(job: u32) -> String {
//     println!("⏳ Processing job {}", job);
//     sleep(Duration::from_millis(300)).await;
//     format!("Done job {}", job)
// }
//
// /// Một future wrapper để poll PollSemaphore và chờ permit
// struct PermitFuture {
//     semaphore: Arc<PollSemaphore>,
// }
//
// impl Future for PermitFuture {
//     type Output = ();
//
//     fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match self.semaphore.poll_acquire(cx) {
//             Poll::Ready(Some(_permit)) => Poll::Ready(()), // giữ _permit sống
//             Poll::Ready(None) => Poll::Pending,            // không cấp permit được
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }