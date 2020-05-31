use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitBlockAccess, LowMask, SignificantBits};
use malachite_base_test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_nz::natural::logic::bit_block_access::{limbs_slice_get_bits, limbs_vec_get_bits};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_3, triples_of_unsigned_small_unsigned_and_small_unsigned_var_1,
    triples_of_unsigned_vec_small_unsigned_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_unsigned, triples_of_natural_small_unsigned_and_small_unsigned_var_1,
};

fn verify_limbs_get_bits(limbs: &[Limb], start: u64, end: u64, out: &[Limb]) {
    let n = Natural::from_limbs_asc(limbs);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Natural, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_get_bits() {
    let test = |limbs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_slice_get_bits(limbs, start, end), out);
        verify_limbs_get_bits(limbs, start, end, out);
    };
    // limb_start >= len
    test(&[], 10, 20, &[]);
    // limb_start < len
    // limb_end >= len
    // offset != 0
    test(&[0x1234_5678, 0xabcd_ef01], 16, 48, &[0xef01_1234]);
    // limb_end < len
    test(&[0x1234_5678, 0xabcd_ef01], 4, 16, &[0x567]);
    // offset == 0
    test(
        &[0x1234_5678, 0xabcd_ef01],
        0,
        100,
        &[0x1234_5678, 0xabcd_ef01],
    );
    test(&[0x1234_5678, 0xabcd_ef01], 10, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_slice_get_bits_fail() {
    limbs_slice_get_bits(&[123], 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_get_bits() {
    let test = |limbs: Vec<Limb>, start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_vec_get_bits(limbs, start, end), out);
    };
    test(vec![], 10, 20, &[]);
    test(vec![0x1234_5678, 0xabcd_ef01], 16, 48, &[0xef01_1234, 0]);
    test(vec![0x1234_5678, 0xabcd_ef01], 4, 16, &[0x567]);
    test(
        vec![0x1234_5678, 0xabcd_ef01],
        0,
        100,
        &[0x1234_5678, 0xabcd_ef01],
    );
    test(vec![0x1234_5678, 0xabcd_ef01], 10, 10, &[0]);
}

#[test]
#[should_panic]
fn limbs_vec_get_bits_fail() {
    limbs_vec_get_bits(vec![123], 10, 5);
}

#[test]
fn test_get_bits() {
    let test = |n, start, end, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().get_bits(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            Natural::from_str(n).unwrap().get_bits_owned(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            get_bits_naive::<Natural, Natural>(&Natural::from_str(n).unwrap(), start, end),
            Natural::from_str(out).unwrap()
        );
    };
    test("12379813738590787192", 16, 48, "4009824820");
    test("12379813738590787192", 4, 16, "1383");
    test("12379813738590787192", 0, 100, "12379813738590787192");
    test("12379813738590787192", 10, 10, "0");
}

#[test]
#[should_panic]
fn get_bits_fail() {
    Natural::from(123u32).get_bits(10, 5);
}

#[test]
#[should_panic]
fn get_bits_owned_fail() {
    Natural::from(123u32).get_bits_owned(10, 5);
}

#[test]
fn limbs_get_bits_properties() {
    test_properties(
        triples_of_unsigned_vec_small_unsigned_and_small_unsigned_var_1,
        |&(ref limbs, start, end)| {
            let out = limbs_slice_get_bits(limbs, start, end);
            verify_limbs_get_bits(limbs, start, end, &out);
            let out = limbs_vec_get_bits(limbs.to_vec(), start, end);
            verify_limbs_get_bits(limbs, start, end, &out);
        },
    );
}

#[test]
fn get_bits_properties() {
    test_properties(
        triples_of_natural_small_unsigned_and_small_unsigned_var_1,
        |&(ref n, start, end)| {
            let bits = n.get_bits(start, end);
            assert_eq!(n.clone().get_bits_owned(start, end), bits);
            assert_eq!(get_bits_naive::<Natural, Natural>(n, start, end), bits);
            assert!(bits <= *n);
            let significant_bits = n.significant_bits();
            assert_eq!(
                n.get_bits(start + significant_bits, end + significant_bits),
                0
            );
            assert_eq!(
                (!n).get_bits(start, end),
                Natural::low_mask(end - start) - &bits
            );
            let mut mut_n = n.clone();
            mut_n.assign_bits(start, end, &bits);
            assert_eq!(*n, mut_n);
        },
    );

    test_properties(pairs_of_natural_and_unsigned, |&(ref n, start)| {
        assert_eq!(n.get_bits(start, start), 0);
    });

    test_properties(pairs_of_unsigneds_var_3, |&(start, end)| {
        assert_eq!(Natural::ZERO.get_bits(start, end), 0);
    });

    test_properties(
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_1::<Limb, u64>,
        |&(n, start, end)| {
            assert_eq!(
                Natural::from(n).get_bits(start, end),
                n.get_bits(start, end)
            );
        },
    );
}
