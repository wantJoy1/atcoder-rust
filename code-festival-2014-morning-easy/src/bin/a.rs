use proconio::input;
fn main() {
    
    input!{
        n: i32,
        a: [f64; n]
    }

    println!("{}", (a[(n-1) as usize]-a[0])/(n-1) as f64);
}
