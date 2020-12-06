use proconio::input;
fn main() {
    
    input!{
        m: i32,
        n: i32,
        mut a: i32
    }
    
    let mut sum = a;
    let mut remain = 0;
    while m<=a+remain {
        let tmp = a+remain;
        a = (tmp/m)*n;
        sum += a;
        remain = tmp%m;
    }
    println!("{}", sum);

}
