use proconio::input;
fn main() {
    
    input!{
        n: i32,
        k: i32,
        p: [f64; n]
    }

    let head = vec![0f64].into_iter();
    let sum_table = head.chain(
        p.into_iter().scan(0f64, |sum,a| {
            *sum = *sum+a;
            Some(*sum)
        }
    ));

    let mut max = 0f64;
    let clone1 = sum_table.clone();
    let clone2 = clone1.clone();

    let left  = clone1.into_iter().take((n-k+1) as usize);
    let right = clone2.into_iter().skip(k as usize);

    for (l,r) in left.zip(right) {
        let exp = (r-l+k as f64)/2.0;
        max = max.max(exp);
    }
    println!("{}", max);

}
