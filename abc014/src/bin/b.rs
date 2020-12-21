use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut x: i32,
        a: [i32; n]
    }

    let mut i = 0usize;
    let mut ans = 0;
    while x != 0 {
        if x % 2 == 1 {
            ans += a[i];
        }
        x = (x - x % 2) / 2;
        i += 1;
    }
    println!("{}", ans);
}
