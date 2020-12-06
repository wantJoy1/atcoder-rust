use proconio::input;
fn main() {
    
    input!{
        n: usize,
        t: String
    }

    let x = 1_00000_00000usize;

    let t_contains_0: bool = "101".repeat((n+2)/3+1)
        .match_indices(t.as_str()).count() > 0;

    let ans = if t_contains_0 { match t.as_str() {
            "1" => {2*x},
            "11" => {x},
            _ => {
                let k = t.match_indices("0").count();
                let last = &t[n-1..n];
                if last=="0" {x-k+1}
                else {x-k}
            }
        }
    } else {0};
    println!("{}", if t_contains_0 {ans} else {0});

    /*
    if n>3 {
        let i: usize = (n+2)/3;
        let short_s = "110".repeat(i as usize+1);
        let count = short_s.match_indices(t.as_str()).count();
        let ans = (10000000000-i+1)*count;
        println!("{}", ans);
    } else {
        let count = "110".match_indices(t.as_str()).count();
        let ans = 10000000000*count;
        println!("{}", ans);
    }
    */
    

}
