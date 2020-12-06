use proconio::input;
fn main() {
    
    input!{
        x: i128,
        k: i128,
        d: i128
    }

    let x = x.abs();
    let ans = if k*d<=x {
        x-k*d
    } else {
        let ans1 = x%d;
        let ans2 = (ans1-d).abs();
        let move_count = k-(x-ans1)/d;
        if move_count%2==0 {ans1} else {ans2}
    };
    println!("{}", ans);
    
}
