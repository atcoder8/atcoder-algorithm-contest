use proconio::input_interactive;

fn main() {
    input_interactive!(n: usize);

    let mut num_combs = 0_usize;
    let mut right = 0;
    for left in 0..n {
        right = right.max(left + 1);
        while right < n {
            println!("? {} {}", left + 1, right + 1);
            input_interactive!(response: String);
            if response == "No" {
                break;
            }
            right += 1;
        }

        num_combs += right - left - 1;
    }

    println!("! {num_combs}");
}
