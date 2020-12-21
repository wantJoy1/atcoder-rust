use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut has_7 = false;

        let mut j = i;
        while !has_7 && j != 0 {
            if j % 8 == 7 {
                has_7 = true;
            }
            j = (j - (j % 8)) / 8;
        }

        let mut j = i;
        while !has_7 && j != 0 {
            if j % 10 == 7 {
                has_7 = true;
            }
            j = (j - (j % 10)) / 10;
        }

        if has_7 {
            ans += 1;
        }
    }
    println!("{}", n - ans);
}
