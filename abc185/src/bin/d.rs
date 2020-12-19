use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: u32,
        mut a: [u32; m]
    }

    a.push(0);
    a.push(n + 1);
    a.sort();
    let a = a;
    // let lens = a.windows(2).map(|t| t.2 - t.1 - 1);
    let lens = a.windows(2).map(|w| w[1] - w[0] - 1).collect::<Vec<u32>>();
    let mut min: u32 = std::u32::MAX;
    for i in lens.clone() {
        if i != 0 {
            min = min.min(i);
        }
    }
    let mut ans = 0;
    if min != 0 {
        for i in lens {
            ans += (i + min - 1) / min;
        }
    }
    println!("{}", if min != 0 { ans } else { 0 });
}
