use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut d: [i32; n],
        m: i32,
        mut t: [i32; m]
    }

    d.sort();
    d.reverse();
    t.sort();
    for i in d {
        /*
        if t.len() > 0 && i == t[t.len() - 1] {
            t.pop();
        }
        */
        let _ = match t.last() {
            Some(j) if i == *j => t.pop(),
            _ => None,
        };
    }
    println!("{}", if t.is_empty() { "YES" } else { "NO" });
}
