use proconio::input;
fn main() {
    
    input!{
        n: u32,
        a: [i32; n],
    }

    let all = 3usize.pow(n);
    let odd = a.iter().fold(1, |x,y| match y%2 {
        0 => x*2,
        _ => x*1
    });
    println!("{}", all-odd);
    
}
