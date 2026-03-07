use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut cc: [u32; m],
        ab: [(Usize1, u32); n],
    }

    let mut total = 0;
    for &(a, b) in &ab {
        let c = &mut cc[a];
        let amount = (*c).min(b);
        total += amount;
        *c -= amount;
    }
    println!("{total}");
}
