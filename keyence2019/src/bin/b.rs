use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }

    let n = s.len();
    let d = n - 7;
    let mut flag = false;
    for i in 1..=n - d {
        if "keyence".to_string() == remove(i, d, s.clone()) {
            flag = true;
        }
    }
    println!("{}", if flag { "YES" } else { "NO" });

    /*
    let ans = {
        s.starts_with("keyence")
            || s.starts_with("k") && s.ends_with("eyence")
            || s.starts_with("ke") && s.ends_with("yence")
            || s.starts_with("key") && s.ends_with("ence")
            || s.starts_with("keye") && s.ends_with("nce")
            || s.starts_with("keyen") && s.ends_with("ce")
            || s.starts_with("keyenc") && s.ends_with("e")
            || s.ends_with("keyence")
    };
    println!("{}", if ans { "YES" } else { "NO" });
    */
}

fn remove(i: usize, d: usize, s: String) -> String {
    let left: String = s.clone().chars().take(i - 1).collect();
    let right: String = s.chars().skip(i + d - 1).collect();
    format!("{}{}", left, right)
}
