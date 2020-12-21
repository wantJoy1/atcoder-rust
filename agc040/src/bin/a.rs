use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut l = vec![0; s.len() + 1];
    let mut r = vec![0; s.len() + 1];
    for i in 0..s.len() {
        if s[i] == '<' {}
    }
}
