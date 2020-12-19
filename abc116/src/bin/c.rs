use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    }

    let mut ans = 0;
    while h.iter().any(|&i| i != 0) {
        ans += 1;
        let mut i = h.iter().position(|&i| i != 0).unwrap();
        while i < n && h[i] != 0 {
            h[i] -= 1;
            i += 1;
        }
    }
    println!("{}", ans);
}
