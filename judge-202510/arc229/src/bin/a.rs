use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut s = "AR".repeat(24) + "ARC" + &"RC".repeat(23);

    for _ in 0..600 - x {
        let pos = (0..s.len() - 2)
            .find(|&pos| &s[pos..pos + 3] == "ARC")
            .unwrap();
        s.replace_range(pos..pos + 3, "CRA");
    }

    println!("{s}");
}
