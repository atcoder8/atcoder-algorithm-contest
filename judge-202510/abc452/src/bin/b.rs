use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let get_color = |coord: (usize, usize)| {
        if coord.0 == 0 || coord.0 == h - 1 || coord.1 == 0 || coord.1 == w - 1 {
            '#'
        } else {
            '.'
        }
    };

    let output = (0..h)
        .map(|row| {
            (0..w)
                .map(move |col| get_color((row, col)))
                .collect::<String>()
        })
        .join("\n");
    println!("{output}");
}
