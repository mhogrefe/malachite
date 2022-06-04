use std::str::FromStr;

/// Converts a string to an `Option<T>`, where `T` implements [`FromStr`].
///
/// If the string does not represent a valid `Option<T>`, `None` is returned.
///
/// If `T` does not implement [`FromStr`], try using [`option_from_str_custom`] instead.
///
/// # Examples
/// ```
/// use malachite_base::options::option_from_str;
///
/// assert_eq!(option_from_str::<bool>("Some(false)"), Some(Some(false)));
/// assert_eq!(option_from_str::<u32>("Some(5)"), Some(Some(5)));
/// assert_eq!(option_from_str::<u32>("None"), Some(None));
/// assert_eq!(option_from_str::<u32>("Some(hi)"), None);
/// assert_eq!(option_from_str::<bool>("abc"), None);
/// ```
#[inline]
pub fn option_from_str<T: FromStr>(src: &str) -> Option<Option<T>> {
    option_from_str_custom(&(|t| t.parse().ok()), src)
}

/// Converts a string to an `Option<T>`, given a function to parse a string into a `T`.
///
/// If the string does not represent a valid `Option<T>`, `None` is returned.
///
/// If `f` just uses the [`FromStr`] implementation on `T`, you can use [`option_from_str`]
/// instead.
///
/// # Examples
/// ```
/// use malachite_base::options::{option_from_str, option_from_str_custom};
/// use malachite_base::orderings::ordering_from_str;
/// use std::cmp::Ordering;
///
/// assert_eq!(
///     option_from_str_custom::<Ordering>(&ordering_from_str, "Some(Less)"),
///     Some(Some(Ordering::Less))
/// );
/// assert_eq!(
///     option_from_str_custom::<Option<bool>>(&option_from_str, "Some(Some(false))"),
///     Some(Some(Some(false)))
/// );
/// assert_eq!(
///     option_from_str_custom::<Ordering>(&ordering_from_str, "Some(hi)"),
///     None
/// );
/// assert_eq!(
///     option_from_str_custom::<Ordering>(&ordering_from_str, "abc"),
///     None
/// );
/// ```
pub fn option_from_str_custom<T>(f: &dyn Fn(&str) -> Option<T>, src: &str) -> Option<Option<T>> {
    if src == "None" {
        Some(None)
    } else if src.starts_with("Some(") && src.ends_with(')') {
        f(&src[5..src.len() - 1]).map(Some)
    } else {
        None
    }
}

/// Iterators that generate [`Option`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`Option`]s randomly.
pub mod random;
