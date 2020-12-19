use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let d = k - 1;
    println!("{}", (n - 1 + d - 1) / d);
}
