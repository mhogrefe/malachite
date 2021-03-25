use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_nz::integer::logic::bit_block_access::{
    limbs_neg_limb_get_bits, limbs_slice_neg_get_bits, limbs_vec_neg_get_bits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_neg_limb_get_bits(x: Limb, start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from(x);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_neg_get_bits(xs: &[Limb], start: u64, end: u64, out: &[Limb]) {
    let n = -Natural::from_limbs_asc(xs);
    let result = n.get_bits(start, end);
    assert_eq!(get_bits_naive::<Integer, Natural>(&n, start, end), result);
    assert_eq!(Natural::from_limbs_asc(out), result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_limb_get_bits() {
    let test = |x: Limb, start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_neg_limb_get_bits(x, start, end), out);
        verify_limbs_neg_limb_get_bits(x, start, end, out);
    };
    // trailing_zeros < end
    // start >= Limb::WIDTH
    test(1, 40, 50, &[0x3ff]);
    // start < Limb::WIDTH
    // trailing_zeros < start
    test(0x12345678, 16, 48, &[0xffffedcb]);
    test(0x12345678, 4, 16, &[0xa98]);
    // trailing_zeros >= start
    test(0x12345678, 0, 100, &[0xedcba988, u32::MAX, u32::MAX, 0xf]);
    test(0x12345678, 10, 10, &[]);
    // trailing_zeros >= end
    test(0x80000000, 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_neg_limb_get_bits_fail() {
    limbs_neg_limb_get_bits(123, 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_neg_get_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_slice_neg_get_bits(xs, start, end), out);
        verify_limbs_neg_get_bits(xs, start, end, out);
    };
    // trailing_zeros < end
    // limb_start >= len
    test(&[1], 40, 50, &[0x3ff]);
    // limb_start < len
    // limb_end >= len
    // offset != 0
    // trailing_zeros < start
    test(&[0x12345678, 0xabcdef01], 16, 48, &[0x10feedcb]);
    // limb_end < len
    test(&[0x12345678, 0xabcdef01], 4, 16, &[0xa98]);
    // offset == 0
    // trailing_zeros >= start
    test(
        &[0x12345678, 0xabcdef01],
        0,
        100,
        &[0xedcba988, 0x543210fe, u32::MAX, 0xf],
    );
    test(&[0x12345678, 0xabcdef01], 10, 10, &[]);
    // trailing_zeros >= end
    test(&[0, 0x80000000], 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_slice_neg_get_bits_fail() {
    limbs_slice_neg_get_bits(&[123], 10, 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_neg_get_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, out: &[Limb]| {
        assert_eq!(limbs_vec_neg_get_bits(xs.to_vec(), start, end), out);
        verify_limbs_neg_get_bits(xs, start, end, out);
    };
    test(&[1], 40, 50, &[0x3ff]);
    test(&[0x12345678, 0xabcdef01], 16, 48, &[0x10feedcb]);
    test(&[0x12345678, 0xabcdef01], 4, 16, &[0xa98]);
    test(
        &[0x12345678, 0xabcdef01],
        0,
        100,
        &[0xedcba988, 0x543210fe, u32::MAX, 0xf],
    );
    test(&[0x12345678, 0xabcdef01], 10, 10, &[]);
    test(&[0, 0x80000000], 5, 10, &[]);
}

#[test]
#[should_panic]
fn limbs_vec_neg_get_bits_fail() {
    limbs_vec_neg_get_bits(vec![123], 10, 5);
}

#[test]
fn test_get_bits() {
    let test = |n, start, end, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().get_bits(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            Integer::from_str(n).unwrap().get_bits_owned(start, end),
            Natural::from_str(out).unwrap()
        );
        assert_eq!(
            get_bits_naive::<Integer, Natural>(&Integer::from_str(n).unwrap(), start, end),
            Natural::from_str(out).unwrap()
        );
    };
    test("12379813738590787192", 16, 48, "4009824820");
    test("12379813738590787192", 4, 16, "1383");
    test("12379813738590787192", 0, 100, "12379813738590787192");
    test("12379813738590787192", 10, 10, "0");
    test("-12379813738590787192", 16, 48, "285142475");
    test("-12379813738590787192", 4, 16, "2712");
    test(
        "-12379813738590787192",
        0,
        100,
        "1267650600215849587758112418184",
    );
    test("-12379813738590787192", 10, 10, "0");
}

#[test]
#[should_panic]
fn get_bits_fail() {
    Integer::from(123).get_bits(10, 5);
}

#[test]
#[should_panic]
fn get_bits_owned_fail() {
    Integer::from(123).get_bits_owned(10, 5);
}
