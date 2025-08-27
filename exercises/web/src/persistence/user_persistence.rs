use crate::models::user::User;
use anyhow::anyhow;
use utils::error::app_error::AppResult;

pub fn find(page_no: u32, page_size: u32) -> AppResult<Vec<User>> {
    let rs: Vec<User> = vec![];
    Ok(rs)
}
pub fn find_by_id(id: u64) -> AppResult<User> {
    Err(anyhow!("User with id {} not found", id))
}

pub fn inssert(user: &User) -> AppResult<bool> {
    Err(anyhow!("User {} not found", user.id))
}

pub fn update(user: &User) -> AppResult<bool> {
    Err(anyhow!("User {} not found", user.id))
}
pub fn delete(id: u64) -> AppResult<bool> {
    Err(anyhow!("User {} not found", id))
}

#[cfg(test)]
mod tests {
    use log::info;
    use utils::log::configuration::init_logger;

    #[test]
    fn test_find_user() {
        init_logger();
        info!("Test user persistence success!");
    }
}
