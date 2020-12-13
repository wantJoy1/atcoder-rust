use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut sum: usize = 0;
    for (i, w) in s.windows(2).enumerate() {
        dbg!(sum);
        dbg!(w);
        dbg!(i);
        let (prev, next) = (w[0], w[1]);
        if prev == '>' && next == '<' {
            sum += i;
        }
    }
}
