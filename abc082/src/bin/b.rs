use proconio::{input, fastout};

#[fastout]
fn main() {
    
    input!{
        mut s: String,
        mut t: String
    }

    let mut s_: Vec<char> = s.chars().collect();
    let mut t_: Vec<char> = t.chars().collect();
    s_.sort();
    t_.sort(); t_.reverse();
    let s_: String = s_.into_iter().collect();
    let t_: String = t_.into_iter().collect();

    println!("{}", if s_<t_ {"Yes"} else {"No"});

    eprintln!("s_: {}", s_);
    eprintln!("t_: {}", t_);

}
