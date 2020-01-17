use std::{i16, i32, i64, i8, u16, u32, u64, u8};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn significant_bits_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds, |&n: &T| {
        let significant_bits = n.significant_bits();
        assert!(significant_bits <= u64::from(T::WIDTH));
        assert_eq!(significant_bits == 0, n == T::ZERO);
        if n != T::ZERO {
            assert_eq!(significant_bits, n.floor_log_two() + 1)
        }
    });
}

fn significant_bits_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds, |&n: &T| {
        let significant_bits = n.significant_bits();
        assert!(significant_bits <= u64::from(T::WIDTH));
        assert_eq!(significant_bits == 0, n == T::ZERO);
        assert_eq!(significant_bits == u64::from(T::WIDTH), n == T::MIN);
        assert_eq!(n.wrapping_neg().significant_bits(), significant_bits);
    });
}

#[test]
fn significant_bits_properties() {
    significant_bits_properties_helper_unsigned::<u8>();
    significant_bits_properties_helper_unsigned::<u16>();
    significant_bits_properties_helper_unsigned::<u32>();
    significant_bits_properties_helper_unsigned::<u64>();
    significant_bits_properties_helper_unsigned::<usize>();
    significant_bits_properties_helper_signed::<i8>();
    significant_bits_properties_helper_signed::<i16>();
    significant_bits_properties_helper_signed::<i32>();
    significant_bits_properties_helper_signed::<i64>();
    significant_bits_properties_helper_signed::<isize>();
}
