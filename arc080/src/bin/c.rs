use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }

    let (a_even, a_odd): (Vec<i32>, Vec<i32>) = a.into_iter().partition(|i| i % 2 == 0);
    let no = a_odd.len();
    let (a4, a2): (Vec<i32>, Vec<i32>) = a_even.into_iter().partition(|&i| i % 4 == 0);
    // no+n2+n4 == n
    let (n4, n2) = (a4.len(), a2.len());
    let ans =
    if n2 == 0 {
        if n4+1 >= no {"Yes"}
        else {"No"}
    } else {
        if n4 >= no {"Yes"}
        else {"No"}
    };
    println!("{}", ans);
}
