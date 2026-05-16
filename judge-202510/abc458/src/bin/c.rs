use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            if ch == 'C' {
                (i + 1).min(s.len() - i)
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{ans}");
}
