extern crate edit_distance;
use edit_distance::edit_distance;
use crate::core::sha_demo::sha;

pub fn sensibility_demo(s: &str, t: &str) {
    let sha_s = sha::get_str_hash(s);
    let sha_t = sha::get_str_hash(t);
    let di = edit_distance(s, t);
    let ds = edit_distance(&*sha_s, &*sha_t);
    let diff = di.abs_diff(ds);
    println!("Delta lev dist: {}", diff);
}