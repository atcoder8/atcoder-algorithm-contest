use itertools::Itertools;
use proconio::input;

const MAX: usize = 3 * 10_usize.pow(5);

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut imos = vec![0_i64; MAX + 1];
    imos[0] = n as i64;
    for &a in &aa {
        imos[a] -= 1;
    }
    for i in 0..MAX {
        imos[i + 1] += imos[i];
    }

    for i in 0..MAX {
        imos[i + 1] += imos[i] / 10;
        imos[i] %= 10;
    }
    while *imos.last().unwrap() == 0 {
        imos.pop();
    }
    imos.reverse();

    println!("{}", imos.iter().join(""));
}
