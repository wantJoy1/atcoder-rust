use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u128
    }

    let upper: u128 = (1..=11).map(|i| l - i).product();
    let under: u128 = (1..=11).product();
    println!("{}", upper / under);
}
