use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: String
    }

    while !x.is_empty() {
        if x.ends_with("ch") {
            x.pop();
            x.pop();
        } else if x.ends_with("o") {
            x.pop();
        } else if x.ends_with("k") {
            x.pop();
        } else if x.ends_with("u") {
            x.pop();
        } else {
            break;
        }
    }

    println!("{}", if x.is_empty() { "YES" } else { "NO" });
}
