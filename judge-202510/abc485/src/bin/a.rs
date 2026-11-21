use proconio::input;

fn main() {
    input! {
        (a, b): (i32, i32),
    }

    println!(
        "{}",
        if a.signum() == b.signum() {
            "Yes"
        } else {
            "No"
        }
    );
}
