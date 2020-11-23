use chars::exhaustive::{exhaustive_chars, ExhaustiveChars};
use strings::strings_from_char_vecs;
use strings::StringsFromCharVecs;
use vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, lex_fixed_length_vecs_from_single,
    ExhaustiveFixedLengthVecs1Input, LexFixedLengthVecsFromSingle,
};

/// Generates all `String`s of a given length with `char`s from a single iterator, in lexicographic
/// order.
///
/// The order is lexicographic with respect to the order of the element iterator.
///
/// `cs` must be finite.
///
/// The output length is $\ell^n$, where $\ell$ is `cs.count()` and $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty `String`.
///
/// If `cs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// $$
/// T(i, n) = O(n + T^\prime (i))
/// $$
///
/// $$
/// M(i, n) = O(n + M^\prime (i))
/// $$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `cs`.
///
/// # Examples
/// ```
/// use malachite_base::strings::exhaustive::lex_fixed_length_strings_using_chars;
///
/// let ss = lex_fixed_length_strings_using_chars(2, ['c', 'a', 't'].iter().cloned())
///     .collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &["cc", "ca", "ct", "ac", "aa", "at", "tc", "ta", "tt"]
/// );
/// ```
#[inline]
pub fn lex_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: usize,
    cs: I,
) -> StringsFromCharVecs<LexFixedLengthVecsFromSingle<I>> {
    strings_from_char_vecs(lex_fixed_length_vecs_from_single(len, cs))
}

/// Generates all `String`s of a given length in lexicographic order.
///
/// The order is lexicographic with respect to the order of `exhaustive_chars`, which is not the
/// default lexicographic order for `char`s. If you want that order, use
/// `lex_fixed_length_strings_using_chars(len, chars_increasing())`.
///
/// The output length is $1112064^n$, where $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty `String`.
///
/// # Complexity per iteration
///
/// $T(i, n) = O(n)$
///
/// $M(i, n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_base::strings::exhaustive::lex_fixed_length_strings;
///
/// let ss = lex_fixed_length_strings(2).take(20).collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &[
///         "aa", "ab", "ac", "ad", "ae", "af", "ag", "ah", "ai", "aj", "ak", "al", "am", "an",
///         "ao", "ap", "aq", "ar", "as", "at"
///     ]
/// );
/// ```
#[inline]
pub fn lex_fixed_length_strings(
    len: usize,
) -> StringsFromCharVecs<LexFixedLengthVecsFromSingle<ExhaustiveChars>> {
    lex_fixed_length_strings_using_chars(len, exhaustive_chars())
}

/// Generates all `String`s of a given length with `char`s from a single iterator.
///
/// If `cs` is finite, the output length is $\ell^n$, where $\ell$ is `cs.count()` and $n$ is `len`.
/// If `cs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty `String`.
///
/// If `cs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// If `cs` is finite:
///
/// $T(i, n) = O((\ell/2)^n T^\prime(\sqrt\[n\]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt\[n\]{i}))$
///
/// If `cs` is infinite:
///
/// $T(i, n) = O(n + T^\prime(\sqrt\[n\]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt\[n\]{i}))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `cs`.
///
/// # Examples
/// ```
/// use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings_using_chars;
///
/// let ss = exhaustive_fixed_length_strings_using_chars(2, ['c', 'a', 't'].iter().cloned())
///     .collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &["cc", "ca", "ac", "aa", "ct", "at", "tc", "ta", "tt"]
/// );
/// ```
#[inline]
pub fn exhaustive_fixed_length_strings_using_chars<I: Iterator<Item = char>>(
    len: usize,
    cs: I,
) -> StringsFromCharVecs<ExhaustiveFixedLengthVecs1Input<I>> {
    strings_from_char_vecs(exhaustive_fixed_length_vecs_from_single(len, cs))
}

/// Generates all `String`s of a given length.
///
/// The output length is $1112064^n$, where $n$ is `len`.
///
/// If `len` is 0, the output consists of one empty `String`.
///
/// # Complexity per iteration
///
/// If `cs` is finite:
///
/// $T(i, n) = O(556032^n \sqrt\[n\]{i})$
///
/// $M(i, n) = O(n + \sqrt\[n\]{i})$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings;
///
/// let ss = exhaustive_fixed_length_strings(2).take(20).collect::<Vec<_>>();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect::<Vec<_>>().as_slice(),
///     &[
///         "aa", "ab", "ba", "bb", "ac", "ad", "bc", "bd", "ca", "cb", "da", "db", "cc", "cd",
///         "dc", "dd", "ae", "af", "be", "bf"
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_fixed_length_strings(
    len: usize,
) -> StringsFromCharVecs<ExhaustiveFixedLengthVecs1Input<ExhaustiveChars>> {
    exhaustive_fixed_length_strings_using_chars(len, exhaustive_chars())
}
