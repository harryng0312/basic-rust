#[cfg(test)]
mod tests {
    use tracing::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_find_user() {
        init_logger();
        info!("Test user persistence success!");
    }
}
