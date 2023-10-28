/// Constants associated with [`bool`]s.
///
/// The constants [`MIN`](crate::comparison::traits::Min::MIN) and
/// [`MAX`](crate::comparison::traits::Max::MAX) are defined as for [`bool`]s as `false` and `true`,
/// respectively. The constant [`NAME`](crate::named::Named::NAME) is defined as "bool".
pub mod constants;
/// An iterator that generates [`bool`]s without repetition.
pub mod exhaustive;
/// The implementation of [`NotAssign`](crate::num::logic::traits::NotAssign) for [`bool`].
pub mod not_assign;
#[cfg(feature = "random")]
/// Iterators that generate [`bool`]s randomly.
pub mod random;
