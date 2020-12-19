use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: isize,
        a: isize,
        b: isize
    }

    let d = k - a + 1;
    let add = b - a;
    let ans1 = k + 1;
    let ans2 = a + add * (d / 2) + d % 2;
    println!("{}", if add <= 2 { ans1 } else { ans2 });
}
