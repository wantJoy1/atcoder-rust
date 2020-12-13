use proconio::input;
fn main() {
    
    input!{
        es: [i32; 6],
        b: i32,
        ls: [i32; 6]
    }

    let mut v = vec![];
    for _ in 0..10 { v.push(0) }
    for e in es { v[e as usize] += 1 }
    for &l in &ls { v[l as usize] += 1 }
    let cnt = v.iter().filter(|&&x| x==2).count();
    println!("{}", match cnt {
        6 => 1,
        5 => {
            if ls.contains(&b) {2} else {3}
        },
        4 => 4,
        3 => 5,
        _ => 0
    })

}
