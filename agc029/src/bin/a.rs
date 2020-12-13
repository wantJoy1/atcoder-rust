use proconio::input;
fn main() {
    
    input!{
        mut s: String
    }

    let mut index_sum = 0;
    let mut w_cnt = 0;
    for (i,c) in s.chars().enumerate() {
        if c=='W' {
            index_sum += i+1;
            w_cnt += 1;
        }
    }
    w_cnt = (1..=w_cnt).sum();
    println!("{}", index_sum-w_cnt);

}