use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let count = s.chars().filter(|&ch| ch == 'i' || ch == 'j').count();
    println!("{count}");
}
