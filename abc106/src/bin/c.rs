use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        k: usize
    }

    let ans = if s.clone().chars().take(k - 1).all(|ch| ch == '1') {
        s.chars().nth(k - 1).unwrap()
    } else {
        s.chars().skip_while(|&ch| ch == '1').nth(0).unwrap()
    };
    println!("{}", ans);
}
