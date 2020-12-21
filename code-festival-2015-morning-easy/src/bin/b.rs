use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }

    if n % 2 == 1 {
        println!("-1");
    } else {
        let iter = s.chars();
        let left = iter.clone().take(n / 2);
        let right = iter.skip(n / 2);
        let ans = left.zip(right).filter(|(i, j)| i != j).count();
        println!("{}", ans);
    }
}
