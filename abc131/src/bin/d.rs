use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut ab: [(i64, i64); n]
    }

    ab.sort_by_key(|t| t.1);
    let ab = ab;
    let mut time: i64 = 0;
    let mut flag = true;
    for (a, b) in ab {
        time += a;
        if time > b {
            flag = false;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}
