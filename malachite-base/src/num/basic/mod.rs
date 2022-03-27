// TODO
pub mod floats;
/// The trait for primitive integers.
///
/// Here are usage examples of the associated constants:
///
/// ```
/// use malachite_base::num::basic::integers::PrimitiveInt;
///
/// assert_eq!(u32::WIDTH, 32);
/// assert_eq!(u32::LOG_WIDTH, 5);
/// assert_eq!(u32::WIDTH_MASK, 0x1f);
/// ```
pub mod integers;
/// The trait for primitive signed integers.
pub mod signeds;
/// Traits for constants and the Iverson bracket.
///
/// Here are usage examples of the Iverson bracket:
///
/// ```
/// use malachite_base::num::basic::traits::Iverson;
///
/// assert_eq!(u32::iverson(false), 0);
/// assert_eq!(i8::iverson(true), 1);
/// ```
pub mod traits;
/// The trait for primitive unsigned integers.
pub mod unsigneds;
