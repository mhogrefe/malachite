#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::logic::traits::BitAccess;
#[cfg(feature = "32_bit_limbs")]
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::bit_access::limbs_clear_bit;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_clear_bit() {
    let test = |xs: &[Limb], index: u64, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_clear_bit(&mut xs, index);
        assert_eq!(xs, out);
    };
    test(&[3, 3], 33, &[3, 1]);
    test(&[3, 1], 1, &[1, 1]);
    test(&[3, 3], 100, &[3, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_clear_bit() {
    let test = |u, index, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.clear_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::exact_from(index), false);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "0");
    test("0", 100, "0");
    test("1024", 10, "0");
    test("101", 0, "100");
    test("1000000001024", 10, "1000000000000");
    test("1000000001024", 100, "1000000001024");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("1267650600228229401496703205381", 100, "5");
}
