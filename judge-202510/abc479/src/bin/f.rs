// unfinished

use itertools::Itertools;
use proconio::input;

use crate::{fenwick_tree::FenwickTree, unique_ordering::UniqueOrdering};

fn main() {
    input! {
        n: usize,
        ss: [u64; n],
        tt: [u64; n],
    }

    let base_cost = ss
        .iter()
        .zip(&tt)
        .map(|(&s, &t)| s.abs_diff(t))
        .sum::<u64>();

    let mut ur = UniqueOrdering::from(ss.iter().chain(&tt).copied().collect_vec());

    let ss = ss.into_iter().map(|s| ur.position(&s)).collect_vec();
    let tt = tt.into_iter().map(|t| ur.position(&t)).collect_vec();

    let mut covered_count = 0;
    let mut not_covered_count = 0;
    let order_by_s = (0..n).sorted_unstable_by_key(|&i| ss[i]).collect_vec();
    {
        let mut ft = FenwickTree::<usize>::new(ur.unique_len());
        for &i in &order_by_s {
            if ss[i] < tt[i] {
                covered_count += ft.sum(tt[i] + 1..);
                not_covered_count += ft.sum(..tt[i]);
                ft.add(tt[i], 1);
            }
        }
    }
    {
        let mut ft = FenwickTree::<usize>::new(ur.unique_len());
        for &i in order_by_s.iter().rev() {
            if ss[i] > tt[i] {
                covered_count += ft.sum(..tt[i]);
                not_covered_count += ft.sum(tt[i] + 1..);
                ft.add(tt[i], 1);
            }
        }
    }

    let mut num_crosses = 0;
    let order_by_t = (0..n).sorted_unstable_by_key(|&i| tt[i]).collect_vec();
    let mut ft = FenwickTree::<usize>::new(ur.unique_len());
    for &i in &order_by_t {
        num_crosses += ft.sum(ss[i] + 1..);
        ft.add(ss[i], 1);
    }

    let ans = base_cost + covered_count as u64
        - (num_crosses - (covered_count + not_covered_count)) as u64;
    println!("{ans}");
}

pub mod unique_ordering {
    //! Module for ordering unique elements.

    use std::ops::Index;

    /// Structure for ordering unique elements.
    #[derive(Debug, Clone)]
    pub struct UniqueOrdering<T> {
        /// A sequence containing the elements to be ordered.
        seq: Vec<T>,

        /// A flag indicating whether `seq` is sorted and deduplicated.
        organized: bool,
    }

    impl<T> Default for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> From<Vec<T>> for UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure by initializing the elements to be ordered with `seq`.
        fn from(seq: Vec<T>) -> Self {
            Self {
                seq,
                organized: false,
            }
        }
    }

    impl<T> Index<usize> for UniqueOrdering<T> {
        type Output = T;

        /// Returns the `index`-th (0-based) unique element.
        fn index(&self, index: usize) -> &Self::Output {
            &self.seq[index]
        }
    }

    impl<T> UniqueOrdering<T>
    where
        T: Clone + Ord,
    {
        /// Creates a structure for ordering unique elements.
        pub fn new() -> Self {
            Self {
                seq: vec![],
                organized: true,
            }
        }

        /// Adds the elements to be ordered.
        pub fn push(&mut self, x: T) {
            self.seq.push(x);
            self.organized = false;
        }

        /// Appends all elements of the iterator to the elements to be ordered.
        pub fn extend<I>(&mut self, other: I)
        where
            I: IntoIterator<Item = T>,
        {
            self.seq.extend(other);
            self.organized = false;
        }

        /// Sorts the sequence of stored elements in ascending order and removes all duplicates.
        fn organize(&mut self) {
            if !self.organized {
                self.seq.sort_unstable();
                self.seq.dedup();
                self.organized = true;
            }
        }

        /// Returns the `x` position of the unique elements sorted in ascending order.
        pub fn position(&mut self, x: &T) -> usize {
            self.organize();

            self.seq.binary_search(x).unwrap_or_else(|_| {
                panic!("The position of `x` is undefined.");
            })
        }

        /// Returns the `index`-th (0-based) unique element.
        ///
        /// Returns `None` if the `index`-th element does not exist.
        pub fn get(&mut self, index: usize) -> Option<&T> {
            self.seq.get(index)
        }

        /// Returns the number of unique elements.
        pub fn unique_len(&mut self) -> usize {
            self.organize();

            self.seq.len()
        }
    }
}

pub mod fenwick_tree {
    //! Processes the following query in `O(log(n))` time
    //! for a sequence of numbers with `n` elements:
    //! * Update one element
    //! * Calculate the sum of the elements of a range
    //! * Gets the elements of a number sequence.

    use std::ops::{AddAssign, RangeBounds, Sub, SubAssign};

    /// # Examples
    ///
    /// ```
    /// use atcoder8_library::fenwick_tree::FenwickTree;
    ///
    /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
    /// assert_eq!(ft.sum(2..), 11);
    /// ```
    #[derive(Debug, Clone)]
    pub struct FenwickTree<T>(Vec<T>);

    impl<T> From<Vec<T>> for FenwickTree<T>
    where
        T: Default + Clone + AddAssign<T>,
    {
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        /// ```
        fn from(t: Vec<T>) -> Self {
            let mut ft = FenwickTree::new(t.len());
            for (i, x) in t.into_iter().enumerate() {
                ft.add(i, x);
            }

            ft
        }
    }

    impl<T> FenwickTree<T> {
        /// Constructs a `FenwickTree<T>` with `n` elements.
        ///
        /// Each element is initialized with `T::default()`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::<i32>::new(5);
        /// assert_eq!(ft.sum(..), 0);
        /// ```
        pub fn new(n: usize) -> Self
        where
            T: Default + Clone,
        {
            FenwickTree(vec![T::default(); n])
        }

        /// Add `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.add(3, 100);
        /// assert_eq!(ft.sum(2..6), 109);
        /// ```
        pub fn add(&mut self, p: usize, x: T)
        where
            T: Clone + AddAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] += x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Subtract `x` from the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::<u32>::from(vec![3, 1, 4, 1, 5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 19);
        ///
        /// ft.sub(3, 1);
        /// assert_eq!(ft.sum(2..6), 18);
        /// ```
        pub fn sub(&mut self, p: usize, x: T)
        where
            T: Clone + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            let mut p = p + 1;
            while p <= n {
                data[p - 1] -= x.clone();
                p += p & p.overflowing_neg().0;
            }
        }

        /// Sets `x` to the `p`-th element.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let mut ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(2..6), 9);
        ///
        /// ft.set(3, 100);
        /// assert_eq!(ft.sum(2..6), 108);
        /// ```
        pub fn set(&mut self, p: usize, x: T)
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T> + SubAssign<T>,
        {
            let FenwickTree(data) = self;
            let n = data.len();

            assert!(p < n);

            self.sub(p, self.get(p));
            self.add(p, x);
        }

        /// Compute the sum of the range [0, r).
        fn inner_sum(&self, r: usize) -> T
        where
            T: Default + Clone + AddAssign<T>,
        {
            let mut s = T::default();
            let mut r = r;
            while r > 0 {
                s += self.0[r - 1].clone();
                r -= r & r.wrapping_neg();
            }

            s
        }

        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, 1, -5, 9, 2]);
        /// assert_eq!(ft.sum(..), 13);
        /// assert_eq!(ft.sum(2..), 11);
        /// assert_eq!(ft.sum(..6), 11);
        /// assert_eq!(ft.sum(2..6), 9);
        /// assert_eq!(ft.sum(6..2), 0);
        /// ```
        pub fn sum<R>(&self, rng: R) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
            R: RangeBounds<usize>,
        {
            let n = self.0.len();

            let l = match rng.start_bound() {
                std::ops::Bound::Included(&start_bound) => start_bound,
                std::ops::Bound::Excluded(&start_bound) => start_bound + 1,
                std::ops::Bound::Unbounded => 0,
            };

            let r = match rng.end_bound() {
                std::ops::Bound::Included(&end_bound) => end_bound + 1,
                std::ops::Bound::Excluded(&end_bound) => end_bound,
                std::ops::Bound::Unbounded => n,
            };

            assert!(l <= n && r <= n);

            if l >= r {
                T::default()
            } else {
                self.inner_sum(r) - self.inner_sum(l)
            }
        }

        /// Returns the value of an element in a sequence of numbers.
        /// Calculate the total of the range.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::fenwick_tree::FenwickTree;
        ///
        /// let ft = FenwickTree::from(vec![3, -1, 4, -1, 5]);
        /// assert_eq!(ft.get(2), 4);
        /// ```
        pub fn get(&self, p: usize) -> T
        where
            T: Default + Clone + AddAssign<T> + Sub<T, Output = T>,
        {
            assert!(p < self.0.len());

            self.sum(p..=p)
        }
    }
}
