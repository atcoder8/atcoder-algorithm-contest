use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", n.div_ceil(2));
}
