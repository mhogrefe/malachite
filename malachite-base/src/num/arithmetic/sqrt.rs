use num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, ShrRound, SqrtRem,
    SqrtRemAssign,
};
use num::basic::integers::PrimitiveInt;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::SignificantBits;
use rounding_modes::RoundingMode;
use std::cmp::Ordering;

const U8_SQUARES: [u8; 16] = [
    0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225,
];

impl FloorSqrt for u8 {
    type Output = u8;

    /// Returns the floor of the square root of an integer.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// The `u8` implementation uses a lookup table.
    fn floor_sqrt(self) -> u8 {
        u8::wrapping_from(match U8_SQUARES.binary_search(&self) {
            Ok(i) => i,
            Err(i) => i - 1,
        })
    }
}

impl CeilingSqrt for u8 {
    type Output = u8;

    /// Returns the ceiling of the square root of an integer.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// The `u8` implementation uses a lookup table.
    fn ceiling_sqrt(self) -> u8 {
        u8::wrapping_from(match U8_SQUARES.binary_search(&self) {
            Ok(i) | Err(i) => i,
        })
    }
}

impl CheckedSqrt for u8 {
    type Output = u8;

    /// Returns the the square root of an integer, or `None` if the integer is not a perfect
    /// square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// The `u8` implementation uses a lookup table.
    fn checked_sqrt(self) -> Option<u8> {
        U8_SQUARES.binary_search(&self).ok().map(u8::wrapping_from)
    }
}

impl SqrtRem for u8 {
    type SqrtOutput = u8;
    type RemOutput = u8;

    /// Returns the floor of the square root of an integer, and the remainder (the difference
    /// between the integer and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// The `u8` implementation uses a lookup table.
    fn sqrt_rem(self) -> (u8, u8) {
        match U8_SQUARES.binary_search(&self) {
            Ok(i) => (u8::wrapping_from(i), 0),
            Err(i) => (u8::wrapping_from(i - 1), self - U8_SQUARES[i - 1]),
        }
    }
}

fn floor_inverse_checked_binary<T: PrimitiveUnsigned, F: Fn(T) -> Option<T>>(
    f: F,
    x: T,
    mut low: T,
    mut high: T,
) -> T {
    loop {
        if high <= low {
            return low;
        }
        let mid: T = low
            .checked_add(high)
            .unwrap()
            .shr_round(1, RoundingMode::Ceiling);
        match f(mid).map(|mid| mid.cmp(&x)) {
            Some(Ordering::Equal) => return mid,
            Some(Ordering::Less) => low = mid,
            Some(Ordering::Greater) | None => high = mid - T::ONE,
        }
    }
}

#[doc(hidden)]
pub fn _floor_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> T {
    if x < T::TWO {
        x
    } else {
        let p = T::power_of_2(x.significant_bits().shr_round(1, RoundingMode::Ceiling));
        floor_inverse_checked_binary(T::checked_square, x, p >> 1, p)
    }
}

#[doc(hidden)]
pub fn _ceiling_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> T {
    let floor_sqrt = _floor_sqrt_binary(x);
    if floor_sqrt.square() == x {
        floor_sqrt
    } else {
        floor_sqrt + T::ONE
    }
}

#[doc(hidden)]
pub fn _checked_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> Option<T> {
    let floor_sqrt = _floor_sqrt_binary(x);
    if floor_sqrt.square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _sqrt_rem_binary<T: PrimitiveUnsigned>(x: T) -> (T, T) {
    let floor_sqrt = _floor_sqrt_binary(x);
    (floor_sqrt, x - floor_sqrt.square())
}

/// TODO clean up float conversion
/// This is n_sqrt from ulong_extras/sqrt.c, FLINT 2.7.1.
fn _floor_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> T {
    if x >= max_square {
        return T::low_mask(T::WIDTH >> 1);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Ordering::Equal => sqrt,
        Ordering::Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Ordering::Equal => return sqrt,
                Ordering::Less => {}
                Ordering::Greater => return sqrt - T::ONE,
            }
        },
        Ordering::Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            if square <= x {
                return sqrt;
            }
        },
    }
}

/// TODO clean up float conversion
fn _ceiling_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> T {
    if x > max_square {
        return T::power_of_2(T::WIDTH >> 1);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Ordering::Equal => sqrt,
        Ordering::Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            if square >= x {
                return sqrt;
            }
        },
        Ordering::Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            match square.cmp(&x) {
                Ordering::Equal => return sqrt,
                Ordering::Greater => {}
                Ordering::Less => return sqrt + T::ONE,
            }
        },
    }
}

/// TODO clean up float conversion
fn _checked_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> Option<T> {
    if x > max_square {
        return None;
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Ordering::Equal => Some(sqrt),
        Ordering::Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Ordering::Equal => return Some(sqrt),
                Ordering::Less => {}
                Ordering::Greater => return None,
            }
        },
        Ordering::Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            match square.cmp(&x) {
                Ordering::Equal => return Some(sqrt),
                Ordering::Less => return None,
                Ordering::Greater => {}
            }
        },
    }
}

/// TODO clean up float conversion
/// This is n_sqrtrem from ulong_extras/sqrtrem.c, FLINT 2.7.1.
fn _sqrt_rem_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> (T, T) {
    if x >= max_square {
        return (T::low_mask(T::WIDTH >> 1), x - max_square);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Ordering::Equal => (sqrt, T::ZERO),
        Ordering::Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Ordering::Equal => return (sqrt, T::ZERO),
                Ordering::Less => {}
                Ordering::Greater => {
                    square -= (sqrt << 1) - T::ONE;
                    sqrt -= T::ONE;
                    return (sqrt, x - square);
                }
            }
        },
        Ordering::Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            if square <= x {
                return (sqrt, x - square);
            }
        },
    }
}

macro_rules! impl_sqrt_approx_and_refine {
    ($t: ident, $max_square: expr) => {
        impl FloorSqrt for $t {
            type Output = $t;

            /// Returns the floor of the square root of an integer.
            ///
            /// $f(x) = \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            ///
            /// # Notes
            /// For `u16` through `u64`, the square root is approximated using floating-point
            /// computation and then adjusting the result. In practice, it seems that only one
            /// adjustment is needed to get an exact answer.
            #[inline]
            fn floor_sqrt(self) -> $t {
                _floor_sqrt_approx_and_refine(|x| x as f64, |x| x as $t, $max_square, self)
            }
        }

        impl CeilingSqrt for $t {
            type Output = $t;

            /// Returns the ceiling of the square root of an integer.
            ///
            /// $f(x) = \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            ///
            /// # Notes
            /// For `u16` through `u64`, the square root is approximated using floating-point
            /// computation and then adjusting the result. In practice, it seems that only one
            /// adjustment is needed to get an exact answer.
            #[inline]
            fn ceiling_sqrt(self) -> $t {
                _ceiling_sqrt_approx_and_refine(|x| x as f64, |x| x as $t, $max_square, self)
            }
        }

        impl CheckedSqrt for $t {
            type Output = $t;

            /// Returns the the square root of an integer, or `None` if the integer is not a
            /// perfect square.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            ///
            /// # Notes
            /// For `u16` through `u64`, the square root is approximated using floating-point
            /// computation and then adjusting the result. In practice, it seems that only one
            /// adjustment is needed to get an exact answer.
            #[inline]
            fn checked_sqrt(self) -> Option<$t> {
                _checked_sqrt_approx_and_refine(|x| x as f64, |x| x as $t, $max_square, self)
            }
        }

        impl SqrtRem for $t {
            type SqrtOutput = $t;
            type RemOutput = $t;

            /// Returns the floor of the square root of an integer, and the remainder (the
            /// difference between the integer and the square of the floor).
            ///
            /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            ///
            /// # Notes
            /// For `u16` through `u64`, the square root is approximated using floating-point
            /// computation and then adjusting the result. In practice, it seems that only one
            /// adjustment is needed to get an exact answer.
            #[inline]
            fn sqrt_rem(self) -> ($t, $t) {
                _sqrt_rem_approx_and_refine(|x| x as f64, |x| x as $t, $max_square, self)
            }
        }
    };
}
impl_sqrt_approx_and_refine!(u16, 0xfe01);
impl_sqrt_approx_and_refine!(u32, 0xfffe0001);
impl_sqrt_approx_and_refine!(u64, 0xfffffffe00000001);
const USIZE_MAX_SQUARE: usize = ((1 << (usize::WIDTH >> 1)) - 1) * ((1 << (usize::WIDTH >> 1)) - 1);
impl_sqrt_approx_and_refine!(usize, USIZE_MAX_SQUARE);

//TODO tune
const U128_SQRT_THRESHOLD: u64 = 125;
const U128_MAX_SQUARE: u128 = 0xfffffffffffffffe0000000000000001;

impl FloorSqrt for u128 {
    type Output = u128;

    /// Returns the floor of the square root of an integer.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    /// Constant-time addition, squaring, bit-shifting, and comparison are assumed.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// For `u128`, using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large `u128`s. To overcome this, large
    /// `u128`s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that
    /// $2^{14} \leq \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn floor_sqrt(self) -> u128 {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            _floor_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            _floor_sqrt_binary(self)
        }
    }
}

impl CeilingSqrt for u128 {
    type Output = u128;

    /// Returns the ceiling of the square root of an integer.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    /// Constant-time addition, squaring, bit-shifting, and comparison is assumed.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// For `u128`, using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large `u128`s. To overcome this, large
    /// `u128`s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that
    /// $2^{14} \leq \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn ceiling_sqrt(self) -> u128 {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            _ceiling_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            _ceiling_sqrt_binary(self)
        }
    }
}

impl CheckedSqrt for u128 {
    type Output = u128;

    /// Returns the the square root of an integer, or `None` if the integer is not a perfect
    /// square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    /// Constant-time addition, squaring, bit-shifting, and comparison is assumed.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// For `u128`, using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large `u128`s. To overcome this, large
    /// `u128`s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that
    /// $2^{14} \leq \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn checked_sqrt(self) -> Option<u128> {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            _checked_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            _checked_sqrt_binary(self)
        }
    }
}

impl SqrtRem for u128 {
    type SqrtOutput = u128;
    type RemOutput = u128;

    /// Returns the floor of the square root of an integer, and the remainder (the difference
    /// between the integer and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    /// Constant-time addition, squaring, bit-shifting, and comparison is assumed.
    ///
    /// # Examples
    /// See the documentation of the `num::arithmetic::sqrt` module.
    ///
    /// # Notes
    /// For `u128`, using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large `u128`s. To overcome this, large
    /// `u128`s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that
    /// $2^{14} \leq \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn sqrt_rem(self) -> (u128, u128) {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            _sqrt_rem_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            _sqrt_rem_binary(self)
        }
    }
}

macro_rules! impl_sqrt_signed {
    ($u: ident, $s: ident) => {
        impl FloorSqrt for $s {
            type Output = $s;

            /// Returns the floor of the square root of an integer.
            ///
            /// $f(x) = \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn floor_sqrt(self) -> Self {
                if self >= 0 {
                    $s::wrapping_from(self.unsigned_abs().floor_sqrt())
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl CeilingSqrt for $s {
            type Output = $s;

            /// Returns the ceiling of the square root of an integer.
            ///
            /// $f(x) = \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn ceiling_sqrt(self) -> $s {
                if self >= 0 {
                    $s::wrapping_from(self.unsigned_abs().ceiling_sqrt())
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl CheckedSqrt for $s {
            type Output = $s;

            /// Returns the the square root of an integer, or `None` if the integer is not a
            /// perfect square.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn checked_sqrt(self) -> Option<$s> {
                if self >= 0 {
                    self.unsigned_abs().checked_sqrt().map($s::wrapping_from)
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl SqrtRem for $s {
            type SqrtOutput = $s;
            type RemOutput = $u;

            /// Returns the floor of the square root of an integer, and the remainder (the
            /// difference between the integer and the square of the floor).
            ///
            /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn sqrt_rem(self) -> ($s, $u) {
                if self >= 0 {
                    let (sqrt, rem) = self.unsigned_abs().sqrt_rem();
                    ($s::wrapping_from(sqrt), rem)
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl SqrtRemAssign for $s {
            type RemOutput = $u;

            /// Replaces an integer with the floor of its square root, and returns the remainder
            /// (the difference between the original integer and the square of the floor).
            ///
            /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
            ///
            /// $x \gets \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn sqrt_rem_assign(&mut self) -> $u {
                let (sqrt, rem) = self.sqrt_rem();
                *self = sqrt;
                rem
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_sqrt_signed);

macro_rules! impl_sqrt_rem_assign_unsigned {
    ($t: ident) => {
        impl SqrtRemAssign for $t {
            type RemOutput = $t;

            /// Replaces an integer with the floor of its square root, and returns the remainder
            /// (the difference between the original integer and the square of the floor).
            ///
            /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
            ///
            /// $x \gets \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn sqrt_rem_assign(&mut self) -> $t {
                let (sqrt, rem) = self.sqrt_rem();
                *self = sqrt;
                rem
            }
        }
    };
}
apply_to_unsigneds!(impl_sqrt_rem_assign_unsigned);

macro_rules! impl_sqrt_assign {
    ($t: ident) => {
        impl FloorSqrtAssign for $t {
            /// Replaces an integer with the floor of its square root.
            ///
            /// $x \gets \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn floor_sqrt_assign(&mut self) {
                *self = self.floor_sqrt();
            }
        }

        impl CeilingSqrtAssign for $t {
            /// Replaces an integer with the ceiling of its square root.
            ///
            /// $x \gets \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::sqrt` module.
            #[inline]
            fn ceiling_sqrt_assign(&mut self) {
                *self = self.ceiling_sqrt();
            }
        }
    };
}
apply_to_primitive_ints!(impl_sqrt_assign);
