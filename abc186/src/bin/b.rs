use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: i32,
        w: i32,
        a: [i32; h*w]
    }

    let min = a.iter().min().unwrap();
    let ans = a.iter().map(|i| i - min).sum::<i32>();
    println!("{}", ans);
}
