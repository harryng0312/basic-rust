use utils::log::configuration::init_logger;
use crate::models::test_rec::TestRecord;

pub(crate) mod models;
pub(crate) mod persistence;

fn main() {
    init_logger();

    let _ = TestRecord{
        id: 0,
        name: "".to_string(),
        available: false,
        created_at: Default::default(),
    };
}
