use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut a: [i32; n]
    }

    a.insert(0, 0);
    a.push(0);
    let a = a;

    let mut sum = 0;
    for w in a.windows(2) {
        sum += (w[0] - w[1]).abs();
    }
    let sum = sum;

    for w in a.windows(3) {
        let ai = w[0];
        let aj = w[1];
        let ak = w[2];
        let ans = sum - (ai - aj).abs() - (aj - ak).abs() + (ai - ak).abs();
        println!("{}", ans);
    }
}
