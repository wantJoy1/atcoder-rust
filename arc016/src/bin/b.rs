use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [Chars; n]
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..9usize {
            let ch = x[i][j];
            if ch == 'x' {
                count += 1;
            } else if ch == 'o' {
                count += 1;
                remove_chain(&mut x, i, j, n);
            }
        }
    }
    println!("{}", count);
}

fn remove_chain(v: &mut Vec<Vec<char>>, mut i: usize, j: usize, n: usize) -> () {
    while i < n && v[i][j] == 'o' {
        v[i][j] = '.';
        i += 1;
    }
}
