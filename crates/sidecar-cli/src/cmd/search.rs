use crate::output;

pub fn run(_query: &str, _limit: u32, _offset: u32, json: bool) -> i32 {
    output::not_implemented("search", json)
}
