use std::iter::repeat;

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive, from_bits_desc_alt,
};
use rand::Rand;

use malachite_test::common::{
    test_properties_custom_limit, test_properties_no_special, LARGE_LIMIT, SMALL_LIMIT,
};
use malachite_test::inputs::base::{
    small_unsigneds, vecs_of_bool_var_2, vecs_of_bool_var_3, vecs_of_bool_var_4, vecs_of_bool_var_5,
};

fn from_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit(limit, vecs_of_bool_var_2::<T>, |bits| {
        let n = T::from_bits_asc(bits.iter().cloned());
        assert_eq!(
            from_bits_asc_unsigned_naive::<T, _>(bits.iter().cloned()),
            n
        );
        assert_eq!(from_bits_asc_alt::<T, _>(bits.iter().cloned()), n);
        let trailing_falses = bits.iter().rev().take_while(|&&bit| !bit).count();
        let trimmed_bits = bits[..bits.len() - trailing_falses].to_vec();
        assert_eq!(n.to_bits_asc(), trimmed_bits);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_asc(repeat(false).take(u)), T::ZERO);
    });
}

fn from_bits_asc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit(limit, vecs_of_bool_var_3::<T>, |bits| {
        let n = T::from_bits_asc(bits.iter().cloned());
        assert_eq!(from_bits_asc_signed_naive::<T, _>(bits.iter().cloned()), n);
        assert_eq!(from_bits_asc_alt::<T, _>(bits.iter().cloned()), n);
        let trimmed_bits = if bits.iter().all(|&bit| !bit) {
            Vec::new()
        } else {
            let sign_bits = if *bits.last().unwrap() {
                bits.iter().rev().take_while(|&&bit| bit).count()
            } else {
                bits.iter().rev().take_while(|&&bit| !bit).count()
            };
            bits[..bits.len() - sign_bits + 1].to_vec()
        };
        assert_eq!(n.to_bits_asc(), trimmed_bits);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_asc(repeat(false).take(u)), T::ZERO);
        assert_eq!(T::from_bits_asc(repeat(true).take(u + 1)), T::NEGATIVE_ONE);
    });
}

#[test]
fn from_bits_asc_properties() {
    from_bits_asc_properties_helper_unsigned::<u8>();
    from_bits_asc_properties_helper_unsigned::<u16>();
    from_bits_asc_properties_helper_unsigned::<u32>();
    from_bits_asc_properties_helper_unsigned::<u64>();
    from_bits_asc_properties_helper_unsigned::<usize>();
    from_bits_asc_properties_helper_signed::<i8>();
    from_bits_asc_properties_helper_signed::<i16>();
    from_bits_asc_properties_helper_signed::<i32>();
    from_bits_asc_properties_helper_signed::<i64>();
    from_bits_asc_properties_helper_signed::<isize>();
}

fn from_bits_desc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit(limit, vecs_of_bool_var_4::<T>, |bits| {
        let n = T::from_bits_desc(bits.iter().cloned());
        assert_eq!(from_bits_desc_alt::<T, _>(bits.iter().cloned()), n);
        let leading_falses = bits.iter().take_while(|&&bit| !bit).count();
        let trimmed_bits = bits[leading_falses..].to_vec();
        assert_eq!(n.to_bits_desc(), trimmed_bits);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_desc(repeat(false).take(u)), T::ZERO);
    });
}

fn from_bits_desc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit(limit, vecs_of_bool_var_5::<T>, |bits| {
        let n = T::from_bits_desc(bits.iter().cloned());
        assert_eq!(from_bits_desc_alt::<T, _>(bits.iter().cloned()), n);
        let trimmed_bits = if bits.iter().all(|&bit| !bit) {
            Vec::new()
        } else {
            let sign_bits = if bits[0] {
                bits.iter().take_while(|&&bit| bit).count()
            } else {
                bits.iter().take_while(|&&bit| !bit).count()
            };
            bits[sign_bits - 1..].to_vec()
        };
        assert_eq!(n.to_bits_desc(), trimmed_bits);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_desc(repeat(false).take(u)), T::ZERO);
        assert_eq!(T::from_bits_desc(repeat(true).take(u + 1)), T::NEGATIVE_ONE);
    });
}

#[test]
fn from_bits_desc_properties() {
    from_bits_desc_properties_helper_unsigned::<u8>();
    from_bits_desc_properties_helper_unsigned::<u16>();
    from_bits_desc_properties_helper_unsigned::<u32>();
    from_bits_desc_properties_helper_unsigned::<u64>();
    from_bits_desc_properties_helper_unsigned::<usize>();
    from_bits_desc_properties_helper_signed::<i8>();
    from_bits_desc_properties_helper_signed::<i16>();
    from_bits_desc_properties_helper_signed::<i32>();
    from_bits_desc_properties_helper_signed::<i64>();
    from_bits_desc_properties_helper_signed::<isize>();
}
