use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::arithmetic::eq_mod::limbs_eq_neg_limb_mod_limb;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_test::inputs::base::triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_integers, triples_of_integer_integer_and_natural,
    triples_of_integer_integer_and_natural_var_1, triples_of_integer_integer_and_natural_var_2,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_neg_limb_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_neg_limb_mod_limb(limbs, limb, modulus), equal);
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 6, 13, true);
    test(&[100, 101, 102], 1_232, 10, true);
    test(&[100, 101, 102], 1_233, 10, false);
    test(&[123, 456], 153, 789, true);
    test(&[123, 456], 1_000, 789, false);
    test(&[0xffff_ffff, 0xffff_ffff], 101, 2, true);
    test(&[0xffff_ffff, 0xffff_ffff], 100, 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 111, 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 110, 3, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_neg_limb_mod_limb_fail() {
    limbs_eq_neg_limb_mod_limb(&[10], 10, 15);
}

#[test]
fn test_eq_mod() {
    let test = |x, y, modulus, out| {
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );

        assert_eq!(
            Integer::from_str(y).unwrap().eq_mod(
                Integer::from_str(x).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x).unwrap().is_congruent(
                &rug::Integer::from_str(y).unwrap(),
                &rug::Integer::from_str(modulus).unwrap()
            ),
            out
        );
    };
    test("0", "0", "0", true);
    test("0", "1", "0", false);
    test("57", "57", "0", true);
    test("57", "58", "0", false);
    test("1000000000000", "57", "0", false);
    test("0", "256", "256", true);
    test("0", "256", "512", false);
    test("13", "23", "10", true);
    test("13", "24", "10", false);
    test("13", "21", "1", true);
    test("13", "21", "2", true);
    test("13", "21", "4", true);
    test("13", "21", "8", true);
    test("13", "21", "16", false);
    test("13", "21", "3", false);
    test("1000000000001", "1", "4096", true);
    test("1000000000001", "1", "8192", false);
    test("12345678987654321", "321", "1000", true);
    test("12345678987654321", "322", "1000", false);
    test("1234", "1234", "1000000000000", true);
    test("1234", "1235", "1000000000000", false);
    test("1000000001234", "1000000002234", "1000", true);
    test("1000000001234", "1000000002235", "1000", false);
    test("1000000001234", "1234", "1000000000000", true);
    test("1000000001234", "1235", "1000000000000", false);
    test("1000000001234", "5000000001234", "1000000000000", true);
    test("1000000001234", "5000000001235", "1000000000000", false);

    test("0", "-1", "0", false);
    test("57", "-57", "0", false);
    test("57", "-58", "0", false);
    test("1000000000000", "-57", "0", false);
    test("0", "-256", "256", true);
    test("0", "-256", "512", false);
    test("13", "-27", "10", true);
    test("13", "-28", "10", false);
    test("29", "-27", "1", true);
    test("29", "-27", "2", true);
    test("29", "-27", "4", true);
    test("29", "-27", "8", true);
    test("29", "-27", "16", false);
    test("29", "-27", "3", false);
    test("999999999999", "-1", "4096", true);
    test("999999999999", "-1", "8192", false);
    test("12345678987654321", "-679", "1000", true);
    test("12345678987654321", "-680", "1000", false);
    test("1000000001234", "-999999999766", "1000", true);
    test("1000000001234", "-999999999767", "1000", false);
    test("1000000001234", "-999999998766", "1000000000000", true);
    test("1000000001234", "-999999998767", "1000000000000", false);

    test("-1", "0", "0", false);
    test("-57", "57", "0", false);
    test("-57", "58", "0", false);
    test("-1000000000000", "57", "0", false);
    test("-256", "0", "256", true);
    test("-256", "0", "512", false);
    test("-13", "27", "10", true);
    test("-13", "28", "10", false);
    test("-29", "27", "1", true);
    test("-29", "27", "2", true);
    test("-29", "27", "4", true);
    test("-29", "27", "8", true);
    test("-29", "27", "16", false);
    test("-29", "27", "3", false);
    test("-999999999999", "1", "4096", true);
    test("-999999999999", "1", "8192", false);
    test("-12345678987654321", "679", "1000", true);
    test("-12345678987654321", "680", "1000", false);
    test("-1000000001234", "999999999766", "1000", true);
    test("-1000000001234", "999999999767", "1000", false);
    test("-1000000001234", "999999998766", "1000000000000", true);
    test("-1000000001234", "999999998767", "1000000000000", false);

    test("-57", "-57", "0", true);
    test("-57", "-58", "0", false);
    test("-1000000000000", "-57", "0", false);
    test("-13", "-23", "10", true);
    test("-13", "-24", "10", false);
    test("-13", "-21", "1", true);
    test("-13", "-21", "2", true);
    test("-13", "-21", "4", true);
    test("-13", "-21", "8", true);
    test("-13", "-21", "16", false);
    test("-13", "-21", "3", false);
    test("-1000000000001", "-1", "4096", true);
    test("-1000000000001", "-1", "8192", false);
    test("-12345678987654321", "-321", "1000", true);
    test("-12345678987654321", "-322", "1000", false);
    test("-1234", "-1234", "1000000000000", true);
    test("-1234", "-1235", "1000000000000", false);
    test("-1000000001234", "-1000000002234", "1000", true);
    test("-1000000001234", "-1000000002235", "1000", false);
    test("-1000000001234", "-1234", "1000000000000", true);
    test("-1000000001234", "-1235", "1000000000000", false);
    test("-1000000001234", "-5000000001234", "1000000000000", true);
    test("-1000000001234", "-5000000001235", "1000000000000", false);
}

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_neg_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(
                (-Natural::from_limbs_asc(limbs))
                    .eq_mod(Integer::from(limb), Natural::from(modulus)),
                equal
            );
        },
    );
}

#[test]
fn eq_mod_properties() {
    test_properties(
        triples_of_integer_integer_and_natural,
        |&(ref x, ref y, ref modulus)| {
            let equal = x.eq_mod(y, modulus);
            assert_eq!(y.eq_mod(x, modulus), equal);

            assert_eq!(x.eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y, modulus), equal);
            assert_eq!(x.clone().eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus.clone()), equal);

            assert_eq!((-x).eq_mod(-y, modulus), equal);
            assert_eq!((x - y).divisible_by(Integer::from(modulus)), equal);
            assert_eq!((y - x).divisible_by(Integer::from(modulus)), equal);
            assert_eq!(
                integer_to_rug_integer(x)
                    .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)),
                equal
            );
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_1,
        |&(ref x, ref y, ref modulus)| {
            assert!(x.eq_mod(y, modulus));
            assert!(y.eq_mod(x, modulus));
            assert!(integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_2,
        |&(ref x, ref y, ref modulus)| {
            assert!(!x.eq_mod(y, modulus));
            assert!(!y.eq_mod(x, modulus));
            assert!(!integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        assert!(x.eq_mod(y, Natural::ONE));
        assert_eq!(x.eq_mod(y, Natural::ZERO), x == y);
    });

    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        assert_eq!(x.eq_mod(Integer::ZERO, y), x.divisible_by(Integer::from(y)));
        assert!(x.eq_mod(x, y));
    });
}
