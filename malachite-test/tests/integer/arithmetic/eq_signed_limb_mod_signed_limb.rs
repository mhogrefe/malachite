use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, Mod};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use rug;

use common::test_properties;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{pairs_of_signeds, triples_of_signeds};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, triples_of_integer_signed_and_signed,
    triples_of_integer_signed_and_signed_var_1, triples_of_integer_signed_and_signed_var_2,
    triples_of_signed_signed_and_integer,
};
use malachite_test::integer::arithmetic::eq_signed_limb_mod_signed_limb::*;

#[test]
fn test_eq_signed_limb_mod_signed_limb() {
    let test = |n, i: SignedLimb, modulus, out| {
        assert_eq!(Integer::from_str(n).unwrap().eq_mod(i, modulus), out);
        assert_eq!(i.eq_mod(&Integer::from_str(n).unwrap(), modulus), out);
        assert_eq!(
            rug_eq_signed_limb_mod_signed_limb(rug::Integer::from_str(n).unwrap(), i, modulus),
            out
        );
    };
    test("0", 0, 0, true);
    test("0", 1, 0, false);
    test("57", 57, 0, true);
    test("57", 58, 0, false);
    test("1000000000000", 57, 0, false);
    test("0", 256, 256, true);
    test("0", 256, 512, false);
    test("13", 23, 10, true);
    test("13", 24, 10, false);
    test("13", 21, 1, true);
    test("13", 21, 2, true);
    test("13", 21, 4, true);
    test("13", 21, 8, true);
    test("13", 21, 16, false);
    test("13", 21, 3, false);
    test("1000000000001", 1, 4_096, true);
    test("1000000000001", 1, 8_192, false);
    test("12345678987654321", 321, 1_000, true);
    test("12345678987654321", 322, 1_000, false);

    test("-1", 1, 2, true);
    test("-1", 2, 2, false);
    test("-57", 57, 0, false);
    test("-1000000000000", 57, 0, false);
    test("-13", 27, 10, true);
    test("-13", 28, 10, false);
    test("-13", 11, 1, true);
    test("-13", 11, 2, true);
    test("-13", 11, 4, true);
    test("-13", 11, 8, true);
    test("-13", 11, 16, false);
    test("-13", 11, 3, true);
    test("-13", 10, 3, false);
    test("-1000000000001", 8_191, 4_096, true);
    test("-1000000000001", 8_191, 8_192, false);
    test("-12345678987654321", 679, 1_000, true);
    test("-12345678987654321", 680, 1_000, false);

    test("0", -1, 0, false);
    test("1", -1, 2, true);
    test("1", -2, 2, false);
    test("57", -57, 0, false);
    test("1000000000000", -57, 0, false);
    test("0", -256, 256, true);
    test("0", -256, 512, false);
    test("13", -27, 10, true);
    test("13", -28, 10, false);
    test("13", -11, 1, true);
    test("13", -11, 2, true);
    test("13", -11, 4, true);
    test("13", -11, 8, true);
    test("13", -11, 16, false);
    test("13", -11, 3, true);
    test("13", -10, 3, false);
    test("1000000000001", -8_191, 4_096, true);
    test("1000000000001", -8_191, 8_192, false);
    test("12345678987654321", -679, 1_000, true);
    test("12345678987654321", -680, 1_000, false);

    test("-57", -57, 0, true);
    test("-57", -58, 0, false);
    test("-1000000000000", -57, 0, false);
    test("-13", -23, 10, true);
    test("-13", -24, 10, false);
    test("-13", -21, 1, true);
    test("-13", -21, 2, true);
    test("-13", -21, 4, true);
    test("-13", -21, 8, true);
    test("-13", -21, 16, false);
    test("-13", -21, 3, false);
    test("-1000000000001", -1, 4_096, true);
    test("-1000000000001", -1, 8_192, false);
    test("-12345678987654321", -321, 1_000, true);
    test("-12345678987654321", -322, 1_000, false);

    test("0", 256, -256, true);
    test("0", 256, -512, false);
    test("13", 23, -10, true);
    test("13", 24, -10, false);
    test("13", 21, -1, true);
    test("13", 21, -2, true);
    test("13", 21, -4, true);
    test("13", 21, -8, true);
    test("13", 21, -16, false);
    test("13", 21, -3, false);
    test("1000000000001", 1, -4_096, true);
    test("1000000000001", 1, -8_192, false);
    test("12345678987654321", 321, -1_000, true);
    test("12345678987654321", 322, -1_000, false);

    test("-1", 1, -2, true);
    test("-1", 2, -2, false);
    test("-13", 27, -10, true);
    test("-13", 28, -10, false);
    test("-13", 11, -1, true);
    test("-13", 11, -2, true);
    test("-13", 11, -4, true);
    test("-13", 11, -8, true);
    test("-13", 11, -16, false);
    test("-13", 11, -3, true);
    test("-13", 10, -3, false);
    test("-1000000000001", 8_191, -4_096, true);
    test("-1000000000001", 8_191, -8_192, false);
    test("-12345678987654321", 679, -1_000, true);
    test("-12345678987654321", 680, -1_000, false);

    test("1", -1, -2, true);
    test("1", -2, -2, false);
    test("0", -256, -256, true);
    test("0", -256, -512, false);
    test("13", -27, -10, true);
    test("13", -28, -10, false);
    test("13", -11, -1, true);
    test("13", -11, -2, true);
    test("13", -11, -4, true);
    test("13", -11, -8, true);
    test("13", -11, -16, false);
    test("13", -11, -3, true);
    test("13", -10, -3, false);
    test("1000000000001", -8_191, -4_096, true);
    test("1000000000001", -8_191, -8_192, false);
    test("12345678987654321", -679, -1_000, true);
    test("12345678987654321", -680, -1_000, false);

    test("-13", -23, -10, true);
    test("-13", -24, -10, false);
    test("-13", -21, -1, true);
    test("-13", -21, -2, true);
    test("-13", -21, -4, true);
    test("-13", -21, -8, true);
    test("-13", -21, -16, false);
    test("-13", -21, -3, false);
    test("-1000000000001", -1, -4_096, true);
    test("-1000000000001", -1, -8_192, false);
    test("-12345678987654321", -321, -1_000, true);
    test("-12345678987654321", -322, -1_000, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_signed_limb_eq_signed_limb_mod_integer() {
    let test = |i: i32, j, modulus, out| {
        assert_eq!(i.eq_mod(j, &Integer::from_str(modulus).unwrap()), out);
    };
    test(0, 0, "0", true);
    test(0, 1, "0", false);
    test(57, 57, "0", true);
    test(57, 58, "0", false);
    test(57, 57, "1000000000000", true);
    test(57, 58, "1000000000000", false);
    test(0, 256, "256", true);
    test(0, 256, "512", false);
    test(13, 23, "10", true);
    test(13, 24, "10", false);
    test(13, 21, "1", true);
    test(13, 21, "2", true);
    test(13, 21, "4", true);
    test(13, 21, "8", true);
    test(13, 21, "16", false);
    test(13, 21, "3", false);

    test(-1, 1, "2", true);
    test(-1, 2, "2", false);
    test(-57, 57, "0", false);
    test(-57, 57, "1000000000000", false);
    test(-57, 58, "1000000000000", false);
    test(-13, 27, "10", true);
    test(-13, 28, "10", false);
    test(-13, 11, "1", true);
    test(-13, 11, "2", true);
    test(-13, 11, "4", true);
    test(-13, 11, "8", true);
    test(-13, 11, "16", false);
    test(-13, 11, "3", true);
    test(-13, 10, "3", false);

    test(0, -1, "0", false);
    test(1, -1, "2", true);
    test(1, -2, "2", false);
    test(57, -57, "0", false);
    test(57, -57, "1000000000000", false);
    test(57, -58, "1000000000000", false);
    test(0, -256, "256", true);
    test(0, -256, "512", false);
    test(13, -27, "10", true);
    test(13, -28, "10", false);
    test(13, -11, "1", true);
    test(13, -11, "2", true);
    test(13, -11, "4", true);
    test(13, -11, "8", true);
    test(13, -11, "16", false);
    test(13, -11, "3", true);
    test(13, -10, "3", false);

    test(-57, -57, "0", true);
    test(-57, -58, "0", false);
    test(-57, -57, "1000000000000", true);
    test(-57, -58, "1000000000000", false);
    test(-13, -23, "10", true);
    test(-13, -24, "10", false);
    test(-13, -21, "1", true);
    test(-13, -21, "2", true);
    test(-13, -21, "4", true);
    test(-13, -21, "8", true);
    test(-13, -21, "16", false);
    test(-13, -21, "3", false);

    test(0, 256, "-256", true);
    test(0, 256, "-512", false);
    test(57, 57, "-1000000000000", true);
    test(57, 58, "-1000000000000", false);
    test(13, 23, "-10", true);
    test(13, 24, "-10", false);
    test(13, 21, "-1", true);
    test(13, 21, "-2", true);
    test(13, 21, "-4", true);
    test(13, 21, "-8", true);
    test(13, 21, "-16", false);
    test(13, 21, "-3", false);

    test(-1, 1, "-2", true);
    test(-1, 2, "-2", false);
    test(-57, 57, "-1000000000000", false);
    test(-57, 58, "-1000000000000", false);
    test(-13, 27, "-10", true);
    test(-13, 28, "-10", false);
    test(-13, 11, "-1", true);
    test(-13, 11, "-2", true);
    test(-13, 11, "-4", true);
    test(-13, 11, "-8", true);
    test(-13, 11, "-16", false);
    test(-13, 11, "-3", true);
    test(-13, 10, "-3", false);

    test(1, -1, "-2", true);
    test(1, -2, "-2", false);
    test(57, -57, "-1000000000000", false);
    test(57, -58, "-1000000000000", false);
    test(0, -256, "-256", true);
    test(0, -256, "-512", false);
    test(13, -27, "-10", true);
    test(13, -28, "-10", false);
    test(13, -11, "-1", true);
    test(13, -11, "-2", true);
    test(13, -11, "-4", true);
    test(13, -11, "-8", true);
    test(13, -11, "-16", false);
    test(13, -11, "-3", true);
    test(13, -10, "-3", false);

    test(-13, -23, "-10", true);
    test(-13, -24, "-10", false);
    test(-57, -57, "-1000000000000", true);
    test(-57, -58, "-1000000000000", false);
    test(-13, -21, "-1", true);
    test(-13, -21, "-2", true);
    test(-13, -21, "-4", true);
    test(-13, -21, "-8", true);
    test(-13, -21, "-16", false);
    test(-13, -21, "-3", false);
}

#[test]
fn eq_signed_limb_mod_signed_limb_properties() {
    test_properties(
        triples_of_integer_signed_and_signed,
        |&(ref n, i, modulus): &(Integer, SignedLimb, SignedLimb)| {
            let equal = n.eq_mod(i, modulus);
            assert_eq!(i.eq_mod(n, modulus), equal);
            assert_eq!(
                *n == i || modulus != 0 && n.mod_op(modulus) == i.mod_op(modulus),
                equal
            );
            assert_eq!((n - i).divisible_by(modulus), equal);
            assert_eq!((i - n).divisible_by(modulus), equal);

            //TODO assert_eq!(n.eq_mod(Integer::from(i), modulus), equal);

            assert_eq!(
                rug_eq_signed_limb_mod_signed_limb(integer_to_rug_integer(n), i, modulus),
                equal
            );
        },
    );

    test_properties(
        triples_of_integer_signed_and_signed_var_1,
        |&(ref n, i, modulus): &(Integer, SignedLimb, SignedLimb)| {
            assert!(n.eq_mod(i, modulus));
            assert!(i.eq_mod(n, modulus));
            assert!(*n == i || modulus != 0 && n.mod_op(modulus) == i.mod_op(modulus));
            assert!((n - i).divisible_by(modulus));
            assert!((i - n).divisible_by(modulus));

            //TODO assert!(n.eq_mod(Integer::from(i), modulus));

            assert!(rug_eq_signed_limb_mod_signed_limb(
                integer_to_rug_integer(n),
                i,
                modulus,
            ));
        },
    );

    test_properties(
        triples_of_integer_signed_and_signed_var_2,
        |&(ref n, i, modulus): &(Integer, SignedLimb, SignedLimb)| {
            assert!(!n.eq_mod(i, modulus));
            assert!(!i.eq_mod(n, modulus));
            assert!(*n != i && (modulus == 0 || n.mod_op(modulus) != i.mod_op(modulus)));
            assert!(!(n - i).divisible_by(modulus));
            assert!(!(i - n).divisible_by(modulus));

            //TODO assert!(!n.eq_mod(Integer::from(i), modulus));

            assert!(!rug_eq_signed_limb_mod_signed_limb(
                integer_to_rug_integer(n),
                i,
                modulus,
            ));
        },
    );

    test_properties(pairs_of_integer_and_signed, |&(ref n, i)| {
        assert!(n.eq_mod(i, 1 as SignedLimb));
        assert!(i.eq_mod(n, 1 as SignedLimb));
        assert_eq!(n.eq_mod(0 as SignedLimb, i), n.divisible_by(i));
        assert_eq!((0 as SignedLimb).eq_mod(n, i), n.divisible_by(i));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, modulus)| {
        assert!(Integer::from(i).eq_mod(i, modulus));
        assert!(i.eq_mod(&Integer::from(i), modulus));
        assert_eq!(Integer::ZERO.eq_mod(i, modulus), i.divisible_by(modulus));
        assert_eq!(i.eq_mod(&Integer::ZERO, modulus), i.divisible_by(modulus));
    });

    test_properties(triples_of_signeds::<SignedLimb>, |&(n, i, modulus)| {
        assert_eq!(n.eq_mod(i, modulus), Integer::from(n).eq_mod(i, modulus));
    });
}

#[test]
fn signed_limb_eq_signed_limb_mod_integer_properties() {
    test_properties(
        triples_of_signed_signed_and_integer,
        |&(i, j, ref modulus): &(SignedLimb, SignedLimb, Integer)| {
            let equal = i.eq_mod(j, modulus);
            assert_eq!(j.eq_mod(i, modulus), equal);
            assert_eq!(
                i == j || *modulus != 0 as Limb && i.mod_op(modulus) == j.mod_op(modulus),
                equal
            );

            //TODO assert_eq!(Integer::from(i).eq_mod(j, modulus), equal);
        },
    );

    test_properties(pairs_of_integer_and_signed::<SignedLimb>, |&(ref n, i)| {
        assert_eq!(i.eq_mod(0, n), i.divisible_by(n));
        assert_eq!(0.eq_mod(i, n), i.divisible_by(n));
        assert!(i.eq_mod(i, n));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        assert!(i.eq_mod(j, &Integer::ONE));
        assert!(j.eq_mod(i, &Integer::ONE));
        assert!(i.eq_mod(j, &Integer::NEGATIVE_ONE));
        assert!(j.eq_mod(i, &Integer::NEGATIVE_ONE));
        assert_eq!(i.eq_mod(j, &Integer::ZERO), i == j);
        assert_eq!(j.eq_mod(i, &Integer::ZERO), i == j);
    });

    test_properties(triples_of_signeds::<SignedLimb>, |&(i, j, modulus)| {
        let equal = i.eq_mod(j, modulus);
        assert_eq!(EqMod::eq_mod(i, j, &Integer::from(modulus)), equal);
    });
}
