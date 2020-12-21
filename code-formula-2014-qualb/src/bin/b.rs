use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String
    }

    let mut odd = 0;
    let mut even = 0;
    for (i, ch) in n.chars().rev().enumerate() {
        let j = ch as i32 - 48;
        if (i + 1) % 2 == 1 {
            odd += j;
        } else {
            even += j;
        }
    }
    println!("{} {}", even, odd);
}
