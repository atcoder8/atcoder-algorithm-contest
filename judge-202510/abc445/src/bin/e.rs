use itertools::{Itertools, izip};
use proconio::input;

use crate::{eratosthenes_sieve::EratosthenesSieve, modint2::Modint998244353};

type Mint = Modint998244353;

const MAX: usize = 10_usize.pow(7);

fn main() {
    input! {
        t: usize,
        aaa: [[usize]; t],
    }

    let sieve = EratosthenesSieve::new(MAX);

    let solve = |aa: &[usize]| {
        let factors_by_a = aa
            .iter()
            .map(|&a| sieve.prime_factorization(a))
            .collect_vec();

        let primes = factors_by_a
            .iter()
            .flat_map(|factors| factors.iter().map(|&(p, _)| p))
            .sorted_unstable()
            .dedup()
            .collect_vec();

        let get_prime_position = |prime: usize| primes.binary_search(&prime).unwrap();

        let mut top_exps_per_prime = vec![[0, 0]; primes.len()];
        for factors in &factors_by_a {
            for &(p, e) in factors {
                let pos = get_prime_position(p);
                push(&mut top_exps_per_prime[pos], e);
            }
        }

        let all_lcm = izip!(&primes, &top_exps_per_prime)
            .fold(Mint::new(1), |acc, (&prime, top_exps)| {
                acc * Mint::new(prime).pow(top_exps[0])
            });

        let sub_solve = |factors: &[(usize, usize)]| {
            let mut lcm = all_lcm;
            for &(p, e) in factors {
                let pos = get_prime_position(p);
                let [exp1, exp2] = top_exps_per_prime[pos];
                if exp1 == e && exp1 > exp2 {
                    lcm /= Mint::new(p).pow(exp1 - exp2);
                }
            }

            lcm
        };

        factors_by_a
            .iter()
            .map(|factors| sub_solve(factors))
            .collect_vec()
    };

    let output = aaa.iter().map(|aa| solve(aa).iter().join(" ")).join("\n");
    println!("{output}");
}

fn push(top_exps: &mut [usize; 2], exp: usize) {
    if exp <= top_exps[1] {
        return;
    }

    top_exps[1] = exp;
    if top_exps[0] < top_exps[1] {
        top_exps.swap(0, 1);
    }
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

pub mod eratosthenes_sieve {
    //! Implements the Sieve of Eratosthenes.
    //!
    //! Finds the smallest prime factor of each number placed on the sieve,
    //! so it can perform Prime Factorization as well as Primality Test.

    #[derive(Debug, Clone)]
    pub struct EratosthenesSieve {
        sieve: Vec<usize>,
    }

    impl EratosthenesSieve {
        /// Constructs a Sieve of Eratosthenes.
        ///
        /// # Arguments
        ///
        /// * `upper_limit` - The largest number placed on the sieve.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// ```
        pub fn new(upper_limit: usize) -> Self {
            let mut sieve: Vec<usize> = (0..=upper_limit).collect();

            for p in (2..).take_while(|&i| i * i <= upper_limit) {
                if sieve[p] != p {
                    continue;
                }

                for i in ((p * p)..=upper_limit).step_by(p) {
                    if sieve[i] == i {
                        sieve[i] = p;
                    }
                }
            }

            Self { sieve }
        }

        /// Returns the least prime factor of `n`.
        ///
        /// However, if `n` is `1`, then `1` is returned.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.get_least_prime_factor(1), 1);
        /// assert_eq!(sieve.get_least_prime_factor(2), 2);
        /// assert_eq!(sieve.get_least_prime_factor(6), 2);
        /// assert_eq!(sieve.get_least_prime_factor(11), 11);
        /// assert_eq!(sieve.get_least_prime_factor(27), 3);
        /// ```
        pub fn get_least_prime_factor(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            self.sieve[n]
        }

        /// Determines if `n` is prime.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert!(!sieve.is_prime(1));
        /// assert!(sieve.is_prime(2));
        /// assert!(!sieve.is_prime(6));
        /// assert!(sieve.is_prime(11));
        /// assert!(!sieve.is_prime(27));
        /// ```
        pub fn is_prime(&self, n: usize) -> bool {
            n >= 2 && self.sieve[n] == n
        }

        /// Performs prime factorization of `n`.
        ///
        /// The result of the prime factorization is returned as a
        /// list of prime factor and exponent pairs.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.prime_factorization(1), vec![]);
        /// assert_eq!(sieve.prime_factorization(12), vec![(2, 2), (3, 1)]);
        /// assert_eq!(sieve.prime_factorization(19), vec![(19, 1)]);
        /// assert_eq!(sieve.prime_factorization(27), vec![(3, 3)]);
        /// ```
        pub fn prime_factorization(&self, n: usize) -> Vec<(usize, usize)> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut factors: Vec<(usize, usize)> = vec![];

            while n != 1 {
                let p = self.sieve[n];

                if factors.is_empty() || factors.last().unwrap().0 != p {
                    factors.push((p, 1));
                } else {
                    factors.last_mut().unwrap().1 += 1;
                }

                n /= p;
            }

            factors
        }

        /// Creates a list of positive divisors of `n`.
        ///
        /// The positive divisors are listed in ascending order.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.create_divisor_list(1), vec![1]);
        /// assert_eq!(sieve.create_divisor_list(12), vec![1, 2, 3, 4, 6, 12]);
        /// assert_eq!(sieve.create_divisor_list(19), vec![1, 19]);
        /// assert_eq!(sieve.create_divisor_list(27), vec![1, 3, 9, 27]);
        /// ```
        pub fn create_divisor_list(&self, n: usize) -> Vec<usize> {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let prime_factors = self.prime_factorization(n);
            let divisor_num: usize = prime_factors.iter().map(|&(_, exp)| exp + 1).product();

            let mut divisors = vec![1];
            divisors.reserve(divisor_num - 1);

            for (p, e) in prime_factors {
                let mut add_divisors = vec![];
                add_divisors.reserve(divisors.len() * e);
                let mut mul = 1;

                for _ in 1..=e {
                    mul *= p;

                    for &d in divisors.iter() {
                        add_divisors.push(d * mul);
                    }
                }

                divisors.append(&mut add_divisors);
            }

            divisors.sort_unstable();

            divisors
        }

        /// Calculates the number of positive divisors of `n`.
        ///
        /// # Examples
        ///
        /// ```
        /// use atcoder8_library::eratosthenes_sieve::EratosthenesSieve;
        ///
        /// let sieve = EratosthenesSieve::new(27);
        /// assert_eq!(sieve.calc_divisor_num(1), 1);
        /// assert_eq!(sieve.calc_divisor_num(12), 6);
        /// assert_eq!(sieve.calc_divisor_num(19), 2);
        /// assert_eq!(sieve.calc_divisor_num(27), 4);
        /// ```
        pub fn calc_divisor_num(&self, n: usize) -> usize {
            assert_ne!(n, 0, "`n` must be at least 1.");

            let mut n = n;

            let mut divisor_num = 1;
            let mut cur_p = None;
            let mut cur_exp = 0;

            while n != 1 {
                let p = self.sieve[n];

                if Some(p) == cur_p {
                    cur_exp += 1;
                } else {
                    divisor_num *= cur_exp + 1;

                    cur_p = Some(p);
                    cur_exp = 1;
                }

                n /= p;
            }

            divisor_num *= cur_exp + 1;

            divisor_num
        }
    }
}
