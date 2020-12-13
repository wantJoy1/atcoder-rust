use proconio::input;
use proconio::marker::Chars;
fn main() {
    
    input!{
        n: Chars
    }

    let ans1 = &n.iter().map(|c| as_int(*c)).sum::<usize>();
    let ans2 = {
        let mut tmp_sum: usize = 0;
        tmp_sum += (n.len()-1)*9;
        tmp_sum += as_int(n[0])-1;
        tmp_sum
    };
    let ans = ans1.max(&ans2);
    println!("{}", ans);

}

fn as_int(c: char) -> usize {
    c as usize - 48
}