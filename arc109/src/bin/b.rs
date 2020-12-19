use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64
    }

    let bs = binary_search(0, n as u128 + 3, n as u128) as i64;
    let ans = 1 + (n - (bs + 1) + 1);
    println!("{}", ans);
}

fn binary_search(l: u128, r: u128, n: u128) -> u128 {
    let i = (l + r) / 2u128;
    let sum = i * (i + 1);
    if i * (i + 1) / 2 <= n + 1 && n + 1 < (i + 1) * (i + 2) / 2 {
        i
    } else if sum <= 2 * (n + 1) {
        binary_search(i, r, n)
    } else {
        binary_search(l, i, n)
    }
}
