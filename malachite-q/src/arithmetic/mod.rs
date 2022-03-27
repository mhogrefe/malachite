pub mod abs;
pub mod add;
pub mod approximate;
pub mod ceiling;
pub mod div;
pub mod floor;
pub mod is_power_of_2;
pub mod log_base;
pub mod log_base_2;
pub mod log_base_power_of_2;
pub mod mul;
pub mod neg;
pub mod next_power_of_2;
pub mod pow;
pub mod power_of_2;
pub mod reciprocal;
pub mod root;
pub mod round_to_multiple;
pub mod round_to_multiple_of_power_of_2;
/// Traits for shifting a `Rational` left (multiplying it by a power of 2).
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational << PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::ZERO << 10u8, 0);
/// assert_eq!(Rational::from(123) << 2u16, 492);
/// assert_eq!((Rational::from_signeds(7, 22) << 2u16).to_string(), "14/11");
///
/// assert_eq!(Rational::ZERO << 10i8, 0);
/// assert_eq!(Rational::from(123) << 2i16, 492);
/// assert_eq!((Rational::from(123) << -2i16).to_string(), "123/4");
/// assert_eq!((Rational::from_signeds(7, 22) << 2i16).to_string(), "14/11");
/// assert_eq!((Rational::from_signeds(22, 7) << -2i16).to_string(), "11/14");
/// ```
///
/// # &Rational << PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert_eq!(&Rational::ZERO << 10u8, 0);
/// assert_eq!(&Rational::from(123) << 2u16, 492);
/// assert_eq!((&Rational::from_signeds(7, 22) << 2u16).to_string(), "14/11");
///
/// assert_eq!(&Rational::ZERO << 10i8, 0);
/// assert_eq!(&Rational::from(123) << 2i16, 492);
/// assert_eq!((&Rational::from(123) << -2i16).to_string(), "123/4");
/// assert_eq!((&Rational::from_signeds(7, 22) << 2i16).to_string(), "14/11");
/// assert_eq!((&Rational::from_signeds(22, 7) << -2i16).to_string(), "11/14");
/// ```
///
/// # Rational <<= PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// let mut x = Rational::ZERO;
/// x <<= 10u8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x <<= 2u16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from_signeds(7, 22);
/// x <<= 2u16;
/// assert_eq!(x.to_string(), "14/11");
///
/// let mut x = Rational::ZERO;
/// x <<= 10i8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x <<= 2i16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from(123);
/// x <<= -2i16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from_signeds(7, 22);
/// x <<= 2i16;
/// assert_eq!(x.to_string(), "14/11");
///
/// let mut x = Rational::from_signeds(22, 7);
/// x <<= -2i16;
/// assert_eq!(x.to_string(), "11/14");
/// ```
pub mod shl;
/// Traits for shifting a `Rational` right (dividing it by a power of 2).
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational >> PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::ZERO >> 10u8, 0);
/// assert_eq!((Rational::from(123) >> 2u16).to_string(), "123/4");
/// assert_eq!((Rational::from_signeds(22, 7) >> 2u16).to_string(), "11/14");
///
/// assert_eq!(Rational::ZERO >> 10i8, 0);
/// assert_eq!((Rational::from(123) >> 2i16).to_string(), "123/4");
/// assert_eq!(Rational::from(123) >> -2i16, 492);
/// assert_eq!((Rational::from_signeds(22, 7) >> 2i16).to_string(), "11/14");
/// assert_eq!((Rational::from_signeds(7, 22) >> -2i16).to_string(), "14/11");
/// ```
///
/// # &Rational >> PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// assert_eq!(&Rational::ZERO >> 10u8, 0);
/// assert_eq!((&Rational::from(123) >> 2u16).to_string(), "123/4");
/// assert_eq!((&Rational::from_signeds(22, 7) >> 2u16).to_string(), "11/14");
///
/// assert_eq!(&Rational::ZERO >> 10i8, 0);
/// assert_eq!((&Rational::from(123) >> 2i16).to_string(), "123/4");
/// assert_eq!(&Rational::from(123) >> -2i16, 492);
/// assert_eq!((&Rational::from_signeds(22, 7) >> 2i16).to_string(), "11/14");
/// assert_eq!((&Rational::from_signeds(7, 22) >> -2i16).to_string(), "14/11");
/// ```
///
/// # Rational >>= PrimitiveInt
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::Rational;
///
/// let mut x = Rational::ZERO;
/// x >>= 10u8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x >>= 2u16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from_signeds(22, 7);
/// x >>= 2u16;
/// assert_eq!(x.to_string(), "11/14");
///
/// let mut x = Rational::ZERO;
/// x >>= 10i8;
/// assert_eq!(x, 0);
///
/// let mut x = Rational::from(123);
/// x >>= 2i16;
/// assert_eq!(x.to_string(), "123/4");
///
/// let mut x = Rational::from(123);
/// x >>= -2i16;
/// assert_eq!(x, 492);
///
/// let mut x = Rational::from_signeds(22, 7);
/// x >>= 2i16;
/// assert_eq!(x.to_string(), "11/14");
///
/// let mut x = Rational::from_signeds(7, 22);
/// x >>= -2i16;
/// assert_eq!(x.to_string(), "14/11");
/// ```
pub mod shr;
pub mod sign;
pub mod simplest_rational_in_interval;
pub mod sqrt;
pub mod square;
pub mod sub;
pub mod traits;
