use proconio::input;
fn main() {
    
    input!{
        a: i64,
        b: i64,
        _: i64,
        k: i64,
    }

    println!("{}", if k%2==0 {a-b} else {b-a});

}
