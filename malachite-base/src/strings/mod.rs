use itertools::Itertools;
use named::Named;
use std::collections::HashSet;
use std::fmt::{Binary, Debug, LowerHex, Octal, UpperHex};

/// Sorts the characters of a string and returns them in a new `String`.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
///
/// # Examples
/// ```
/// use malachite_base::strings::string_sort;
///
/// assert_eq!(string_sort("Hello, world!"), " !,Hdellloorw");
/// assert_eq!(string_sort("Mississippi"), "Miiiippssss");
/// ```
pub fn string_sort(s: &str) -> String {
    let mut chars = s.chars().collect_vec();
    chars.sort_unstable();
    chars.iter().collect()
}

/// Removes duplicate characters from a string and returns the result in a new `String`.
///
/// The unique characters are output in order of appearance.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
///
/// # Examples
/// ```
/// use malachite_base::strings::string_unique;
///
/// assert_eq!(string_unique("Hello, world!"), "Helo, wrd!");
/// assert_eq!(string_unique("Mississippi"), "Misp");
/// ```
pub fn string_unique(s: &str) -> String {
    let mut chars = HashSet::new();
    let mut nub = String::new();
    for c in s.chars() {
        if chars.insert(c) {
            nub.push(c);
        }
    }
    nub
}

/// Returns whether all of the first `&str`'s characters are present in the second `&str`.
///
/// Does not take multiplicities into account.
///
/// # Worst-case complexity
/// $T(n) = O(n)$
///
/// $M(n) = O(n + m)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `s.len()`, and $m$ is `t.len()`.
///
/// # Examples
/// ```
/// use malachite_base::strings::string_is_subset;
///
/// assert_eq!(string_is_subset("oH, well", "Hello, world!"), true);
/// assert_eq!(string_is_subset("MMM", "Mississippi"), true);
/// assert_eq!(string_is_subset("Hello, World!", "Hello, world!"), false);
/// assert_eq!(string_is_subset("j", "Mississippi"), false);
/// ```
pub fn string_is_subset(s: &str, t: &str) -> bool {
    let t_chars: HashSet<char> = t.chars().collect();
    s.chars().all(|c| t_chars.contains(&c))
}

impl_named!(String);

/// A trait that provides an ergonomic way to create the string specified by a `Debug`
/// implementation.
pub trait ToDebugString: Debug {
    fn to_debug_string(&self) -> String;
}

impl<T: Debug> ToDebugString for T {
    /// Returns the `String` produced by `T`s `Debug` implementation.
    ///
    /// Time: depends on `Debug` implementation
    ///
    /// Additional memory: depends on `Debug` implementation
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    ///
    /// assert_eq!([1, 2, 3].to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     [vec![2, 3], vec![], vec![4]].to_debug_string(),
    ///     "[[2, 3], [], [4]]"
    /// );
    /// assert_eq!(Some(5).to_debug_string(), "Some(5)");
    /// ```
    #[inline]
    fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
}

/// A trait that provides an ergonomic way to create the string specified by a `Binary`
/// implementation.
pub trait ToBinaryString: Binary {
    fn to_binary_string(&self) -> String;
}

impl<T: Binary> ToBinaryString for T {
    /// Returns the `String` produced by `T`s `Binary` implementation.
    ///
    /// Time: depends on `Binary` implementation
    ///
    /// Additional memory: depends on `Binary` implementation
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToBinaryString;
    ///
    /// assert_eq!(5u64.to_binary_string(), "101");
    /// assert_eq!((-100i16).to_binary_string(), "1111111110011100");
    /// ```
    #[inline]
    fn to_binary_string(&self) -> String {
        format!("{:b}", self)
    }
}

/// A trait that provides an ergonomic way to create the string specified by an `Octal`
/// implementation.
pub trait ToOctalString: Octal {
    fn to_octal_string(&self) -> String;
}

impl<T: Octal> ToOctalString for T {
    /// Returns the `String` produced by `T`s `Octal` implementation.
    ///
    /// Time: depends on `Octal` implementation
    ///
    /// Additional memory: depends on `Octal` implementation
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToOctalString;
    ///
    /// assert_eq!(50u64.to_octal_string(), "62");
    /// assert_eq!((-100i16).to_octal_string(), "177634");
    /// ```
    #[inline]
    fn to_octal_string(&self) -> String {
        format!("{:o}", self)
    }
}

/// A trait that provides an ergonomic way to create the string specified by a `LowerHex`
/// implementation.
pub trait ToLowerHexString: LowerHex {
    fn to_lower_hex_string(&self) -> String;
}

impl<T: LowerHex> ToLowerHexString for T {
    /// Returns the `String` produced by `T`s `LowerHex` implementation.
    ///
    /// Time: depends on `LowerHex` implementation
    ///
    /// Additional memory: depends on `LowerHex` implementation
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToLowerHexString;
    ///
    /// assert_eq!(50u64.to_lower_hex_string(), "32");
    /// assert_eq!((-100i16).to_lower_hex_string(), "ff9c");
    /// ```
    #[inline]
    fn to_lower_hex_string(&self) -> String {
        format!("{:x}", self)
    }
}

/// A trait that provides an ergonomic way to create the string specified by an `UpperHex`
/// implementation.
pub trait ToUpperHexString: UpperHex {
    fn to_upper_hex_string(&self) -> String;
}

impl<T: UpperHex> ToUpperHexString for T {
    /// Returns the `String` produced by `T`s `UpperHex` implementation.
    ///
    /// Time: depends on `UpperHex` implementation
    ///
    /// Additional memory: depends on `UpperHex` implementation
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToUpperHexString;
    ///
    /// assert_eq!(50u64.to_upper_hex_string(), "32");
    /// assert_eq!((-100i16).to_upper_hex_string(), "FF9C");
    /// ```
    #[inline]
    fn to_upper_hex_string(&self) -> String {
        format!("{:X}", self)
    }
}

/// Generates `String`s, given an iterator that generates `Vec<char>`s.
///
/// This `struct` is created by the `strings_from_char_vecs` function. See its documentation for
/// more.
#[derive(Clone, Debug)]
pub struct StringsFromCharVecs<I: Iterator<Item = Vec<char>>> {
    css: I,
}

impl<I: Iterator<Item = Vec<char>>> Iterator for StringsFromCharVecs<I> {
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<String> {
        self.css.next().map(|cs| cs.into_iter().collect())
    }
}

/// Generates `String`s, given an iterator that generates `Vec<char>`s.
///
/// The elements appear in the same order as they do in the given iterator, but as `String`s.
///
/// The output length is `css.count()`.
///
/// # Complexity per iteration
/// Same as the time and additional memory complexity of iterating `css`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::strings::strings_from_char_vecs;
///
/// let ss =
///     &strings_from_char_vecs([vec!['a', 'b'], vec!['c', 'd']].iter().cloned()).collect_vec();
/// assert_eq!(
///     ss.iter().map(|cs| cs.as_str()).collect_vec().as_slice(),
///     &["ab", "cd"]
/// );
/// ```
#[inline]
pub fn strings_from_char_vecs<I: Iterator<Item = Vec<char>>>(css: I) -> StringsFromCharVecs<I> {
    StringsFromCharVecs { css }
}

//TODO doc and test

pub trait ExtraToString {
    fn to_string(&self) -> String;
}

/// This module contains iterators that generate `String`s without repetition.
pub mod exhaustive;
/// This module contains iterators that generate `String`s randomly.
pub mod random;
