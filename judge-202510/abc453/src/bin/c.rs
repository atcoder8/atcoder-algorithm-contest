use proconio::input;

fn main() {
    input! {
        n: usize,
        ll: [i64; n],
    }

    let mut max_score = 0;
    // (移動回数,現在の位置,座標0の通過回数)
    let mut stack = vec![(0, 0, 0)];
    while let Some((num_moves, x, score)) = stack.pop() {
        if num_moves == n {
            max_score = max_score.max(score);
            continue;
        }

        let dist = ll[num_moves];
        stack.push((
            num_moves + 1,
            x + dist,
            score + (x < 0 && x + dist >= 0) as usize,
        ));
        stack.push((
            num_moves + 1,
            x - dist,
            score + (x >= 0 && x - dist < 0) as usize,
        ));
    }

    println!("{max_score}");
}
