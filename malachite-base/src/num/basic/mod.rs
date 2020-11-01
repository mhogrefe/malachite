/// This module defines `PrimitiveInt`.
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
/// This module defines `PrimitiveSigned`.
pub mod signeds;
/// This module defines traits for constants and the Iverson bracket.
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
/// This module defines `PrimitiveUnsigned`.
pub mod unsigneds;
