use chars::random::{random_chars, RandomCharRange};
use random::Seed;
use strings::StringsFromCharVecs;
use vecs::random::{random_fixed_length_vecs_from_single, RandomFixedLengthVecsFromSingle};

/// Randomly generates `String`s of a given length using `char`s from a single iterator.
///
/// The probability of a particular length-$n$ `String` being generated is the product of the
/// probabilities of each of its `char`s.
///
/// If `len` is 0, the output consists of the empty `String`, repeated.
///
/// `cs` must be infinite.
///
/// # Expected complexity per iteration
///
/// $T(n) = O(nT^\prime)$
///
/// $M(n) = O(nM^\prime)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory taken by `cs`.
///
/// # Examples
/// ```
/// use malachite_base::chars::random::random_char_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_fixed_length_strings_using_chars;
///
/// let ss = random_fixed_length_strings_using_chars(
///     2,
///     random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c')
/// ).take(10).collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &["ba", "bc", "bb", "ab", "ac", "ba", "bc", "ca", "ba", "cc"]
/// );
/// ```
#[inline]
pub fn random_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: usize,
    cs: I,
) -> StringsFromCharVecs<RandomFixedLengthVecsFromSingle<I>> {
    StringsFromCharVecs {
        css: random_fixed_length_vecs_from_single(len, cs),
    }
}

/// Randomly generates `String`s of a given length.
///
/// The probability of a particular length-$n$ `String` being generated is $1112064^{-\ell}$, where
/// $\ell$ is `len`.
///
/// If `len` is 0, the output consists of the empty `String`, repeated.
///
/// # Expected complexity per iteration
///
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::strings::random::random_fixed_length_strings;
///
/// let ss = random_fixed_length_strings(EXAMPLE_SEED, 2).take(10).collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &[
///         "\u{5f771}\u{87234}", "\u{bcd36}\u{9e195}", "\u{5da07}\u{36553}", "\u{45028}\u{1cdfd}",
///         "\u{d8530}\u{c7f2e}", "\u{ba4bc}\u{ff677}", "\u{a12e2}\u{d775c}", "\u{f827b}\u{bdf7a}",
///         "簅\u{15aca}", "\u{4e5e2}\u{bb286}"
///     ]
/// );
/// ```
#[inline]
pub fn random_fixed_length_strings(
    seed: Seed,
    len: usize,
) -> StringsFromCharVecs<RandomFixedLengthVecsFromSingle<RandomCharRange>> {
    random_fixed_length_strings_using_chars(len, random_chars(seed))
}
