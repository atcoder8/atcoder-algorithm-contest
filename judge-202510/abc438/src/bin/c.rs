use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut stack: Vec<usize> = vec![];
    for &a in &aa {
        if stack.len() >= 3 && stack[stack.len() - 3..].iter().all(|&v| v == a) {
            stack.truncate(stack.len() - 3);
        } else {
            stack.push(a);
        }
    }

    println!("{}", stack.len());
}
