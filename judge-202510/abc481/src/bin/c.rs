use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        q: usize,
        ss: [String; q],
    }

    let mut map = HashMap::<String, usize>::new();
    for s in ss {
        let cand_id = map.len();
        let id = *map.entry(s).or_insert(cand_id);
        println!("{}", id + 1);
    }
}
