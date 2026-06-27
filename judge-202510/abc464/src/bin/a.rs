use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let num_e = s.chars().filter(|&ch| ch == 'E').count();
    let num_w = s.len() - num_e;
    let ans = if num_e > num_w { "East" } else { "West" };
    println!("{ans}");
}
