use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
        l: i64,
    }

    if a * l > b {
        let remain = k % l;
        let ans = (k / l) * b + (remain * a).min(b);
        println!("{}", ans);
    } else {
        println!("{}", k * a);
    }
}
