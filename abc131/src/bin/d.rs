use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut ab: [(i32, i32); n]
    }

    ab.sort_by_key(|t| t.1);
    let ab = ab;
}
