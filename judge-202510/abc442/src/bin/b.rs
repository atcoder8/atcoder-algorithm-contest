use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        q: usize,
        aa: [u8; q],
    }

    let mut volume = 0_u32;
    let mut playing = false;
    let output = aa
        .iter()
        .map(|&a| {
            match a {
                1 => volume += 1,
                2 => volume = volume.saturating_sub(1),
                3 => playing = !playing,
                _ => panic!(),
            };
            if volume >= 3 && playing { "Yes" } else { "No" }
        })
        .join("\n");
    println!("{output}");
}
