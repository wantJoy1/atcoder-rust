use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }

    let mut v: Vec<char> = vec![];
    for ch in s.chars() {
        if ch == 'B' {
            v.pop();
        } else {
            v.push(ch);
        }
    }

    for ch in v {
        print!("{}", ch);
    }
    println!();
}
