use proconio::input;
fn main() {
    input! {
        n: u128
    }

    let alphabet: Vec<char> = (b'a'..=b'z').map(|c| c as char).collect();
    let mut v: Vec<u8> = vec![];
    let ans = go(n, &mut v);
    for i in ans.into_iter().rev() {
        print!("{}", alphabet[*i as usize]);
    }
    println!();
}

fn go(q: u128, v: &mut Vec<u8>) -> &mut Vec<u8> {
    if q == 0 {
        v
    } else {
        let r = (q - 1) % 26;
        v.push(r as u8);
        go((q - (r + 1)) / 26, v)
    }
}
