use proconio::input;

fn main() {
    input! {
        (n, w): (usize, u64),
        ab: [(u64, u64); n],
    }

    let mut max_score = 0;
    for bits in 0..1 << n {
        let mut score = 0;
        let mut cost = 0;

        for i in 0..n {
            if bits >> i & 1 == 0 {
                continue;
            }

            let (a, b) = ab[i];
            score += a;
            cost += b;
        }

        if cost <= w {
            max_score = max_score.max(score);
        }
    }

    println!("{max_score}");
}
