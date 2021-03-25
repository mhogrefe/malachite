use malachite_base::num::logic::traits::BitAccess;
use std::str::FromStr;
#[cfg(feature = "32_bit_limbs")]
use std::u32;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;
use malachite_nz::integer::Integer;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_set_bit_neg() {
    let test = |xs: &[u32], index: u64, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        limbs_set_bit_neg(&mut mut_xs, index);
        assert_eq!(mut_xs, out);
    };
    test(&[3, 2, 1], 100, &[3, 2, 1]);
    test(&[0, 0, 0b1101, 0b11], 96, &[0, 0, 0b1101, 0b10]);
    test(&[0, 0, 0b1101, 0b11], 66, &[0, 0, 0b1001, 0b11]);
    test(&[0, 0, 0b1100, 0b11], 64, &[0, 0, 0b1011, 0b11]);
    test(&[0, 0, 0b1101, 0b11], 32, &[0, u32::MAX, 0b1100, 0b11]);
}

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
    test("-1", 5, "-1");
    test("-1", 100, "-1");
    test("-33", 5, "-1");
    test("-1267650600228229401496703205377", 100, "-1");
    test("-32", 0, "-31");
    test("-1000000000000", 10, "-999999998976");
    test("-1000000000000", 100, "-1000000000000");
    test("-1267650600228229402496703205376", 100, "-1000000000000");
    test("-18446744078004518912", 0, "-18446744078004518911");
    test("-18446744078004518912", 32, "-18446744078004518912");
    test("-18446744078004518912", 33, "-18446744078004518912");
    test("-18446744078004518912", 64, "-4294967296");
    test("-18446744078004518912", 65, "-18446744078004518912");
    test("-4294967296", 0, "-4294967295");
}
