use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: i32,
        _: i32,
        a: [String; h]
    }

    let a = a
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut ans = a;

    ans.retain(|v| v.iter().any(|&ch| ch == '#'));
    ans = transpose(ans);
    ans.retain(|v| v.iter().any(|&ch| ch == '#'));
    ans = transpose(ans);

    for v in ans {
        for c in v {
            print!("{}", c);
        }
        println!();
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
