#![warn(
    clippy::cast_lossless,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map_next,
    clippy::large_digit_groups,
    clippy::manual_filter_map,
    clippy::manual_find_map,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms,
    unstable_name_collisions
)]

extern crate itertools;
extern crate rand;
extern crate rand_chacha;
extern crate sha3;

#[cfg(feature = "test_build")]
extern crate clap;
#[cfg(feature = "test_build")]
extern crate gnuplot;
#[cfg(feature = "test_build")]
extern crate time;
#[cfg(feature = "test_build")]
extern crate walkdir;

/// Contains the `Named` trait, for getting a type's name.
#[macro_use]
pub mod named;

#[doc(hidden)]
#[macro_use]
pub mod macros;

/// Functions for working with `bool`s.z
#[macro_use]
pub mod bools;
/// Functions for working with `char`s.
#[macro_use]
pub mod chars;
/// Macros and traits related to comparing values.
pub mod comparison;
/// Functions and adaptors for iterators.
pub mod iterators;
/// Contains `Never`, a type that cannot be instantiated.
pub mod nevers;
/// Functions for working with primitive integers and floats.
#[macro_use]
pub mod num;
/// Functions for working with `Ordering`s.
pub mod options;
/// Functions for working with `Option`s.
pub mod orderings;
/// Functions for generating random values with `ChaCha20Rng`.
pub mod random;
/// Contains `RationalSequence`s.
///
/// A rational sequence is a sequence that is finite or eventually repeating, just like the digits
/// of a rational number.
pub mod rational_sequences;
/// Contains `RoundingMode`s.
pub mod rounding_modes;
/// Functions for working with `HashSet`s and `BTreeSet`s.
pub mod sets;
/// Functions for working with slices.
#[macro_use]
pub mod slices;
/// Functions for working with strings.
pub mod strings;
/// Functions for working with tuples.
pub mod tuples;
/// Contains unions (sum types).
///
/// Here are usage examples of the macro-generated functions:
///
/// # unwrap
/// ```
/// use malachite_base::unions::Union3;
///
/// let mut u: Union3<char, char, char>;
///
/// u = Union3::A('a');
/// assert_eq!(u.unwrap(), 'a');
///
/// u = Union3::B('b');
/// assert_eq!(u.unwrap(), 'b');
///
/// u = Union3::C('c');
/// assert_eq!(u.unwrap(), 'c');
/// ```
///
/// # fmt (Display)
/// ```
/// use malachite_base::unions::Union3;
///
/// let mut u: Union3<char, u32, bool>;
///
/// u = Union3::A('a');
/// assert_eq!(u.to_string(), "A(a)");
///
/// u = Union3::B(5);
/// assert_eq!(u.to_string(), "B(5)");
///
/// u = Union3::C(false);
/// assert_eq!(u.to_string(), "C(false)");
/// ```
///
/// # from_str
/// ```
/// use std::str::FromStr;
///
/// use malachite_base::unions::{Union3, UnionFromStrError};
///
/// let u3: Union3<bool, u32, char> = Union3::from_str("B(5)").unwrap();
/// assert_eq!(u3, Union3::B(5));
///
/// let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("xyz");
/// assert_eq!(result, Err(UnionFromStrError::Generic("xyz".to_string())));
///
/// let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("A(ab)");
/// if let Err(UnionFromStrError::Specific(Union3::A(_e))) = result {
/// } else {
///     panic!("wrong error variant")
/// }
/// ```
pub mod unions;
/// Functions for working with `Vec`s.
pub mod vecs;

#[cfg(feature = "test_build")]
pub mod test_util;
