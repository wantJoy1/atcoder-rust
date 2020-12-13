use proconio::input;
use std::process::exit;
fn main() {
    
    input!{
        n: i32,
        m: i32,
        sc: [(i32, u8); m]
    }

    let mut cnt: Vec<i32> = vec![];
    let mut ints: Vec<u8> = vec![];
    for _ in 0..n {
        cnt.push(0);
        ints.push(0);
    }

    for (s,c) in sc {
        let i = s as usize - 1;
        if cnt[i]!=0 && ints[i]!=c {
            println!("-1");
            exit(0);
        }
        ints[i] = c;
        cnt[i] += 1;
    }

    if n==1 {
        println!("{}", ints[0]);
    } else {
        if ints[0]==0 && cnt[0]!=0 {
            println!("-1");
            exit(0);
        } else {
            print!("{}", if cnt[0]==0 {1} else {ints[0]});
            for i in 1..n as usize {
                print!("{}", ints[i]);
            }
        }
    }

}
