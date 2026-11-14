use proconio::input;

fn main() {
    input! {
        k: usize,
        aa: [u8; k],
    }

    if aa.contains(&9) {
        let min = aa.iter().min().unwrap();
        println!("{min}");
    } else {
        println!("-1");
    }
}
