pub mod digits;
pub mod floating_point_from_natural;
pub mod from_floating_point;
pub mod from_limbs;
pub mod from_primitive_int;
/// This module implements a trait for determining whether a `Natural` is an integer. (It always
/// is.)
pub mod is_integer;
pub mod limb_count;
/// This implements traits for converting numbers to and from mantissa and exponent
/// representations.
///
/// Here are some examples of the macro-generated functions:
///
/// # sci_mantissa_and_exponent
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_nz::natural::Natural;
///
/// let test = |n: Natural, mantissa: f32, exponent: u64| {
///     let (m, e) = n.sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(m), NiceFloat(mantissa));
///     assert_eq!(e, exponent);
/// };
/// test(Natural::from(3u32), 1.5, 1);
/// test(Natural::from(123u32), 1.921875, 6);
/// test(Natural::from(1000000000u32), 1.8626451, 29);
/// test(Natural::from(10u32).pow(52), 1.670478, 172);
/// ```
///
/// # from_sci_mantissa_and_exponent
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// let test = |mantissa: f32, exponent: u64, out: Option<Natural>| {
///     assert_eq!(
///         <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
///             mantissa, exponent
///         ),
///         out
///     );
/// };
/// test(1.5, 1, Some(Natural::from(3u32)));
/// test(1.51, 1, Some(Natural::from(3u32)));
/// test(1.921875, 6, Some(Natural::from(123u32)));
/// test(
///     1.670478,
///     172,
///     Some(Natural::from_str("10000000254586612611935772707803116801852191350456320").unwrap()),
/// );
///
/// test(2.0, 1, None);
/// test(10.0, 1, None);
/// test(0.5, 1, None);
/// ```
pub mod mantissa_and_exponent;
pub mod primitive_int_from_natural;
pub mod string;
pub mod to_limbs;
