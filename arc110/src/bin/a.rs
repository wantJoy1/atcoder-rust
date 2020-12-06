use num::integer::lcm;
use proconio::input;
fn main() {
    
    input!{
        n: u64
    }

    let lcm = (2..=n).fold(1, |a,b| lcm(a,b));
    println!("{}", lcm+1);
    
}
