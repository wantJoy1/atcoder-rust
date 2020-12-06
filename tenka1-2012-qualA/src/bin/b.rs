fn main() {

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let cs: String = s.trim().parse().ok().unwrap();
    let mut ss = cs.split_whitespace();
    print!("{}", ss.next().unwrap());
    for s in ss {
        print!(",{}", s);
    }
    println!();

}
