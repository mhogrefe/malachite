#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
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
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(
    clippy::cognitive_complexity,
    clippy::many_single_char_names,
    clippy::too_many_arguments,
    unstable_name_collisions
)]

extern crate itertools;
extern crate rand;
extern crate rand_chacha;
extern crate sha3;

/// This module contains the `Named` trait, for getting a type's name.
#[macro_use]
pub mod named;

/// This module contains functions for working with `bool`s.
#[macro_use]
pub mod bools;
/// This module contains functions for working with `char`s.
#[macro_use]
pub mod chars;
/// This module contains macros and traits related to comparing values.
pub mod comparison;
/// This module contains functions and adaptors for iterators.
pub mod iterators;
/// This module contains `Never`, a type that cannot be instantiated.
pub mod nevers;
/// This module contains functions for working with primitive integers and floats.
#[macro_use]
pub mod num;
/// This module contains functions for working with `Ordering`s.
pub mod options;
/// This module contains functions for working with `Option`s.
pub mod orderings;
/// This module contains functions for generating random values with `ChaCha20Rng`.
pub mod random;
/// This module contains the `RoundingMode` enum.
///
/// A `RoundingMode` can often be specified when a function conceptually returns a value of one
/// type, but must be rounded to another type. The most common case is a conceptually real-valued
/// function whose result must be rounded to an integer, like `div_round`.
///
/// When converting a real value to an integer, the different `RoundingMode`s act as follows:
/// - `Floor` applies the floor function: $x \mapsto \lfloor x \rfloor$. In other words, the value
///   is rounded towards $-\infty$.
/// - `Ceiling` applies the ceiling function: $x \mapsto \lceil x \rceil$. In other words, the value
///   is rounded towards $\infty$.
/// - `Down` applies the function $x \mapsto \operatorname{sgn}(x) \lfloor |x| \rfloor$. In other
///   words, the value is rounded towards $0$.
/// - `Up` applies the function $x \mapsto \operatorname{sgn}(x) \lceil |x| \rceil$. In other words,
///   the value is rounded away from $0$.
/// - `Nearest` applies the function
///   $$
///     x \mapsto \\begin{cases}
///         \lfloor x \rfloor & x - \lfloor x \rfloor < \frac{1}{2} \\\\
///         \lceil x \rceil & x - \lfloor x \rfloor > \frac{1}{2} \\\\
///         \lfloor x \rfloor &
///    x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is even} \\\\
///         \lceil x \rceil &
///    x - \lfloor x \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor x \rfloor \\ \text{is odd.}
///     \\end{cases}
///   $$
///   In other words, it rounds to the nearest integer, and when there's a tie, it rounds to the
///   nearest even integer. This is also called _bankers' rounding_ and is often used as a default.
/// - `Exact` panics if the value is not already rounded.
///
/// # Examples
///
/// Here are some examples of how floating-point values would be rounded to integer values using the
/// different `RoundingMode`s.
///
/// | x    | `Floor` | `Ceiling` | `Down` | `Up` | `Nearest` | `Exact`    |
/// |------|---------|-----------|--------|------|-----------|------------|
/// |  3.0 |       3 |         3 |      3 |    3 |         3 |          3 |
/// |  3.2 |       3 |         4 |      3 |    4 |         3 | `panic!()` |
/// |  3.8 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  3.5 |       3 |         4 |      3 |    4 |         4 | `panic!()` |
/// |  4.5 |       4 |         5 |      4 |    5 |         4 | `panic!()` |
/// | -3.2 |      -4 |        -3 |     -3 |   -4 |        -3 | `panic!()` |
/// | -3.8 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -3.5 |      -4 |        -3 |     -3 |   -4 |        -4 | `panic!()` |
/// | -4.5 |      -5 |        -4 |     -4 |   -5 |        -4 | `panic!()` |
///
/// Sometimes a `RoundingMode` is used in an unusual context, such as rounding an integer to a
/// floating-point number, in which case further explanation of its behavior is provided at the
/// usage site.
pub mod rounding_modes;
/// This module contains functions for working with slices.
#[macro_use]
pub mod slices;
pub mod strings;
/// This module contains functions for working with tuples.
pub mod tuples;
pub mod vecs;
