use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::next_power_of_2::{
    limbs_next_power_of_2, limbs_slice_next_power_of_2_in_place, limbs_vec_next_power_of_2_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_next_power_of_2_and_limbs_vec_next_power_of_2_in_place() {
    let test = |xs: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_next_power_of_2(xs), out);

        let mut xs = xs.to_vec();
        limbs_vec_next_power_of_2_in_place(&mut xs);
        assert_eq!(xs, out);
    };
    test(&[3], &[4]);
    test(&[6, 7], &[0, 8]);
    test(&[100, 101, 102], &[0, 0, 128]);
    test(&[123, 456], &[0, 512]);
    test(&[123, 456, u32::MAX], &[0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_next_power_of_2_fail() {
    limbs_next_power_of_2(&[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_next_power_of_2_in_place_fail() {
    limbs_slice_next_power_of_2_in_place(&mut []);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_next_power_of_2_in_place_fail() {
    limbs_vec_next_power_of_2_in_place(&mut Vec::new());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_next_power_of_2_in_place() {
    let test = |xs: &[Limb], carry: bool, out: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_slice_next_power_of_2_in_place(&mut xs), carry);
        assert_eq!(xs, out);
    };
    test(&[3], false, &[4]);
    test(&[6, 7], false, &[0, 8]);
    test(&[100, 101, 102], false, &[0, 0, 128]);
    test(&[123, 456], false, &[0, 512]);
    test(&[123, 456, u32::MAX], true, &[0, 0, 0]);
}

#[test]
fn test_next_power_of_2() {
    let test = |u, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.next_power_of_2_assign();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).next_power_of_2();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap().next_power_of_two();
        assert_eq!(n.to_string(), out);
    };
    test("0", "1");
    test("1", "1");
    test("2", "2");
    test("3", "4");
    test("4", "4");
    test("5", "8");
    test("6", "8");
    test("7", "8");
    test("8", "8");
    test("9", "16");
    test("10", "16");
    test("123", "128");
    test("1000", "1024");
    test("1000000", "1048576");
    test("1000000000", "1073741824");
    test("1000000000000", "1099511627776");
    test("1073741823", "1073741824");
    test("1073741824", "1073741824");
    test("1073741825", "2147483648");
    test("2147483647", "2147483648");
    test("2147483648", "2147483648");
    test("2147483649", "4294967296");
    test("21344980687", "34359738368");
}
