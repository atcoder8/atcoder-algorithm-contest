use argio::proconio::{input, marker::Usize1};
use proconio::fastout;

use crate::disjoint_intervals::DisjointIntervals;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        lr: [(Usize1, usize); q],
    }

    let mut num_black_squares = 0_usize;
    let mut di = DisjointIntervals::new();
    for &(l, r) in &lr {
        num_black_squares += di.insert_range(l..r);
        println!("{}", n - num_black_squares);
    }
}

pub mod disjoint_intervals {
    use std::{collections::BTreeMap, ops};

    /// Data structure that represent a set by the union of disjoint intervals.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct DisjointIntervals<T> {
        /// Map with the start of the interval as key and the end of the interval as value.
        /// Each interval represented as a right half-open interval.
        intervals: BTreeMap<T, T>,

        /// Number of elements in the set.
        len: usize,
    }

    impl<I> From<I> for DisjointIntervals<usize>
    where
        I: IntoIterator<Item = usize>,
    {
        fn from(iterable: I) -> Self {
            let mut set = DisjointIntervals::new();
            for value in iterable {
                set.insert(value);
            }

            set
        }
    }

    impl Default for DisjointIntervals<usize> {
        /// Creates a new empty set.
        fn default() -> Self {
            Self::new()
        }
    }

    impl DisjointIntervals<usize> {
        /// Creates a new empty set.
        pub fn new() -> Self {
            DisjointIntervals {
                intervals: BTreeMap::new(),
                len: 0,
            }
        }

        /// Returns the number of elements in the set.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the set is empty.
        pub fn is_empty(&self) -> bool {
            self.intervals.is_empty()
        }

        /// Finds the interval such that `value` belong to.
        /// If no such interval exists, returns `None`.
        pub fn find_interval(&self, value: usize) -> Option<ops::Range<usize>> {
            match self.intervals.range(..=value).next_back() {
                Some((&start, &end)) if value < end => Some(start..end),
                _ => None,
            }
        }

        /// Returns `true` if the set contains `value`.
        pub fn contains(&self, value: usize) -> bool {
            self.find_interval(value).is_some()
        }

        /// Returns the number of non-contiguous intervals.
        pub fn number_of_intervals(&self) -> usize {
            self.intervals.len()
        }

        /// Inserts the elements contained in the `range`.
        /// Returns the increasing number of elements in the set.
        pub fn insert_range(&mut self, range: ops::Range<usize>) -> usize {
            if range.is_empty() {
                return 0;
            }

            let before_len = self.len;

            // Find the upper interval of the `range` in the set after insertion.
            let insert_start = match self.find_interval(range.start.saturating_sub(1)) {
                Some(left_interval) => left_interval.start,
                None => range.start,
            };
            let insert_end = match self.find_interval(range.end) {
                Some(right_interval) => right_interval.end,
                None => range.end,
            };

            // Remove intervals covered by the upper interval.
            let lower_intervals = self
                .intervals
                .range(insert_start..)
                .take_while(|&(_, &end)| end <= insert_end)
                .map(|(&start, _)| start)
                .collect::<Vec<usize>>();
            for start in lower_intervals {
                let end = self.intervals.remove(&start).unwrap();
                self.len -= end - start;
            }

            // Insert the upper interval.
            self.intervals.insert(insert_start, insert_end);
            self.len += insert_end - insert_start;

            self.len - before_len
        }

        /// Inserts an element to the set.
        /// Returns `true` if the elements of the set have increased.
        pub fn insert(&mut self, value: usize) -> bool {
            self.insert_range(value..value + 1) == 1
        }

        /// Removes elements in `range` from the set.
        /// Returns the decreasing number of elements in the set.
        pub fn remove_range(&mut self, range: ops::Range<usize>) -> usize {
            if range.is_empty() {
                return 0;
            }

            let before_len = self.len;

            // Temporarily insert elements in the `range` to make an interval that completely contains the `range`.
            self.insert_range(range.clone());
            let upper_interval = self.find_interval(range.start).unwrap();

            // Remove elements in the interval that completely contains `range`.
            self.intervals.remove(&upper_interval.start);
            self.len -= upper_interval.len();

            // Insert the removed elements which are not included in `range`.
            self.insert_range(upper_interval.start..range.start);
            self.insert_range(range.end..upper_interval.end);

            before_len - self.len
        }

        /// Removes an element from the set.
        /// Returns `true` if the elements of the set have decreased.
        pub fn remove(&mut self, value: usize) -> bool {
            self.remove_range(value..value + 1) == 1
        }

        /// Returns the smallest non-negative integer not included in the set.
        pub fn mex(&self) -> usize {
            match self.intervals.first_key_value() {
                Some((&start, &end)) if start == 0 => end,
                _ => 0,
            }
        }

        /// Creates an iterator that traverses the elements contained in the set.
        pub fn range_inclusively(&self, range: ops::Range<usize>) -> RangeInclusively<usize> {
            RangeInclusively {
                intervals: self,
                range,
            }
        }

        /// Creates an iterator that traverses the elements not contained in the set.
        pub fn range_exclusively(&self, range: ops::Range<usize>) -> RangeExclusively<usize> {
            RangeExclusively {
                intervals: self,
                range,
            }
        }
    }

    /// Iterator that traverses elements in the range that are included in the set.
    #[derive(Debug)]
    pub struct RangeInclusively<'a, T> {
        intervals: &'a DisjointIntervals<T>,
        range: ops::Range<usize>,
    }

    impl<'a> Iterator for RangeInclusively<'a, usize> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if !self.intervals.contains(self.range.start) {
                match self.intervals.intervals.range(self.range.start..).next() {
                    Some((&start, _)) => self.range.start = start,
                    None => {
                        self.range.start = self.range.end;
                        return None;
                    }
                }
            }

            self.range.start += 1;

            Some(self.range.start - 1)
        }
    }

    impl<'a> DoubleEndedIterator for RangeInclusively<'a, usize> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if !self.intervals.contains(self.range.end - 1) {
                match self.intervals.intervals.range(..self.range.end).next_back() {
                    Some((_, &end)) => self.range.end = end,
                    None => {
                        self.range.end = self.range.start;
                        return None;
                    }
                }
            }

            self.range.end -= 1;

            Some(self.range.end)
        }
    }

    /// Iterator that traverses elements in the range that are **not** included in the set.
    #[derive(Debug)]
    pub struct RangeExclusively<'a, T> {
        intervals: &'a DisjointIntervals<T>,
        range: ops::Range<usize>,
    }

    impl<'a> Iterator for RangeExclusively<'a, usize> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if self.intervals.contains(self.range.start) {
                let (_, &end) = self
                    .intervals
                    .intervals
                    .range(..=self.range.start)
                    .next_back()
                    .unwrap();
                self.range.start = end.min(self.range.end);

                if self.range.is_empty() {
                    return None;
                }
            }

            self.range.start += 1;

            Some(self.range.start - 1)
        }
    }

    impl<'a> DoubleEndedIterator for RangeExclusively<'a, usize> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.range.is_empty() {
                return None;
            }

            if self.intervals.contains(self.range.end - 1) {
                let (&start, _) = self
                    .intervals
                    .intervals
                    .range(..self.range.end)
                    .next_back()
                    .unwrap();
                self.range.end = start.max(self.range.start);

                if self.range.is_empty() {
                    return None;
                }
            }

            self.range.end -= 1;

            Some(self.range.end)
        }
    }
}
