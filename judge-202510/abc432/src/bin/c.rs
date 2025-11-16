use proconio::input;

fn main() {
    match solve() {
        Some(answer) => println!("{answer}"),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, x, y): (usize, usize, usize),
        aa: [usize; n],
    }

    let d = y - x;
    let min_a = *aa.iter().min().unwrap();

    aa.iter()
        .map(|&a| {
            let dw = (a - min_a) * y;
            if dw % d == 0 && dw / d <= a {
                Some(a - dw / d)
            } else {
                None
            }
        })
        .sum()
}
