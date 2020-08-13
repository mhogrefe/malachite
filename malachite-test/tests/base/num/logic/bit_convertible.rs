use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_asc_signed_naive, from_bits_asc_unsigned_naive,
    from_bits_desc_alt, from_bits_desc_signed_naive, from_bits_desc_unsigned_naive,
    to_bits_asc_alt, to_bits_asc_signed_naive, to_bits_asc_unsigned_naive, to_bits_desc_alt,
    to_bits_desc_signed_naive, to_bits_desc_unsigned_naive,
};
use rand::Rand;

use malachite_test::common::{
    test_properties, test_properties_custom_limit, test_properties_no_special, LARGE_LIMIT,
    SMALL_LIMIT,
};
use malachite_test::inputs::base::{
    signeds, small_unsigneds, unsigneds, vecs_of_bool_var_2, vecs_of_bool_var_3,
    vecs_of_bool_var_4, vecs_of_bool_var_5,
};

fn to_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(unsigneds::<T>, |&u| {
        let bits = u.to_bits_asc();
        assert_eq!(to_bits_asc_unsigned_naive(u), bits);
        assert_eq!(to_bits_asc_alt(&u), bits);
        assert_eq!(u.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().cloned().rev().collect::<Vec<bool>>(),
            u.to_bits_desc()
        );
        assert_eq!(T::from_bits_asc(&bits), u);
        if u != T::ZERO {
            assert_eq!(*bits.last().unwrap(), true);
        }
        assert_eq!(bits.len(), usize::exact_from(u.significant_bits()));
    });
}

fn to_bits_asc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&i| {
        let bits = i.to_bits_asc();
        assert_eq!(to_bits_asc_signed_naive(i), bits);
        assert_eq!(to_bits_asc_alt(&i), bits);
        assert_eq!(i.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().cloned().rev().collect::<Vec<bool>>(),
            i.to_bits_desc()
        );
        assert_eq!(T::from_bits_asc(&bits), i);
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
        assert_eq!(to_bits_desc_unsigned_naive(u), bits);
        assert_eq!(to_bits_desc_alt(&u), bits);
        assert_eq!(u.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().cloned().rev().collect::<Vec<bool>>(),
            u.to_bits_asc()
        );
        assert_eq!(T::from_bits_desc(&bits), u);
        if u != T::ZERO {
            assert_eq!(bits[0], true);
        }
        assert_eq!(bits.len(), usize::exact_from(u.significant_bits()));
    });
}

fn to_bits_desc_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(signeds::<T>, |&i| {
        let bits = i.to_bits_desc();
        assert_eq!(to_bits_desc_signed_naive(i), bits);
        assert_eq!(to_bits_desc_alt(&i), bits);
        assert_eq!(i.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(
            bits.iter().cloned().rev().collect::<Vec<bool>>(),
            i.to_bits_asc()
        );
        assert_eq!(T::from_bits_desc(&bits), i);
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

fn from_bits_asc_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    let limit = if T::WIDTH == u8::WIDTH {
        SMALL_LIMIT
    } else {
        LARGE_LIMIT
    };
    test_properties_custom_limit(limit, vecs_of_bool_var_2::<T>, |bits| {
        let n = T::from_bits_asc(bits);
        assert_eq!(from_bits_asc_unsigned_naive::<T>(bits), n);
        assert_eq!(from_bits_asc_alt::<T>(bits), n);
        let trailing_falses = bits.iter().rev().take_while(|&&bit| !bit).count();
        let trimmed_bits = bits[..bits.len() - trailing_falses].to_vec();
        assert_eq!(n.to_bits_asc(), trimmed_bits);
        assert_eq!(T::from_bit_iterator_asc(bits.iter().cloned()), n);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_asc(&vec![false; u]), T::ZERO);
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
        let n = T::from_bits_asc(bits);
        assert_eq!(from_bits_asc_signed_naive::<T>(bits), n);
        assert_eq!(from_bits_asc_alt::<T>(bits), n);
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
        assert_eq!(T::from_bit_iterator_asc(bits.iter().cloned()), n);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_asc(&vec![false; u]), T::ZERO);
        assert_eq!(T::from_bits_asc(&vec![true; u + 1]), T::NEGATIVE_ONE);
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
        let n = T::from_bits_desc(bits);
        assert_eq!(from_bits_desc_unsigned_naive::<T>(bits), n);
        assert_eq!(from_bits_desc_alt::<T>(bits), n);
        let leading_falses = bits.iter().take_while(|&&bit| !bit).count();
        let trimmed_bits = bits[leading_falses..].to_vec();
        assert_eq!(n.to_bits_desc(), trimmed_bits);
        assert_eq!(T::from_bit_iterator_desc(bits.iter().cloned()), n);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_desc(&vec![false; u]), T::ZERO);
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
        let n = T::from_bits_desc(bits);
        assert_eq!(from_bits_desc_signed_naive::<T>(bits), n);
        assert_eq!(from_bits_desc_alt::<T>(bits), n);
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
        assert_eq!(T::from_bit_iterator_desc(bits.iter().cloned()), n);
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::from_bits_desc(&vec![false; u]), T::ZERO);
        assert_eq!(T::from_bits_desc(&vec![true; u + 1]), T::NEGATIVE_ONE);
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
