use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        aa1: [u8; 6],
        aa2: [u8; 6],
        aa3: [u8; 6],
    }

    let mut cnt = 0_usize;
    for (a1, a2, a3) in iproduct!(aa1, aa2, aa3) {
        let mut rolls = [a1, a2, a3];
        rolls.sort_unstable();
        cnt += (rolls == [4, 5, 6]) as usize;
    }
    let prob = cnt as f64 / 6_u32.pow(3) as f64;
    println!("{prob}");
}
