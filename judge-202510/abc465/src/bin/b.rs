use proconio::input;

fn main() {
    input! {
        (x, y): (u32, u32),
        (l, r): (u32, u32),
        (a, b): (u32, u32),
    }

    let cost = (a..b)
        .map(|i| if l <= i && i < r { x } else { y })
        .sum::<u32>();
    println!("{cost}");
}
