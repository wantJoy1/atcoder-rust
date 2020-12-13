use proconio::input;
use std::collections::HashMap;
fn main() {
    
    input!{
        n: i32,
        _: i32,
        aa: [i32; n]
    }

    let mut map = HashMap::<i32,i32>::new();
    for &a in &aa { 
        *map.entry(a).or_insert(0) += 1;
    }
    let mut max = 0;
    for (_,v) in &map {
        max = max.max(*v);
    }
    map.retain(|_,&mut v| v==max);
    if n<max*2 && map.len()==1 {
        println!("{}",map.keys().into_iter().next().unwrap());
    } else {
        println!("?");
    };

}
