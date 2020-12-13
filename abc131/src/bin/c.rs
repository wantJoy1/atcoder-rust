use num::integer::lcm;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let lcm = lcm(c * d);
}
