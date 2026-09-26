use proconio::input;

fn main() {
    input! {
        c: char,
    }

    let ans = match c {
        'B' => 'Y',
        'Y' => 'R',
        'R' => 'B',
        _ => panic!(),
    };
    println!("{ans}");
}
