use bools::random::{random_bools, RandomBools};
use num::arithmetic::traits::Parity;
use num::basic::integers::PrimitiveInteger;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::CheckedInto;
use num::random::random_unsigneds_less_than;
use num::random::random_unsigneds_less_than::RandomUnsignedsLessThan;
use random::seed::Seed;

#[derive(Clone, Debug)]
pub struct GeometricRandomNaturalValues<T: PrimitiveInteger> {
    xs: RandomUnsignedsLessThan<u64>,
    numerator: u64,
    min: T,
    max: T,
}

impl<T: PrimitiveInteger> Iterator for GeometricRandomNaturalValues<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut failures = self.min;
        loop {
            if self.xs.next().unwrap() < self.numerator {
                return Some(failures);
            } else {
                // Wrapping to min is equivalent to restarting this function.
                if failures == self.max {
                    failures = self.min;
                } else {
                    failures += T::ONE;
                }
            }
        }
    }
}

//TODO use actual gcd
fn gcd(u: u64, v: u64) -> u64 {
    if u == v {
        u
    } else if u == 0 {
        v
    } else if v == 0 {
        u
    } else if u.even() {
        if v.odd() {
            gcd(u >> 1, v)
        } else {
            gcd(u >> 1, v >> 1) << 1
        }
    } else if v.even() {
        gcd(u, v >> 1)
    } else if u > v {
        gcd((u - v) >> 1, v)
    } else {
        gcd((v - u) >> 1, u)
    }
}

pub(crate) fn mean_to_p_with_min<T: PrimitiveInteger>(
    um_numerator: u64,
    um_denominator: u64,
    min: T,
) -> (u64, u64) {
    let gcd = gcd(um_numerator, um_denominator);
    let (um_numerator, um_denominator) = (um_numerator / gcd, um_denominator / gcd);
    let um_numerator = um_numerator
        .checked_sub(
            um_denominator
                .checked_mul(CheckedInto::<u64>::checked_into(min).unwrap())
                .unwrap(),
        )
        .unwrap();
    (
        um_denominator,
        um_numerator.checked_add(um_denominator).unwrap(),
    )
}

fn geometric_random_natural_values_range<T: PrimitiveInteger>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomNaturalValues<T> {
    assert!(min < max);
    assert_ne!(um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(um_numerator, um_denominator, min);
    GeometricRandomNaturalValues {
        xs: random_unsigneds_less_than(seed, denominator),
        numerator,
        min,
        max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomNegativeSigneds<T: PrimitiveSigned> {
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
    abs_min: T,
    abs_max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomNegativeSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut result = self.abs_min;
        loop {
            if self.xs.next().unwrap() < self.abs_numerator {
                return Some(result);
            } else {
                // Wrapping to min is equivalent to restarting this function.
                if result == self.abs_max {
                    result = self.abs_min;
                } else {
                    result -= T::ONE;
                }
            }
        }
    }
}

fn geometric_random_negative_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    abs_min: T,
    abs_max: T,
) -> GeometricRandomNegativeSigneds<T> {
    assert!(abs_min > abs_max);
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(
        abs_um_numerator,
        abs_um_denominator,
        abs_min.checked_neg().unwrap(),
    );
    GeometricRandomNegativeSigneds {
        xs: random_unsigneds_less_than(seed, denominator),
        abs_numerator: numerator,
        abs_min,
        abs_max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomNonzeroSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
    min: T,
    max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomNonzeroSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            if self.bs.next().unwrap() {
                let mut result = T::ONE;
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        return Some(result);
                    } else if result == self.max {
                        break;
                    } else {
                        result += T::ONE;
                    }
                }
            } else {
                let mut result = T::NEGATIVE_ONE;
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        return Some(result);
                    } else if result == self.min {
                        break;
                    } else {
                        result -= T::ONE;
                    }
                }
            }
        }
    }
}

fn geometric_random_nonzero_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomNonzeroSigneds<T> {
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) = mean_to_p_with_min(abs_um_numerator, abs_um_denominator, T::ONE);
    GeometricRandomNonzeroSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: random_unsigneds_less_than(seed.fork("xs"), denominator),
        abs_numerator: numerator,
        min,
        max,
    }
}

#[derive(Clone, Debug)]
pub struct GeometricRandomSigneds<T: PrimitiveSigned> {
    bs: RandomBools,
    xs: RandomUnsignedsLessThan<u64>,
    abs_numerator: u64,
    min: T,
    max: T,
}

impl<T: PrimitiveSigned> Iterator for GeometricRandomSigneds<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        loop {
            let mut result = T::ZERO;
            if self.bs.next().unwrap() {
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        } else {
                            return Some(result);
                        }
                    } else if result == self.max {
                        break;
                    } else {
                        result += T::ONE;
                    }
                }
            } else {
                loop {
                    if self.xs.next().unwrap() < self.abs_numerator {
                        if result == T::ZERO && self.bs.next().unwrap() {
                            break;
                        } else {
                            return Some(result);
                        }
                    } else if result == self.min {
                        break;
                    } else {
                        result -= T::ONE;
                    }
                }
            }
        }
    }
}

fn geometric_random_signeds_range<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    min: T,
    max: T,
) -> GeometricRandomSigneds<T> {
    assert_ne!(abs_um_denominator, 0);
    let (numerator, denominator) =
        mean_to_p_with_min(abs_um_numerator, abs_um_denominator, T::ZERO);
    GeometricRandomSigneds {
        bs: random_bools(seed.fork("bs")),
        xs: random_unsigneds_less_than(seed.fork("xs"), denominator),
        abs_numerator: numerator,
        min,
        max,
    }
}

/// Generates random unsigned integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities of $P(0), P(1), P(2), \ldots$ decrease in a geometric sequence;
/// that's were the "geometric" comes from. Unlike a true geometric distribution, this distribution
/// is truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator` / `um_denominator`. The unadjusted mean is
/// what the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator` + `um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[0, 2^W)$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `um_numerator` + `um_denominator`.
///
/// # Panics
/// Panics if `um_numerator` or `um_denominator` are zero, or, if after being reduced to lowest
/// terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_unsigneds;
///
/// assert_eq!(
///     geometric_random_unsigneds::<u64>(EXAMPLE_SEED, 1, 1).take(10).collect::<Vec<_>>(),
///     &[1, 0, 0, 3, 4, 4, 1, 0, 0, 1]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{1-(1-p)^{2^W}} & 0 \\leq n < 2^W \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(0) = p = \frac{1}{m_u + 1}.
/// $$
pub fn geometric_random_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert_ne!(um_numerator, 0);
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ZERO, T::MAX)
}

/// Generates random positive unsigned integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities of $P(1), P(2), P(3), \ldots$ decrease in a geometric sequence;
/// that's were the "geometric" comes from. Unlike a true geometric distribution, this distribution
/// is truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator` / `um_denominator`. The unadjusted mean is
/// what the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than one. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator` + `um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[1, 2^W)$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `um_numerator` + `um_denominator`.
///
/// # Panics
/// Panics if `um_denominator` is zero or if `um_numerator` <= `um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_positive_unsigneds;
///
/// assert_eq!(
///     geometric_random_positive_unsigneds::<u64>(EXAMPLE_SEED, 2, 1)
///         .take(10).collect::<Vec<_>>(),
///     &[2, 1, 1, 4, 5, 5, 2, 1, 1, 2]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{n-1}p}{1-(1-p)^{2^W-1}} & 0 < n < 2^W \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(1) = p = \frac{1}{m_u}.
/// $$
pub fn geometric_random_positive_unsigneds<T: PrimitiveUnsigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert!(um_numerator > um_denominator);
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ONE, T::MAX)
}

/// Generates random signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by mirroring it,
/// producing a truncated double geometric distribution. Zero is included.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities of $P(0), P(\pm 1), P(\pm 2), \ldots$ decrease in a
/// geometric sequence; that's were the "geometric" comes from. Values below `T::MIN` or above
/// `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator` / `abs_um_denominator`. The unadjusted
/// mean is what the mean generated value would be if the distribution were not truncated, and were
/// restricted to non-negative values. If $m_u$ is significantly lower than `T::MAX`, which is
/// usually the case, then it is very close to the actual mean of the distribution restricted to
/// positive values. The higher $m_u$ is, the more gently the probabilities drop; the lower it is,
/// the more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but
/// note that the iteration time increases linearly with `abs_um_numerator` + `abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 2^{W-1})$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// Whenever $n \geq 0$ and $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1},
/// $$
/// and whenever $n \leq 0$ and $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `abs_um_numerator` + `abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_numerator` or `abs_um_denominator` are zero, or, if after being reduced to
/// lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_signeds;
///
/// assert_eq!(
///     geometric_random_signeds::<i64>(EXAMPLE_SEED, 1, 1).take(10).collect::<Vec<_>>(),
///     &[-1, -1, -1, 1, -2, 1, 0, 0, 0, 0]
/// )
/// ```
///
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{|n|}p}{((1-p)^{2^{W-1}}-1)(p-2)} & 0 < n < 2^W \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(0) = \frac{p}{2-p} = \frac{1}{2 m_u + 1}.
/// $$
pub fn geometric_random_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomSigneds<T> {
    assert_ne!(abs_um_numerator, 0);
    geometric_random_signeds_range(seed, abs_um_numerator, abs_um_denominator, T::MIN, T::MAX)
}

/// Generates random natural (non-negative) signed integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities of $P(0), P(1), P(2), \ldots$ decrease in a geometric sequence;
/// that's were the "geometric" comes from. Unlike a true geometric distribution, this distribution
/// is truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator` / `um_denominator`. The unadjusted mean is
/// what the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than zero. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator` + `um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[0, 2^{W-1})$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u + 1}{m_u}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `um_numerator` + `um_denominator`.
///
/// # Panics
/// Panics if `um_numerator` or `um_denominator` are zero, or, if after being reduced to lowest
/// terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_natural_signeds;
///
/// assert_eq!(
///     geometric_random_natural_signeds::<i64>(EXAMPLE_SEED, 1, 1).take(10).collect::<Vec<_>>(),
///     &[1, 0, 0, 3, 4, 4, 1, 0, 0, 1]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p} - 1$, or $p = \frac{1}{m_u + 1}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^np}{1-(1-p)^{2^{W-1}}} & 0 \\leq n < 2^{W-1} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(0) = p = \frac{1}{m_u + 1}.
/// $$
pub fn geometric_random_natural_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    assert_ne!(um_numerator, 0);
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ZERO, T::MAX)
}

/// Generates random positive signed integers from a truncated geometric distribution.
///
/// With this distribution, the probability of a value being generated decreases as the value
/// increases. The probabilities of $P(1), P(2), P(3), \ldots$ decrease in a geometric sequence;
/// that's were the "geometric" comes from. Unlike a true geometric distribution, this distribution
/// is truncated, meaning that values above `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `um_numerator` / `um_denominator`. The unadjusted mean is
/// what the mean generated value would be if the distribution were not truncated. If $m_u$ is
/// significantly lower than `T::MAX`, which is usually the case, then it is very close to the
/// actual mean. The higher $m_u$ is, the more gently the probabilities drop; the lower it is, the
/// more quickly they drop. $m_u$ must be greater than one. It may be arbitrarily high, but note
/// that the iteration time increases linearly with `um_numerator` + `um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[1, 2^{W-1})$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `um_numerator` + `um_denominator`.
///
/// # Panics
/// Panics if `um_denominator` is zero or if `um_numerator` <= `um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_positive_signeds;
///
/// assert_eq!(
///     geometric_random_positive_signeds::<i64>(EXAMPLE_SEED, 2, 1).take(10).collect::<Vec<_>>(),
///     &[2, 1, 1, 4, 5, 5, 2, 1, 1, 2]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{n-1}p}{1-(1-p)^{2^{W-1}-1}} & 0 < n < 2^{W-1} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(1) = p = \frac{1}{m_u}.
/// $$
pub fn geometric_random_positive_signeds<T: PrimitiveSigned>(
    seed: Seed,
    um_numerator: u64,
    um_denominator: u64,
) -> GeometricRandomNaturalValues<T> {
    geometric_random_natural_values_range(seed, um_numerator, um_denominator, T::ONE, T::MAX)
}

/// Generates random negative signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by negating its domain.
/// The distribution is truncated at `T::MIN`.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities of $P(-1), P(-2), P(-3), \ldots$ decrease in a geometric
/// sequence; that's were the "geometric" comes from. Values below `T::MIN` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator` / `abs_um_denominator`. The unadjusted
/// mean is what the absolute value of the mean generated value would be if the distribution were
/// not truncated. If $m_u$ is significantly lower than `-T::MIN`, which is usually the case, then
/// it is very close to the actual absolute value of the mean. The higher $m_u$ is, the more gently
/// the probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be greater than
/// one. It may be arbitrarily high, but note that the iteration time increases linearly with
/// `abs_um_numerator` + `abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 0)$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
///
/// and whenever $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `abs_um_numerator` + `abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_denominator` is zero or if `abs_um_numerator` <= `abs_um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_negative_signeds;
///
/// assert_eq!(
///     geometric_random_negative_signeds::<i64>(EXAMPLE_SEED, 2, 1).take(10).collect::<Vec<_>>(),
///     &[-2, -1, -1, -4, -5, -5, -2, -1, -1, -2]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{-n-1}p}{1-(1-p)^{2^{W-1}}} & -2^{W-1} \leq n < 0 \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(-1) = p = \frac{1}{m_u}.
/// $$
pub fn geometric_random_negative_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNegativeSigneds<T> {
    assert!(abs_um_numerator > abs_um_denominator);
    geometric_random_negative_signeds_range(
        seed,
        abs_um_numerator,
        abs_um_denominator,
        T::NEGATIVE_ONE,
        T::MIN,
    )
}

/// Generates random nonzero signed integers from a modified geometric distribution.
///
/// This distribution can be derived from a truncated geometric distribution by mirroring it,
/// producing a truncated double geometric distribution. Zero is excluded.
///
/// With this distribution, the probability of a value being generated decreases as its absolute
/// value increases. The probabilities of $P(\pm 1), P(\pm 2), P(\pm 3), \ldots$ decrease in a
/// geometric sequence; that's were the "geometric" comes from. Values below `T::MIN` or above
/// `T::MAX` are never generated.
///
/// The probabilities can drop more quickly or more slowly depending on a parameter $m_u$, called
/// the unadjusted mean. It is equal to `abs_um_numerator` / `abs_um_denominator`. The unadjusted
/// mean is what the absolute value of the mean generated value would be if the distribution were
/// not truncated. If $m_u$ is significantly lower than `T::MAX`, which is usually the case, then it
/// is very close to the actual absolute value of the mean. The higher $m_u$ is, the more gently the
/// probabilities drop; the lower it is, the more quickly they drop. $m_u$ must be greater than one.
/// It may be arbitrarily high, but note that the iteration time increases linearly with
/// `abs_um_numerator` + `abs_um_denominator`.
///
/// Here is a more precise characterization of this distribution. Let its support $S \subset \Z$
/// equal $[-2^{W-1}, 2^{W-1}) \setminus \\{0\\}$, where $W$ is `T::WIDTH`. Then we have
/// $$
///     P(n) \neq 0 \leftrightarrow n \in S
/// $$
/// $$
///     P(1) = P(-1)
/// $$
/// Whenever $n > 0$ and $n, n + 1 \in S$,
/// $$
/// \frac{P(n)}{P(n+1)} = \frac{m_u}{m_u - 1},
/// $$
/// and whenever $n < 0$ and $n, n - 1 \in S$,
/// $$
/// \frac{P(n)}{P(n-1)} = \frac{m_u}{m_u - 1}.
/// $$
///
/// As a corollary, $P(n) = P(-n)$ whenever $n, -n \in S$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
/// $E[T(i)] = \mathcal{O}(n)$
///
/// $E[M(i)] = \mathcal{O}(1)$
///
/// where $n$ = `abs_um_numerator` + `abs_um_denominator`.
///
/// # Panics
/// Panics if `abs_um_denominator` is zero or if `abs_um_numerator` <= `abs_um_denominator`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::num::random::geometric::geometric_random_nonzero_signeds;
///
/// assert_eq!(
///     geometric_random_nonzero_signeds::<i64>(EXAMPLE_SEED, 2, 1).take(10).collect::<Vec<_>>(),
///     &[-2, -2, -2, 2, -3, 2, -1, -1, -1, 1]
/// )
/// ```
///
/// # Further details
/// Geometric distributions are more typically parametrized by a parameter $p$. The relationship
/// between $p$ and $m_u$ is $m_u = \frac{1}{p}$, or $p = \frac{1}{m_u}$.
///
/// The probability mass function of this distribution is
/// $$
/// P(n) = \\begin{cases}
///     \frac{(1-p)^{|n|}p}{(1-p)^{2^{W-1}}(p-2)-2p+2} &
///         -2^{W-1} \leq n < 0 \\ \mathrm{or} \\ 0 < n < -2^{W-1} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
/// where $W$ is `T::WIDTH`.
///
/// It's also useful to note that
/// $$
///     \lim_{W \to \infty} P(1) = \frac{p}{2} = \frac{1}{2 m_u}.
/// $$
pub fn geometric_random_nonzero_signeds<T: PrimitiveSigned>(
    seed: Seed,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
) -> GeometricRandomNonzeroSigneds<T> {
    assert!(abs_um_numerator > abs_um_denominator);
    geometric_random_nonzero_signeds_range(
        seed,
        abs_um_numerator,
        abs_um_denominator,
        T::MIN,
        T::MAX,
    )
}
