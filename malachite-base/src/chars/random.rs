use chars::crement::{char_to_contiguous_range, contiguous_range_to_char, decrement_char};
use comparison::traits::{Max, Min};
use num::random::{random_unsigned_inclusive_range, RandomUnsignedInclusiveRange};
use random::Seed;

/// Uniformly generates random `char`s in a closed interval.
///
/// This `struct` is created by the `random_char_range` and `random_char_inclusive_range` methods.
/// See their documentation for more.
#[derive(Clone, Debug)]
pub struct RandomCharRange {
    chunks: RandomUnsignedInclusiveRange<u32>,
}

impl Iterator for RandomCharRange {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        contiguous_range_to_char(self.chunks.next().unwrap())
    }
}

/// Uniformly generates random `char`s.
///
/// $P(c) = \frac{1}{2^{20}+2^{16}-2^{11}}$.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_chars(EXAMPLE_SEED).take(10).collect::<String>().as_str(),
///     "\u{5f771}\u{87234}\u{bcd36}\u{9e195}\u{5da07}\u{36553}\u{45028}\u{1cdfd}\u{d8530}\u{c7f2e}"
/// )
/// ```
pub fn random_chars(seed: Seed) -> RandomCharRange {
    random_char_inclusive_range(seed, char::MIN, char::MAX)
}

/// Uniformly generates random ASCII `char`s.
///
/// $$
/// P(c) = \\begin{cases}
///     2^{-7} & c < \mathrm{char\\_to\\_contiguous\\_range(2^7)} \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_ascii_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_ascii_chars(EXAMPLE_SEED).take(20).collect::<String>().as_str(),
///     "q^\u{17}bF\\4T!/\u{1}q6\n/\u{11}Y\\wB"
/// )
/// ```
pub fn random_ascii_chars(seed: Seed) -> RandomCharRange {
    random_char_inclusive_range(seed, char::MIN, '\u{7f}')
}

/// Uniformly generates random `char`s in the half-open interval $[a, b)$.
///
/// `a` must be less than `b`. This function cannot create a range that includes `char::MAX`; for
/// that, use `random_char_inclusive_range`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}
///     {\mathrm{char\\_to\\_contiguous\\_range(b)}-\mathrm{char\\_to\\_contiguous\\_range(a)}} &
///         a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_char_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_char_range(EXAMPLE_SEED, 'a', 'z').take(50).collect::<String>().as_str(),
///     "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqes"
/// )
/// ```
#[inline]
pub fn random_char_range(seed: Seed, a: char, mut b: char) -> RandomCharRange {
    if a >= b {
        panic!("a must be less than b. a: {}, b: {}", a, b);
    }
    decrement_char(&mut b);
    random_char_inclusive_range(seed, a, b)
}

/// Uniformly generates random `char`s in the closed interval $[a, b]$.
///
/// `a` must be less than or equal to `b`.
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{1}
///         {\mathrm{char\\_to\\_contiguous\\_range(b)}-\mathrm{char\\_to\\_contiguous\\_range(a)}
///         +1} &
///         a \leq x < b \\\\
///     0 & \\text{otherwise}
/// \\end{cases}
/// $$
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
///
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_char_inclusive_range(EXAMPLE_SEED, 'a', 'z').take(50).collect::<String>().as_str(),
///     "rlewrsgkdlbeouylrelopxqkoonftexoshqulgvonioatekqes"
/// )
/// ```
#[inline]
pub fn random_char_inclusive_range(seed: Seed, a: char, b: char) -> RandomCharRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomCharRange {
        chunks: random_unsigned_inclusive_range(
            seed,
            char_to_contiguous_range(a),
            char_to_contiguous_range(b),
        ),
    }
}
