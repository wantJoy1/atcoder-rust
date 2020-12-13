use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }

    let mut k: u64 = 1;
    let mut sum: u64 = 0;
    while sum <= n + 1 {
        sum += k;
        k += 1;
    }
}
