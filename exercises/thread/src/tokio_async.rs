#[cfg(test)]
mod test {
    use log::info;
    use utils::log::configuration::init_logger;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_async() {
        init_logger();
        info!("Hello");
    }
}
