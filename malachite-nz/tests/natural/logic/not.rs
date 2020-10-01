use std::str::FromStr;

use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place, limbs_not_to_out};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_and_limbs_not_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_not(xs), out);

        let mut mut_xs = xs.to_vec();
        limbs_not_in_place(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[0, 1, 2], &[u32::MAX, u32::MAX - 1, u32::MAX - 2]);
    test(&[u32::MAX, u32::MAX - 1, u32::MAX - 2], &[0, 1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_to_out() {
    let test = |xs: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut mut_out = out_before.to_vec();
        limbs_not_to_out(&mut mut_out, xs);
        assert_eq!(mut_out, out_after);
    };
    test(&[], &[], &[]);
    test(&[0x11111111], &[5], &[0xeeeeeeee]);
    test(
        &[0xffff0000, 0xf0f0f0f0],
        &[0, 1, 2],
        &[0xffff, 0xf0f0f0f, 2],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_not_to_out_fail() {
    let mut out = vec![1, 2];
    limbs_not_to_out(&mut out, &[1, 2, 3]);
}

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}
