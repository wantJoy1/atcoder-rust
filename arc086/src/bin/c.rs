use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut v: Vec<usize> = vec![0; n];
    for i in a {
        v[i - 1] += 1;
    }
    v.retain(|&i| i != 0);
    v.sort();

    let dif = if v.len() <= k { 0 } else { v.len() - k };
    let ans = v.into_iter().take(dif).sum::<usize>();
    println!("{}", ans);
}
