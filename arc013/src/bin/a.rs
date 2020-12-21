use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: (i32, i32, i32, i32, i32, i32)
    }

    let (a, b, c, d, e, f) = t;
    let mut ans = 0;
    ans = ans.max((a / d) * (b / e) * (c / f));
    ans = ans.max((a / d) * (b / f) * (c / e));
    ans = ans.max((a / e) * (b / d) * (c / f));
    ans = ans.max((a / e) * (b / f) * (c / d));
    ans = ans.max((a / f) * (b / d) * (c / e));
    ans = ans.max((a / f) * (b / e) * (c / d));

    println!("{}", ans);
}
