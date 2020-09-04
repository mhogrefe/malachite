use malachite_base::num::arithmetic::traits::ModPowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::common::integer_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::integer::integers;

macro_rules! unsigned_properties {
    ($t: ident) => {
        properties!($t);

        test_properties(integers, |x| {
            let result = $t::checked_from(x);
            assert_eq!($t::checked_from(x.clone()), result);
            if *x >= 0 && x.significant_bits() <= $t::WIDTH {
                assert_eq!(Integer::from(result.unwrap()), *x);
                assert_eq!(result, Some($t::wrapping_from(x)));
                assert_eq!(result, Some($t::exact_from(x)));
            } else {
                assert!(result.is_none());
            }
            assert_eq!(result.is_none(), $t::overflowing_from(x).1);

            let result = $t::wrapping_from(x);
            assert_eq!(result, $t::exact_from((&x).mod_power_of_two($t::WIDTH)));
        });
    };
}

macro_rules! signed_properties {
    ($t: ident) => {
        properties!($t);

        test_properties(integers, |x| {
            let result = $t::checked_from(x);
            assert_eq!($t::checked_from(x.clone()), result);
            //TODO if *x >= 0 && x.significant_bits() <= u64::from($t::WIDTH - 1) {
            //TODO     assert_eq!(Integer::from(result.unwrap()), *x);
            //TODO     assert_eq!(result, Some($t::wrapping_from(x)));
            //TODO     assert_eq!(result, Some($t::exact_from(x)));
            //TODO } else {
            //TODO     assert!(result.is_none());
            //TODO }
            assert_eq!(result.is_none(), $t::overflowing_from(x).1);
        });
    };
}

macro_rules! properties {
    ($t: ident) => {
        test_properties(integers, |x| {
            let result = $t::wrapping_from(x);
            assert_eq!($t::wrapping_from(x.clone()), result);
            assert_eq!(result, $t::overflowing_from(x).0);

            let result = $t::saturating_from(x);
            assert_eq!($t::saturating_from(x.clone()), result);
            //TODO assert!(result <= *x);
            //TODO assert_eq!(result == *x, $t::convertible_from(x));

            let result = $t::overflowing_from(x);
            assert_eq!($t::overflowing_from(x.clone()), result);
            assert_eq!(result, ($t::wrapping_from(x), !$t::convertible_from(x)));

            let convertible = $t::convertible_from(x.clone());
            assert_eq!($t::convertible_from(x), convertible);
            //TODO assert_eq!(convertible, *x >= $t::MIN && *x <= $t::MAX);
        });
    };
}

#[test]
fn primitive_int_from_integer_properties() {
    test_properties(integers, |x| {
        assert_eq!(integer_to_rug_integer(x).to_u32(), u32::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_u32_wrapping(),
            u32::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_u64(), u64::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_u64_wrapping(),
            u64::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_i32(), i32::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_i32_wrapping(),
            i32::wrapping_from(x)
        );
        assert_eq!(integer_to_rug_integer(x).to_i64(), i64::checked_from(x));
        assert_eq!(
            integer_to_rug_integer(x).to_i64_wrapping(),
            i64::wrapping_from(x)
        );
    });

    unsigned_properties!(u8);
    unsigned_properties!(u16);
    unsigned_properties!(u32);
    unsigned_properties!(u64);
    unsigned_properties!(usize);

    signed_properties!(i8);
    signed_properties!(i16);
    signed_properties!(i32);
    signed_properties!(i64);
    signed_properties!(isize);
}
