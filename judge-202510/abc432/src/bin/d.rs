use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, i64, i64),
        cab: [(char, i64, i64); n],
    }

    let mut rectangles = vec![Rectangle::new(0, x, 0, y)];
    for &(c, a, b) in &cab {
        rectangles = rectangles
            .into_iter()
            .flat_map(|rect| {
                if c == 'X' {
                    rect.horizontal_shifted(a, b)
                } else {
                    rect.vertical_shifted(a, b)
                }
            })
            .collect();
    }

    let mut visited = vec![false; rectangles.len()];
    let mut counts = vec![];
    for start in 0..rectangles.len() {
        if visited[start] {
            continue;
        }

        let mut sum_area = 0;
        let mut stack = vec![start];
        while let Some(curr) = stack.pop() {
            if visited[curr] {
                continue;
            }

            visited[curr] = true;
            sum_area += rectangles[curr].area();

            stack.extend(
                (0..rectangles.len()).filter(|&idx| rectangles[curr].adjacent(&rectangles[idx])),
            );
        }

        counts.push(sum_area);
    }
    counts.sort_unstable();

    println!("{}\n{}", counts.len(), counts.iter().join(" "));
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    top: i64,
    bottom: i64,
    left: i64,
    right: i64,
}

impl Rectangle {
    fn new(top: i64, bottom: i64, left: i64, right: i64) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn area(&self) -> i64 {
        (self.bottom - self.top) * (self.right - self.left)
    }

    fn adjacent(&self, other: &Self) -> bool {
        ((self.top == other.bottom || other.top == self.bottom)
            && self.left < other.right
            && other.left < self.right)
            || ((self.left == other.right || other.left == self.right)
                && self.top < other.bottom
                && other.top < self.bottom)
    }

    fn horizontal_shifted(&self, border: i64, shift: i64) -> Vec<Rectangle> {
        let mut shifted_rectangles = vec![];

        if self.top < border {
            shifted_rectangles.push(Rectangle::new(
                self.top,
                self.bottom.min(border),
                self.left - shift,
                self.right - shift,
            ))
        }

        if self.bottom > border {
            shifted_rectangles.push(Rectangle::new(
                self.top.max(border),
                self.bottom,
                self.left + shift,
                self.right + shift,
            ))
        }

        shifted_rectangles
    }

    fn vertical_shifted(&self, border: i64, shift: i64) -> Vec<Rectangle> {
        let mut shifted_rectangles = vec![];

        if self.left < border {
            shifted_rectangles.push(Rectangle::new(
                self.top - shift,
                self.bottom - shift,
                self.left,
                self.right.min(border),
            ));
        }

        if self.right > border {
            shifted_rectangles.push(Rectangle::new(
                self.top + shift,
                self.bottom + shift,
                self.left.max(border),
                self.right,
            ));
        }

        shifted_rectangles
    }
}
