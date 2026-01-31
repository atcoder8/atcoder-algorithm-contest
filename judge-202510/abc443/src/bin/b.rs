use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut cnt = 0;
    let ans = (n..)
        .take_while(|&age| {
            cnt += age;
            cnt < k
        })
        .count();
    println!("{ans}");
}
