// pub type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
pub type AppResult<T> = anyhow::Result<T>;
