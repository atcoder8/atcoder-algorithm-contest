use proconio::input;

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        aaa: [[u8; w]; h],
        bb: [u8; n],
    }

    let ans = aaa
        .iter()
        .map(|aa| aa.iter().filter(|&&a| bb.contains(&a)).count())
        .max()
        .unwrap();
    println!("{ans}");
}
