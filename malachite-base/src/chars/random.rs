use chars::char_is_graphic;
use chars::crement::{char_to_contiguous_range, contiguous_range_to_char, decrement_char};
use comparison::traits::{Max, Min};
use num::random::geometric::SimpleRational;
use num::random::{
    random_unsigned_inclusive_range, random_unsigneds_less_than, RandomUnsignedInclusiveRange,
    RandomUnsignedsLessThan,
};
use random::Seed;
use vecs::{random_values_from_vec, RandomValuesFromVec};

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

/// Uniformly generates random `char`s in a closed interval, weighting graphic and non-graphic
/// `char`s separately.
///
/// This `struct` is created by the `graphic_weighted_random_char_range` and
/// `graphic_weighted_random_char_inclusive_range` methods. See their documentation for more.
#[derive(Clone, Debug)]
pub struct WeightedGraphicRandomCharRange {
    numerator: u64,
    xs: RandomUnsignedsLessThan<u64>,
    graphic: RandomValuesFromVec<char>,
    non_graphic: RandomValuesFromVec<char>,
}

impl Iterator for WeightedGraphicRandomCharRange {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if self.xs.next().unwrap() < self.numerator {
            self.graphic.next()
        } else {
            self.non_graphic.next()
        }
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
///     2^{-7} & c < \\backslash\\text{u\\{0x80\\}} \\\\
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

/// Generates random `char`s, weighting graphic and non-graphic `char`s separately.
///
/// See `char_is_graphic` for the definition of a graphic `char`.
///
/// Given a weight $w$ = `w_numerator` / `w_denominator`, the set of graphic `char`s is selected
/// with probability $\frac{w}{w+1}$ and the set of non-graphic `chars` with probability
/// $\frac{1}{w+1}$. Then, a `char` is selected uniformly from the appropriate set. There are
/// 141,798 graphic `char`s out of 1,112,064, so we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{w}{141798(w+1)} & x \\ \\text{is} \\ \\text{graphic} \\\\
///     \frac{1}{970266w} & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $w = 23633/161711$, which is roughly $1/7$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `w_numerator` or `w_denominator` are zero.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_chars(EXAMPLE_SEED, 10, 1).take(20).collect::<String>().as_str(),
///     "𘓸𰫖祝껆ꆍ枽쮱𬭊▟𣡌⢻𱉳灋\u{8c401}ՠ𦷆𪑘\u{369b5}\u{d5da0}𧎊"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_chars(
    seed: Seed,
    w_numerator: u64,
    w_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    graphic_weighted_random_char_inclusive_range(
        seed,
        char::MIN,
        char::MAX,
        w_numerator,
        w_denominator,
    )
}

/// Generates random ASCII `char`s, weighting graphic and non-graphic `char`s separately.
///
/// See `char_is_graphic` for the definition of a graphic `char`.
///
/// Given a weight $w$ = `w_numerator` / `w_denominator`, the set of graphic ASCII `char`s is
/// selected with probability $\frac{w}{w+1}$ and the set of non-graphic ASCII `chars` with
/// probability $\frac{1}{w+1}$. Then, a `char` is selected uniformly from the appropriate set.
/// There are 95 graphic ASCII `char`s out of 128, so we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{w}{95(w+1)} &
///     x < \\backslash\\text{u\\{0x80\\}} \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1}{33w} &
///     x < \\backslash\\text{u\\{0x80\\}} \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $w = 95/33$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `w_numerator` or `w_denominator` are zero.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_ascii_chars;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_ascii_chars(EXAMPLE_SEED, 10, 1)
///         .take(40).collect::<String>().as_str(),
///     "x14N(bcXr$g)7\u{1b}/E+\u{8}\rf\u{2}\u{11}Y\u{11}Poo.$V2R.$V=6\u{13}\t\u{11}"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_ascii_chars(
    seed: Seed,
    w_numerator: u64,
    w_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    graphic_weighted_random_char_inclusive_range(
        seed,
        char::MIN,
        '\u{7f}',
        w_numerator,
        w_denominator,
    )
}

/// Generates random `char`s in the half-open interval $[a, b)$, weighting graphic and non-graphic
/// `char`s separately.
///
/// See `char_is_graphic` for the definition of a graphic `char`.
///
/// `a` must be less than `b`. Furthermore, $[a, b)$ must contain both graphic and non-graphic
/// `char`s. This function cannot create a range that includes `char::MAX`; for that, use
/// `graphic_weighted_random_char_inclusive_range`.
///
/// Given a weight $w$ = `w_numerator` / `w_denominator`, the set of graphic `char`s in the
/// specified range is selected with probability $\frac{w}{w+1}$ and the set of non-graphic `chars`
/// in the range with probability $\frac{1}{w+1}$. Then, a `char` is selected uniformly from the
/// appropriate set.
///
/// Let $g$ be the number of graphic `char`s in $[a, b)$. Then we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{w}{g(w+1)} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1}{(b-a-g)w} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $w = g/(b-a-g)$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `w_numerator` or `w_denominator` are zero, if $a \geq b$, if $[a, b)$ contains no
/// graphic `char`s, or if $[a, b)$ contains only graphic `char`s.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_char_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_char_range(EXAMPLE_SEED, '\u{100}', '\u{400}', 10, 1)
///         .take(30).collect::<String>().as_str(),
///     "ǘɂŜȢΙƘƣʅΰǟ˳ˊȇ\u{31b}ʰɥΈ\u{324}\u{35a}Ϟ\u{367}\u{337}ƃ\u{342}ʌμƢϳϪǰ"
/// )
/// ```
#[inline]
pub fn graphic_weighted_random_char_range(
    seed: Seed,
    a: char,
    mut b: char,
    w_numerator: u64,
    w_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    if a >= b {
        panic!("a must be less than b. a: {}, b: {}", a, b);
    }
    decrement_char(&mut b);
    graphic_weighted_random_char_inclusive_range(seed, a, b, w_numerator, w_denominator)
}

/// Generates random `char`s in the closed interval $[a, b]$, weighting graphic and non-graphic
/// `char`s separately.
///
/// See `char_is_graphic` for the definition of a graphic `char`.
///
/// `a` must be less than or equal to `b`. Furthermore, $[a, b]$ must contain both graphic and non-
/// graphic `char`s.
///
/// Given a weight $w$ = `w_numerator` / `w_denominator`, the set of graphic `char`s in the
/// specified range is selected with probability $\frac{w}{w+1}$ and the set of non-graphic `chars`
/// in the range with probability $\frac{1}{w+1}$. Then, a `char` is selected uniformly from the
/// appropriate set.
///
/// Let $g$ be the number of graphic `char`s in $[a, b]$. Then we have
///
/// $$
/// P(x) = \\begin{cases}
///     \frac{w}{g(w+1)} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is graphic} \\\\
///     \frac{1}{(b-a-g+1)w} & a \leq x < b \\ \\text{and} \\ x \\ \\text{is not graphic} \\\\
///     0 & \\text{otherwise.}
/// \\end{cases}
/// $$
///
/// To recover the uniform distribution, use $w = g/(b-a-g+1)$.
///
/// The output length is infinite.
///
/// # Expected complexity per iteration
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `w_numerator` or `w_denominator` are zero, if $a > b$, if $[a, b]$ contains no graphic
/// `char`s, or if $[a, b]$ contains only graphic `char`s.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     graphic_weighted_random_char_inclusive_range(EXAMPLE_SEED, '\u{100}', '\u{3ff}', 10, 1)
///         .take(30).collect::<String>().as_str(),
///     "ǘɂŜȢΙƘƣʅΰǟ˳ˊȇ\u{31b}ʰɥΈ\u{324}\u{35a}Ϟ\u{367}\u{337}ƃ\u{342}ʌμƢϳϪǰ"
/// )
/// ```
pub fn graphic_weighted_random_char_inclusive_range(
    seed: Seed,
    a: char,
    b: char,
    w_numerator: u64,
    w_denominator: u64,
) -> WeightedGraphicRandomCharRange {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    assert_ne!(w_numerator, 0);
    let (graphic_chars, non_graphic_chars): (Vec<_>, Vec<_>) =
        (a..=b).partition(|&c| char_is_graphic(c));
    if graphic_chars.is_empty() {
        panic!("The range {:?}..={:?} contains no graphic chars", a, b);
    }
    if non_graphic_chars.is_empty() {
        panic!("The range {:?}..={:?} only contains graphic chars", a, b);
    }
    let w = SimpleRational::new(w_numerator, w_denominator)
        .inverse()
        .add_u64(1)
        .inverse();
    WeightedGraphicRandomCharRange {
        numerator: w.n,
        xs: random_unsigneds_less_than(seed.fork("xs"), w.d),
        graphic: random_values_from_vec(seed.fork("graphic"), graphic_chars),
        non_graphic: random_values_from_vec(seed.fork("non_graphic"), non_graphic_chars),
    }
}
