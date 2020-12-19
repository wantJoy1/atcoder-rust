use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [i32; 4]
    }

    println!("{}", a.into_iter().min().unwrap());
}
