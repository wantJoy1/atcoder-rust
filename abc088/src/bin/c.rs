use proconio::input;
fn main() {
    
    input!{
        cs: [i32; 9]
    }

    let ans = {
        (cs.iter().take(3).sum::<i32>() + 
        cs.iter().skip(3).take(3).sum::<i32>() -
        cs.iter().skip(6).sum::<i32>() * 2)
        %3 == 0
    };
    println!("{}", if ans {"Yes"} else {"No"});

}
