use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i32; n]
    }

    let mut v = x.clone();
    v.sort();
    let v = v;

    let median1 = v[n / 2 - 1];
    let median2 = v[n / 2];
    for i in x {
        let ans = if i <= median1 { median2 } else { median1 };
        println!("{}", ans);
    }
}
