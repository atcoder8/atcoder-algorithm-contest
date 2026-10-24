use itertools::Itertools;
use proconio::{input, marker::Usize1};

use crate::{min_heap::MinHeap, modint2::Modint998244353};

type Mint = Modint998244353;

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, u32); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &abc {
        graph[u].push((v, weight));
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct Node {
        rank: usize,
        number: u32,
        current: usize,
        score: Mint,
    }

    let mut min_scores = vec![None::<Mint>; n];
    let mut heap = MinHeap::from_value(Node::default(), |node| (node.rank, node.number));
    let mut rank = 0;
    let mut last_pair = (0, 0);
    while let Some(node) = heap.pop() {
        if min_scores[node.current].is_some() {
            continue;
        }

        min_scores[node.current] = Some(node.score);

        // 遷移元と辺の数字のペアが更新された場合にランクをインクリメントする
        if (node.rank, node.number) != last_pair {
            last_pair = (node.rank, node.number);
            rank += 1;
        }

        heap.extend(graph[node.current].iter().map(|&(next, number)| Node {
            rank,
            number,
            current: next,
            score: 10 * node.score + number,
        }));
    }

    let output = min_scores[1..]
        .iter()
        .map(|min_score| min_score.unwrap())
        .join("\n");
    println!("{output}");
}

pub mod modint2 {
    //! This module implements modular arithmetic.

    use std::{
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    type InnerType = u32;

    /// Returns `x` such that `a * x` is equivalent to `1` with `m` as the modulus.
    fn modinv(a: u32, m: u32) -> u32 {
        let (mut a, mut b, mut s, mut t) = (a as i64, m as i64, 1, 0);
        while b != 0 {
            let q = a / b;
            a -= q * b;
            std::mem::swap(&mut a, &mut b);
            s -= q * t;
            std::mem::swap(&mut s, &mut t);
        }

        assert_eq!(
            a.abs(),
            1,
            "\
There is no multiplicative inverse of `a` with `m` as the modulus, \
because `a` and `m` are not prime to each other (gcd(a, m) = {}).",
            a.abs()
        );

        ((s % m as i64 + m as i64) % m as i64) as u32
    }

    pub trait Reminder {
        /// Returns the remainder divided by `modulus`.
        fn reminder(self, modulus: InnerType) -> InnerType;
    }

    macro_rules! impl_reminder_for_small_unsigned_int {
        ($($unsigned_small_int: tt), *) => {
            $(
                impl Reminder for $unsigned_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        self as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `u8`, `u16` and `u32`.
    impl_reminder_for_small_unsigned_int!(u8, u16, u32);

    macro_rules! impl_reminder_for_large_unsigned_int {
        ($($unsigned_large_int: tt), *) => {
            $(
                impl Reminder for $unsigned_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self) as InnerType
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `usize`, `u64` and `u128`.
    impl_reminder_for_large_unsigned_int!(usize, u64, u128);

    macro_rules! impl_reminder_for_small_signed_int {
        ($($signed_small_int: tt), *) => {
            $(
                impl Reminder for $signed_small_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self as i32 % modulus as i32 + modulus as i32) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `i8`, `i16` and `i32`.
    impl_reminder_for_small_signed_int!(i8, i16, i32);

    macro_rules! impl_reminder_for_large_signed_int {
        ($($signed_large_int: tt), *) => {
            $(
                impl Reminder for $signed_large_int {
                    fn reminder(self, modulus: InnerType) -> InnerType {
                        (self % modulus as Self + modulus as Self) as InnerType % modulus
                    }
                }
            )*
        };
    }

    // Implements `Reminder` trait for `isize`, `i64` and `i128`.
    impl_reminder_for_large_signed_int!(isize, i64, i128);

    /// Structure for modular arithmetic.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modint<const MODULUS: InnerType> {
        rem: InnerType,
    }

    impl<const MODULUS: InnerType> std::fmt::Display for Modint<MODULUS> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.rem)
        }
    }

    impl<const MODULUS: InnerType> Default for Modint<MODULUS> {
        /// Returns a `Modint` instance equivalent to `0`.
        fn default() -> Self {
            Self::raw(0)
        }
    }

    impl<T, const MODULUS: InnerType> From<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn from(value: T) -> Self {
            Self::new(value)
        }
    }

    impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem + MODULUS - rhs.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
            Self::raw((self.rem as u64 * rhs.rem as u64 % MODULUS as u64) as InnerType)
        }
    }

    impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for Modint<MODULUS> {
        type Output = Self;

        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
            self * rhs.inv()
        }
    }

    impl<const MODULUS: InnerType> Neg for Modint<MODULUS> {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::raw((MODULUS - self.rem) % MODULUS)
        }
    }

    impl<const MODULUS: InnerType> AddAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn add_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self + rhs;
        }
    }

    impl<const MODULUS: InnerType> SubAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn sub_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self - rhs;
        }
    }

    impl<const MODULUS: InnerType> MulAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn mul_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self * rhs;
        }
    }

    impl<const MODULUS: InnerType> DivAssign<Modint<MODULUS>> for Modint<MODULUS> {
        fn div_assign(&mut self, rhs: Modint<MODULUS>) {
            *self = *self / rhs;
        }
    }

    impl<const MODULUS: InnerType, T> Add<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn add(self, rhs: T) -> Self::Output {
            self + Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Sub<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn sub(self, rhs: T) -> Self::Output {
            self - Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Mul<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn mul(self, rhs: T) -> Self::Output {
            self * Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> Div<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        type Output = Modint<MODULUS>;

        fn div(self, rhs: T) -> Self::Output {
            self / Self::new(rhs)
        }
    }

    impl<const MODULUS: InnerType, T> AddAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn add_assign(&mut self, rhs: T) {
            *self += Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> SubAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn sub_assign(&mut self, rhs: T) {
            *self -= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> MulAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn mul_assign(&mut self, rhs: T) {
            *self *= Modint::new(rhs);
        }
    }

    impl<const MODULUS: InnerType, T> DivAssign<T> for Modint<MODULUS>
    where
        T: Reminder,
    {
        fn div_assign(&mut self, rhs: T) {
            *self /= Modint::new(rhs);
        }
    }

    macro_rules! impl_ops_for_integer {
        ($($integer:ty), *) => {
            $(
                impl<const MODULUS: InnerType> Add<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn add(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) + rhs
                    }
                }

                impl<const MODULUS: InnerType> Sub<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn sub(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) - rhs
                    }
                }

                impl<const MODULUS: InnerType> Mul<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn mul(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) * rhs
                    }
                }

                impl<const MODULUS: InnerType> Div<Modint<MODULUS>> for $integer {
                    type Output = Modint<MODULUS>;

                    fn div(self, rhs: Modint<MODULUS>) -> Self::Output {
                        Modint::new(self) / rhs
                    }
                }
            )*
        };
    }

    impl_ops_for_integer!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
    );

    impl<const MODULUS: InnerType> Sum<Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }

    impl<'a, const MODULUS: InnerType> Sum<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn sum<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, &x| acc + x)
        }
    }

    impl<const MODULUS: InnerType> Product<Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }

    impl<'a, const MODULUS: InnerType> Product<&'a Modint<MODULUS>> for Modint<MODULUS> {
        fn product<I: Iterator<Item = &'a Modint<MODULUS>>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, &x| acc * x)
        }
    }

    impl<const MODULUS: InnerType> Modint<MODULUS> {
        /// Returns the modulus.
        pub fn modulus() -> InnerType {
            MODULUS
        }

        /// Returns a `Modint` instance equivalent to `a`.
        pub fn new<T>(a: T) -> Self
        where
            T: Reminder,
        {
            Self {
                rem: a.reminder(MODULUS),
            }
        }

        /// Creates a `Modint` instance from a non-negative integer less than `MODULUS`.
        pub fn raw(a: InnerType) -> Self {
            Self { rem: a }
        }

        /// Set the remainder of `Modint` instance to `a`.
        pub fn set_rem<T>(&mut self, a: T)
        where
            T: Reminder,
        {
            self.rem = a.reminder(MODULUS);
        }

        /// Returns `x` such that `x * q` is equivalent to `p`.
        pub fn frac<T>(p: T, q: T) -> Self
        where
            T: Reminder,
        {
            Self::new(p) / Self::new(q)
        }

        /// Returns the remainder divided by `MODULUS`.
        /// The returned value is a non-negative integer less than `MODULUS`.
        pub fn rem(self) -> InnerType {
            self.rem
        }

        /// Returns the modular multiplicative inverse.
        pub fn inv(self) -> Self {
            Self {
                rem: modinv(self.rem, MODULUS),
            }
        }

        /// Calculates the power of `exp` using the iterative squaring method.
        pub fn pow<T>(self, exp: T) -> Self
        where
            T: ToExponent,
        {
            let mut ret = Self::new(1);
            let mut mul = self;
            let exp = exp.to_exponent();
            let mut t = exp.abs;

            while t != 0 {
                if t & 1 == 1 {
                    ret *= mul;
                }

                mul *= mul;
                t >>= 1;
            }

            if exp.neg {
                ret = ret.inv();
            }

            ret
        }
    }

    pub struct Exponent {
        neg: bool,
        abs: u128,
    }

    pub trait ToExponent {
        fn to_exponent(self) -> Exponent;
    }

    macro_rules! impl_to_exponent_for_unsigned_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: false,
                            abs: self as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_unsigned_int!(usize, u8, u16, u32, u64, u128);

    macro_rules! impl_to_exponent_for_signed_int {
        ($($ty: tt), *) => {
            $(
                impl ToExponent for $ty {
                    fn to_exponent(self) -> Exponent {
                        Exponent {
                            neg: self.is_negative(),
                            abs: self.abs() as u128,
                        }
                    }
                }
            )*
        };
    }

    impl_to_exponent_for_signed_int!(isize, i8, i16, i32, i64, i128);

    #[derive(Debug, Clone)]
    pub struct Factorial<Modint> {
        /// Upper limit of available factorial argument.
        upper_limit: usize,

        /// List of factorials.
        fac: Vec<Modint>,

        /// List of factorial inverses.
        inv_fac: Vec<Modint>,
    }

    impl<const MODULUS: InnerType> Factorial<Modint<MODULUS>> {
        /// Calculates factorial and its inverse for non-negative integers bellow `upper_limit`.
        pub fn new(upper_limit: usize) -> Self {
            let mut fac = vec![Modint::new(1); upper_limit + 1];
            for i in 0..upper_limit {
                fac[i + 1] = fac[i] * (i + 1);
            }

            let mut inv_fac = vec![fac[upper_limit].inv(); upper_limit + 1];
            for i in (0..upper_limit).rev() {
                inv_fac[i] = inv_fac[i + 1] * (i + 1);
            }

            Self {
                upper_limit,
                fac,
                inv_fac,
            }
        }

        /// Returns the factorial `n`.
        pub fn factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.fac[n]
        }

        /// Returns the inverse of the factorial of `n`.
        pub fn inverse_factorial(&self, n: usize) -> Modint<MODULUS> {
            assert!(
                n <= self.upper_limit,
                "The maximum number of available factorial arguments has been exceeded."
            );

            self.inv_fac[n]
        }

        /// Calculates the number of ways to select and arrange `k` objects from `n` unique objects.
        pub fn permutations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects.
        pub fn combinations(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n >= k {
                self.factorial(n) * self.inverse_factorial(n - k) * self.inverse_factorial(k)
            } else {
                Modint::new(0)
            }
        }

        /// Calculates the number of ways to select `k` objects from `n` unique objects, allowing for duplicates.
        pub fn combinations_with_repetition(&self, n: usize, k: usize) -> Modint<MODULUS> {
            if n == 0 {
                return if k == 0 {
                    Modint::new(1)
                } else {
                    Modint::new(0)
                };
            }

            self.combinations(n + k - 1, k)
        }
    }

    /// The type `Modint` with 1000000007 as the modulus.
    pub type Modint1000000007 = Modint<1000000007>;

    /// The type `Modint` with 998244353 as the modulus.
    pub type Modint998244353 = Modint<998244353>;
}

pub mod min_heap {
    use std::{cmp::Reverse, collections::BinaryHeap, iter::FusedIterator, marker::PhantomData};

    /// キー抽出関数を用いて作成されたヒープノード
    #[derive(Debug, Clone)]
    struct HeapNode<Value, SortKey: Ord> {
        value: Value,
        sort_key: SortKey,
    }

    impl<Value, SortKey: Ord> PartialEq for HeapNode<Value, SortKey> {
        fn eq(&self, other: &Self) -> bool {
            self.sort_key == other.sort_key
        }
    }

    impl<Value, SortKey: Ord> Eq for HeapNode<Value, SortKey> where SortKey: Ord {}

    impl<Value, SortKey: Ord> PartialOrd for HeapNode<Value, SortKey> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<Value, SortKey: Ord> Ord for HeapNode<Value, SortKey> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.sort_key.cmp(&other.sort_key)
        }
    }

    /// キー抽出関数を用いてヒープノードを作成するBuilderです。
    #[derive(Debug, Clone)]
    struct NodeBuilder<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> {
        value: PhantomData<Value>,
        sort_key: PhantomData<SortKey>,
        extract_key: ExtractKey,
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey>
        NodeBuilder<Value, SortKey, ExtractKey>
    {
        fn new(extract_key: ExtractKey) -> Self {
            Self {
                value: PhantomData,
                sort_key: PhantomData,
                extract_key,
            }
        }

        fn build(&self, value: Value) -> HeapNode<Value, SortKey> {
            let sort_key = (self.extract_key)(&value);
            HeapNode { value, sort_key }
        }
    }

    /// キー抽出関数を保持する最小ヒープです。
    #[derive(Debug, Clone)]
    pub struct MinHeap<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> {
        /// キー抽出関数を用いて順序付けされたヒープ
        heap: BinaryHeap<Reverse<HeapNode<Value, SortKey>>>,

        /// キー抽出関数を用いてヒープノードを作成するBuilder
        node_builder: NodeBuilder<Value, SortKey, ExtractKey>,
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> Extend<Value>
        for MinHeap<Value, SortKey, ExtractKey>
    {
        fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
            let nodes = iter.into_iter().map(|value| self.node_builder.build(value));
            self.heap.extend(nodes.map(Reverse));
        }
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> MinHeap<Value, SortKey, ExtractKey> {
        /// 空のヒープを作成します。ヒープ内部の順序付けはキー抽出関数を用いて行われます。
        pub fn new(extract_key: ExtractKey) -> Self {
            Self {
                heap: BinaryHeap::new(),
                node_builder: NodeBuilder::new(extract_key),
            }
        }

        /// 単一の値を持つヒープを生成します。
        pub fn from_value(value: Value, extract_key: ExtractKey) -> Self {
            let mut min_heap = Self::new(extract_key);
            min_heap.push(value);

            min_heap
        }

        /// 値のコレクションをヒープに変換します。
        pub fn from_iter<I>(values: I, extract_key: ExtractKey) -> Self
        where
            I: IntoIterator<Item = Value>,
        {
            let mut min_heap = Self::new(extract_key);
            min_heap.extend(values);

            min_heap
        }

        /// ヒープの要素数を返します。
        pub fn len(&self) -> usize {
            self.heap.len()
        }

        /// ヒープが空であるかどうかを表すブール値を返します。
        pub fn is_empty(&self) -> bool {
            self.heap.is_empty()
        }

        /// ヒープの全ての要素を訪れるイテレータです。
        /// 要素の訪問順は不定です。
        pub fn iter(&self) -> impl Iterator<Item = &Value> {
            self.heap.iter().map(|Reverse(node)| &node.value)
        }

        /// ヒープ内の値のうち、キー抽出関数を最小化するものを返します。
        /// ヒープが空である場合は`None`を返します。
        pub fn peek(&self) -> Option<&Value> {
            Some(&self.heap.peek()?.0.value)
        }

        /// ヒープに値を追加します。
        pub fn push(&mut self, value: Value) {
            self.heap.push(Reverse(self.node_builder.build(value)));
        }

        /// ヒープ内の値のうち、キー抽出関数を最小化するものを削除して返します。
        /// ヒープが空である場合は`None`を返します。
        pub fn pop(&mut self) -> Option<Value> {
            Some(self.heap.pop()?.0.value)
        }

        /// ヒープ内の全ての要素を作成します。
        pub fn clear(&mut self) {
            self.heap.clear();
        }

        /// ヒープに格納された値のうち、`predicate`により`true`と判定されるもののみを残します。
        pub fn retain<Predicate>(&mut self, mut predicate: Predicate)
        where
            Predicate: FnMut(&Value) -> bool,
        {
            self.heap.retain(|Reverse(node)| predicate(&node.value));
        }

        /// 追加の要素のための領域を確保します。
        /// `reserve`を呼び出した後、ヒープの容量は`self.len() + additional`以上になります。
        /// 頻繁なメモリの再確保を避けるために領域を余分に確保する可能性があります。
        pub fn reserve(&mut self, additional: usize) {
            self.heap.reserve(additional);
        }

        /// 追加の要素のための領域を確保します。
        /// `reserve_exact`を呼び出した後、ヒープの容量は`self.len() + additional`以上になります。
        /// `reserve`と異なり、頻繁なメモリの再確保を避けるための余分な領域の確保を行いません。
        pub fn reserve_exact(&mut self, additional: usize) {
            self.heap.reserve_exact(additional);
        }

        /// ヒープの余分な容量を破棄します。
        pub fn shrink_to_fit(&mut self) {
            self.heap.shrink_to_fit();
        }

        /// `min_capacity`を下限としてヒープの余分な容量を破棄します。
        pub fn shrink_to(&mut self, min_capacity: usize) {
            self.heap.shrink_to(min_capacity);
        }
    }

    /// ヒープの要素をキー抽出関数の戻り値に関して昇順に返すイテレータです。
    #[derive(Debug, Clone)]
    pub struct MinHeapIter<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> {
        min_heap: MinHeap<Value, SortKey, ExtractKey>,
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> ExactSizeIterator
        for MinHeapIter<Value, SortKey, ExtractKey>
    {
        fn len(&self) -> usize {
            self.min_heap.len()
        }
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> FusedIterator
        for MinHeapIter<Value, SortKey, ExtractKey>
    {
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> Iterator
        for MinHeapIter<Value, SortKey, ExtractKey>
    {
        type Item = Value;

        fn next(&mut self) -> Option<Self::Item> {
            self.min_heap.pop()
        }
    }

    impl<Value, SortKey: Ord, ExtractKey: Fn(&Value) -> SortKey> IntoIterator
        for MinHeap<Value, SortKey, ExtractKey>
    {
        type Item = Value;

        type IntoIter = MinHeapIter<Value, SortKey, ExtractKey>;

        /// ヒープを消費してイテレータを生成します。
        /// 生成されたイテレータはヒープ内の全ての値をキー抽出関数の戻り値に関して昇順に返します。
        fn into_iter(self) -> Self::IntoIter {
            MinHeapIter { min_heap: self }
        }
    }
}
