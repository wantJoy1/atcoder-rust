use proconio::input;
fn main() {
    
    input!{
        n: i32,
        a: [i32; n]
    }

    let mut v: Vec<i32> = vec![];
    let mut max = 0;
    for &i in &a[..] {
        max = max.max(i);
    }
    for _ in 0..=max+10 {
        v.push(0);
    }
    for i in a {
        let j = i as usize;
        v[j] += 1;
        if 0<=i-1 {
            v[j-1] += 1;
        }
        if i+1<=max-1 {
            v[j+1] += 1;
        }
    }
    let ans = v.iter().max().unwrap();
    println!("{}", ans);
    
}
