use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i128,
        b: i128,
        n: i128
    }

    let i = if b <= n { b - 1 } else { n };
    let ans = (a * i) / b - a * (i / b);
    println!("{}", ans);
}
