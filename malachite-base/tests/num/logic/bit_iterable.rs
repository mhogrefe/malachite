use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitIterable;
use malachite_base::test_util::generators::{
    signed_bool_vec_pair_gen_var_1, signed_gen, signed_unsigned_pair_gen_var_1,
    unsigned_bool_vec_pair_gen_var_1, unsigned_gen, unsigned_gen_var_5, unsigned_pair_gen_var_2,
};
use std::ops::Index;

#[test]
pub fn test_bits() {
    let mut bits = 105u8.bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], false);
    assert_eq!(bits[2], false);
    assert_eq!(bits[3], true);
    assert_eq!(bits[4], false);
    assert_eq!(bits[5], true);
    assert_eq!(bits[6], true);
    assert_eq!(bits[7], false);
    assert_eq!(bits[8], false);

    let mut bits = 105u32.bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    let mut bits = (-105i8).bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], true);
    assert_eq!(bits[2], true);
    assert_eq!(bits[3], false);
    assert_eq!(bits[4], true);
    assert_eq!(bits[5], false);
    assert_eq!(bits[6], false);
    assert_eq!(bits[7], true);
    assert_eq!(bits[8], true);

    let mut bits = (-105i32).bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);
}

fn bits_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    T::BitIterator: Index<u64, Output = bool>,
{
    unsigned_gen::<T>().test_properties(|n| {
        let significant_bits = usize::exact_from(n.significant_bits());
        assert_eq!(
            n.bits().size_hint(),
            (significant_bits, Some(significant_bits))
        );
    });

    unsigned_bool_vec_pair_gen_var_1::<T>().test_properties(|(n, bs)| {
        let mut bits = n.bits();
        let mut bit_vec = Vec::new();
        let mut i = 0;
        for b in bs {
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
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], false);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

fn bits_properties_helper_signed<T: PrimitiveSigned>()
where
    T::BitIterator: Index<u64, Output = bool>,
{
    signed_gen::<T>().test_properties(|i| {
        i.bits();
    });

    signed_bool_vec_pair_gen_var_1::<T>().test_properties(|(n, bs)| {
        let mut bits = n.bits();
        let mut bit_vec = Vec::new();
        let mut i = 0;
        for b in bs {
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
    });

    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], n < T::ZERO);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(T::ZERO.bits()[u], false);
    });
}

#[test]
fn bits_properties() {
    apply_fn_to_unsigneds!(bits_properties_helper_unsigned);
    apply_fn_to_signeds!(bits_properties_helper_signed);
}
