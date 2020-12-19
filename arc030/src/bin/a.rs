use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32
    }

    println!("{}", if (n / 2).max(1) >= k { "YES" } else { "NO" });
}
