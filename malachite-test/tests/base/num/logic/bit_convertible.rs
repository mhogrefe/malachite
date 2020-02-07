use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::integers::_to_bits_desc_alt;
use malachite_base::num::logic::signeds::{_to_bits_asc_signed_naive, _to_bits_desc_signed_naive};
use malachite_base::num::logic::unsigneds::{
    _to_bits_asc_unsigned_naive, _to_bits_desc_unsigned_naive,
};
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, unsigneds};

fn to_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&u| {
        let bits = u.to_bits_asc();
        assert_eq!(_to_bits_asc_unsigned_naive(u), bits);
        if u != T::ZERO {
            assert_eq!(*bits.last().unwrap(), true);
        }
        assert!(bits.len() <= usize::exact_from(T::WIDTH));
    });
}

fn to_bits_asc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&i| {
        let bits = i.to_bits_asc();
        assert_eq!(_to_bits_asc_signed_naive(i), bits, "{}", i);
        if i != T::ZERO {
            assert_eq!(*bits.last().unwrap(), i < T::ZERO);
        }
        let bit_len = bits.len();
        assert!(bit_len <= usize::exact_from(T::WIDTH));
        if bit_len > 1 {
            assert_ne!(bits[bit_len - 1], bits[bit_len - 2]);
        }
    });
}

#[test]
fn to_bits_asc_properties() {
    to_bits_asc_properties_helper_unsigned::<u8>();
    to_bits_asc_properties_helper_unsigned::<u16>();
    to_bits_asc_properties_helper_unsigned::<u32>();
    to_bits_asc_properties_helper_unsigned::<u64>();
    to_bits_asc_properties_helper_unsigned::<usize>();
    to_bits_asc_properties_helper_signed::<i8>();
    to_bits_asc_properties_helper_signed::<i16>();
    to_bits_asc_properties_helper_signed::<i32>();
    to_bits_asc_properties_helper_signed::<i64>();
    to_bits_asc_properties_helper_signed::<isize>();
}

fn to_bits_desc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&u| {
        let bits = u.to_bits_desc();
        assert_eq!(_to_bits_desc_unsigned_naive(u), bits);
        assert_eq!(_to_bits_desc_alt(&u), bits);
        if u != T::ZERO {
            assert_eq!(bits[0], true);
        }
        assert!(bits.len() <= usize::exact_from(T::WIDTH));
    });
}

fn to_bits_desc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&i| {
        let bits = i.to_bits_desc();
        assert_eq!(_to_bits_desc_signed_naive(i), bits);
        assert_eq!(_to_bits_desc_alt(&i), bits);
        if i != T::ZERO {
            assert_eq!(bits[0], i < T::ZERO);
        }
        let bit_len = bits.len();
        assert!(bit_len <= usize::exact_from(T::WIDTH));
        if bit_len > 1 {
            assert_ne!(bits[0], bits[1]);
        }
    });
}

#[test]
fn to_bits_desc_properties() {
    to_bits_desc_properties_helper_unsigned::<u8>();
    to_bits_desc_properties_helper_unsigned::<u16>();
    to_bits_desc_properties_helper_unsigned::<u32>();
    to_bits_desc_properties_helper_unsigned::<u64>();
    to_bits_desc_properties_helper_unsigned::<usize>();
    to_bits_desc_properties_helper_signed::<i8>();
    to_bits_desc_properties_helper_signed::<i16>();
    to_bits_desc_properties_helper_signed::<i32>();
    to_bits_desc_properties_helper_signed::<i64>();
    to_bits_desc_properties_helper_signed::<isize>();
}
