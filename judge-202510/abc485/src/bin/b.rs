use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let clogged_faucet = (0..n).find(|&i| (0..m).all(|j| ss[i][j] == 'x'));
    let clogged_drain = (0..m).find(|&j| (0..n).all(|i| ss[i][j] == 'x'));

    if let Some(i) = clogged_faucet {
        println!("Faucet {}", i + 1);
    } else if let Some(j) = clogged_drain {
        println!("Drain {}", j + 1);
    }
}
