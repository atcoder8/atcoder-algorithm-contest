use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.chars().skip_while(|&ch| ch == 'o').collect::<String>();
    if !ans.is_empty() {
        println!("{ans}");
    }
}
