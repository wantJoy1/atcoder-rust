use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut h: [i32; n]
    }

    let mut count = 0;
    while !h.is_empty() && h.contains(&0) {
        let mut h = h
            .clone()
            .into_iter()
            .skip_while(|&i| i == 0)
            .take_while(|&i| i != 0)
            .collect::<Vec<i32>>();
        count += 1;
        for &i in &h {
            if i == 0 {
                count += 1
            }
        }
        for i in 0..h.len() {
            h[i] -= 1;
        }
    }
    println!("{}", count);
}
