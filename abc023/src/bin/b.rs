use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        s: String
    }

    // O(kasu)
    let mut acc = String::from("b");
    let mut ans = -1;

    if s == acc {
        ans = 0;
    }

    for i in 1..=n {
        let _ = match i % 3 {
            1 => {
                acc.insert(0, 'a');
                acc.push('c');
            }
            2 => {
                acc.insert(0, 'c');
                acc.push('a');
            }
            _ => {
                acc.insert(0, 'b');
                acc.push('b');
            }
        };
        if s == acc {
            ans = i;
        }
    }

    println!("{}", ans);
}
