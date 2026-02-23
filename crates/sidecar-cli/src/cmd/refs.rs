use crate::output;

pub fn run(_uid: &str, _limit: u32, _offset: u32, json: bool) -> i32 {
    output::not_implemented("refs", json)
}
