use proconio::{input, marker::Usize1};

const MAX: usize = 10_usize.pow(6);

fn main() {
    input! {
        (n, d): (usize, usize),
        st: [(Usize1, usize); n],
    }

    let mut imos = vec![0_i64; MAX + 1];
    for &(s, t) in &st {
        imos[s] += 1;
        imos[t.saturating_sub(d).max(s)] -= 1;
    }
    for i in 0..MAX {
        imos[i + 1] += imos[i];
    }

    let num_combs = imos.iter().map(|cnt| cnt * (cnt - 1) / 2).sum::<i64>();
    println!("{num_combs}");
}
