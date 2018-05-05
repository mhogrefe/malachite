use common::test_properties;
use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{nonempty_vecs_of_unsigned, vecs_of_u32_var_1};
use malachite_test::inputs::integer::integers;
use malachite_test::integer::logic::checked_count_zeros::{
    integer_checked_count_zeros_alt_1, integer_checked_count_zeros_alt_2,
};
use std::str::FromStr;

#[test]
fn test_limbs_count_zeros_neg() {
    let test = |limbs, out| {
        assert_eq!(limbs_count_zeros_neg(limbs), out);
    };
    test(&[0, 1, 2], 33);
    test(&[1, 0xffff_ffff], 32);
}

#[test]
fn test_checked_count_zeros() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().checked_count_zeros(), out);
    };
    test("0", None);
    test("105", None);
    test("-105", Some(3));
    test("1000000000000", None);
    test("-1000000000000", Some(24));
    test("4294967295", None);
    test("-4294967295", Some(31));
    test("4294967296", None);
    test("-4294967296", Some(32));
    test("18446744073709551615", None);
    test("-18446744073709551615", Some(63));
    test("18446744073709551616", None);
    test("-18446744073709551616", Some(64));
}

#[test]
fn limbs_count_zeros_neg_properties() {
    test_properties(nonempty_vecs_of_unsigned, |limbs| {
        limbs_count_zeros_neg(limbs);
    });

    test_properties(vecs_of_u32_var_1, |limbs| {
        assert_eq!(
            Some(limbs_count_zeros_neg(limbs)),
            (-Natural::from_limbs_asc(limbs)).checked_count_zeros()
        );
    });
}

#[test]
fn checked_count_zeros_properties() {
    test_properties(integers, |x| {
        let zeros = x.checked_count_zeros();
        assert_eq!(integer_checked_count_zeros_alt_1(x), zeros);
        assert_eq!(integer_checked_count_zeros_alt_2(x), zeros);
        assert_eq!((!x).checked_count_ones(), zeros);
    });
}
