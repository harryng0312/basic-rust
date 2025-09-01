use crate::models::test_rec::TestRecord;
use utils::log::configuration::init_logger;

pub(crate) mod dto;
pub(crate) mod models;
pub(crate) mod persistence;

fn main() {
    init_logger();
    let rec = TestRecord::new(0, String::default(), false, Default::default());
}
