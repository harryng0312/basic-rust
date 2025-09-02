use ::utils::log::configuration::init_logger;

pub(crate) mod dto;
pub(crate) mod models;
pub(crate) mod persistence;
pub(crate) mod utils;

record! {
    TestStruct {
        id: i64,
    }
}

fn main() {
    init_logger();
    // let rec = TestRecord::new(0, String::default(), false, Default::default());
}
