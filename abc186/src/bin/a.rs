use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        w: i32
    }

    println!("{}", n / w);
}
