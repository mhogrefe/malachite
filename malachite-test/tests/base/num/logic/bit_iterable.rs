use std::ops::Index;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitIterable;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_signed_and_vec_of_bool_var_2,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_and_vec_of_bool_var_2, small_unsigneds,
    unsigneds,
};

fn bits_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>()
where
    <T as BitIterable>::BitIterator: Index<u64, Output = bool>,
{
    test_properties(unsigneds::<T>, |n| {
        let significant_bits = usize::exact_from(n.significant_bits());
        assert_eq!(
            n.bits().size_hint(),
            (significant_bits, Some(significant_bits))
        );
    });

    test_properties(
        pairs_of_unsigned_and_vec_of_bool_var_2::<T>,
        |&(ref n, ref bs)| {
            let mut bits = n.bits();
            let mut bit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    bit_vec.insert(i, bits.next().unwrap());
                    i += 1;
                } else {
                    bit_vec.insert(i, bits.next_back().unwrap())
                }
            }
            assert!(bits.next().is_none());
            assert!(bits.next_back().is_none());
            assert_eq!(n.to_bits_asc(), bit_vec);
        },
    );

    test_properties(pairs_of_unsigned_and_small_unsigned::<T, u64>, |&(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], false);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

fn bits_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as BitIterable>::BitIterator: Index<u64, Output = bool>,
{
    test_properties(
        pairs_of_signed_and_vec_of_bool_var_2::<T>,
        |&(ref n, ref bs)| {
            let mut bits = n.bits();
            let mut bit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    bit_vec.insert(i, bits.next().unwrap());
                    i += 1;
                } else {
                    bit_vec.insert(i, bits.next_back().unwrap())
                }
            }
            assert!(bits.next().is_none());
            assert!(bits.next_back().is_none());
            assert_eq!(n.to_bits_asc(), bit_vec);
        },
    );

    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], n < T::ZERO);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

#[test]
fn bits_properties() {
    bits_properties_helper_unsigned::<u8>();
    bits_properties_helper_unsigned::<u16>();
    bits_properties_helper_unsigned::<u32>();
    bits_properties_helper_unsigned::<u64>();
    bits_properties_helper_unsigned::<usize>();
    bits_properties_helper_signed::<i8>();
    bits_properties_helper_signed::<i16>();
    bits_properties_helper_signed::<i32>();
    bits_properties_helper_signed::<i64>();
    bits_properties_helper_signed::<isize>();
}
