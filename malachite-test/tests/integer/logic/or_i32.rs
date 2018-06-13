use common::test_properties;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::or_i32::{limbs_neg_or_neg_limb, limbs_pos_or_neg_limb};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_u32_vec_and_positive_u32_var_1, signeds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::logic::or_i32::{integer_or_i32_alt_1, integer_or_i32_alt_2};
use rug::{self, Assign};
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_pos_or_neg_limb() {
    let test = |limbs: &[u32], u: u32, out: u32| {
        assert_eq!(limbs_pos_or_neg_limb(limbs, u), out);
    };
    test(&[6, 7], 3, 0xffff_fff9);
    test(&[100, 101, 102], 10, 0xffff_ff92);
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_pos_or_neg_limb_fail() {
    limbs_pos_or_neg_limb(&[], 10);
}

#[test]
fn test_limbs_neg_or_neg_limb() {
    let test = |limbs: &[u32], u: u32, out: u32| {
        assert_eq!(limbs_neg_or_neg_limb(limbs, u), out);
    };
    test(&[6, 7], 3, 5);
    test(&[100, 101, 102], 10, 98);
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_neg_or_neg_limb_fail() {
    limbs_neg_or_neg_limb(&[], 10);
}

#[test]
fn test_or_i32() {
    let test = |u, v: i32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);

        let n = v | Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v | &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v | rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v | &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "507");
    test("999999999999", 123, "999999999999");
    test("1000000000000", 123, "1000000000123");
    test("1000000000001", 123, "1000000000123");
    test("12345678987654321", 0, "12345678987654321");
    test("12345678987654321", 456, "12345678987654649");
    test("12345678987654321", 987_654_321, "12345679395421361");

    test("0", -123, "-123");
    test("123", -456, "-389");
    test("999999999999", -123, "-1");
    test("1000000000000", -123, "-123");
    test("1000000000001", -123, "-123");
    test("12345678987654321", -456, "-327");
    test("12345678987654321", -987_654_321, "-407767041");

    test("-123", 0, "-123");
    test("-123", 456, "-51");
    test("-999999999999", 123, "-999999999877");
    test("-1000000000000", 123, "-999999999877");
    test("-1000000000001", 123, "-1000000000001");
    test("-12345678987654321", 0, "-12345678987654321");
    test("-12345678987654321", 456, "-12345678987654193");
    test("-12345678987654321", 987_654_321, "-12345678407767041");

    test("-123", -456, "-67");
    test("-999999999999", -123, "-123");
    test("-1000000000000", -123, "-123");
    test("-1000000000001", -123, "-1");
    test("-12345678987654321", -456, "-129");
    test("-12345678987654321", -987_654_321, "-579887281");
}

#[test]
fn limbs_pos_or_neg_limb_properties() {
    test_properties(
        pairs_of_u32_vec_and_positive_u32_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_pos_or_neg_limb(limbs, u),
                -(Integer::from(Natural::from_limbs_asc(limbs))
                    | Integer::from_owned_twos_complement_limbs_asc(vec![u, u32::MAX]))
            );
        },
    );
}

#[test]
fn limbs_neg_or_neg_limb_properties() {
    test_properties(
        pairs_of_u32_vec_and_positive_u32_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_neg_or_neg_limb(limbs, u),
                -(-Natural::from_limbs_asc(limbs)
                    | Integer::from_owned_twos_complement_limbs_asc(vec![u, u32::MAX]))
            );
        },
    );
}

#[test]
fn or_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let mut mut_n = n.clone();
            mut_n |= i;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n |= i;
            assert_eq!(rug_integer_to_integer(&rug_n), result);

            let result_alt = n | i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = n.clone() | i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i | n;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i | n.clone();
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(integer_or_i32_alt_1(&n, i), result);
            assert_eq!(integer_or_i32_alt_2(&n, i), result);

            assert_eq!(n | Integer::from(i), result);
            assert_eq!(Integer::from(i) | n, result);

            assert_eq!(&result | i, result);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) | i)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n | 0, *n);
        assert_eq!(0 | n, *n);
        assert_eq!(n | -1, -1);
        assert_eq!(-1 | n, -1);
    });

    test_properties(signeds, |&i: &i32| {
        assert_eq!(&Integer::ZERO | i, i);
        assert_eq!(i | &Integer::ZERO, i);
        assert_eq!(&Integer::NEGATIVE_ONE | i, -1);
        assert_eq!(i | &Integer::NEGATIVE_ONE, -1);
    });
}
