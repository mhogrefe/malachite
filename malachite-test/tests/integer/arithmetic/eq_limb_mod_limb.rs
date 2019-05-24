use common::test_properties;
use malachite_base::num::traits::{DivisibleBy, EqMod, Mod, NegativeOne, One, Zero};
use malachite_nz::integer::arithmetic::eq_limb_mod_limb::limbs_eq_neg_limb_mod_limb;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigneds,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_unsigned, triples_of_integer_unsigned_and_unsigned,
    triples_of_integer_unsigned_and_unsigned_var_1, triples_of_integer_unsigned_and_unsigned_var_2,
    triples_of_unsigned_unsigned_and_integer,
};
use malachite_test::inputs::natural::{
    triples_of_natural_unsigned_and_unsigned, triples_of_unsigned_unsigned_and_natural,
};
#[cfg(feature = "32_bit_limbs")]
use rug;
use std::str::FromStr;

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
fn test_eq_limb_mod_limb() {
    let test = |n, u: Limb, modulus, out| {
        assert_eq!(Integer::from_str(n).unwrap().eq_mod(u, modulus), out);
        assert_eq!(u.eq_mod(&Integer::from_str(n).unwrap(), modulus), out);
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .is_congruent_u(u, modulus),
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
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limb_eq_limb_mod_integer() {
    let test = |u: u32, v, modulus, out| {
        assert_eq!(u.eq_mod(v, &Integer::from_str(modulus).unwrap()), out);
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
    test(57, 57, "-1000000000000", true);
    test(57, 58, "-1000000000000", false);
    test(0, 256, "-256", true);
    test(0, 256, "-512", false);
    test(13, 23, "-10", true);
    test(13, 24, "-10", false);
    test(13, 21, "-1", true);
    test(13, 21, "-2", true);
    test(13, 21, "-4", true);
    test(13, 21, "-8", true);
    test(13, 21, "-16", false);
    test(13, 21, "-3", false);
}

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_neg_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(
                (-Natural::from_limbs_asc(limbs)).eq_mod(limb, modulus),
                equal
            );
        },
    );
}

#[test]
fn eq_limb_mod_limb_properties() {
    test_properties(
        triples_of_integer_unsigned_and_unsigned,
        |&(ref n, u, modulus): &(Integer, Limb, Limb)| {
            let equal = n.eq_mod(u, modulus);
            assert_eq!(u.eq_mod(n, modulus), equal);
            assert_eq!(
                *n == u || modulus != 0 && n.mod_op(modulus) == u.mod_op(modulus),
                equal
            );
            assert_eq!((n - u).divisible_by(modulus), equal);
            assert_eq!((u - n).divisible_by(modulus), equal);

            //TODO assert_eq!(n.eq_mod(Integer::from(u), modulus), equal);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(integer_to_rug_integer(n).is_congruent_u(u, modulus), equal);
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_unsigned_var_1,
        |&(ref n, u, modulus): &(Integer, Limb, Limb)| {
            assert!(n.eq_mod(u, modulus));
            assert!(u.eq_mod(n, modulus));
            assert!(*n == u || modulus != 0 && n.mod_op(modulus) == u.mod_op(modulus));
            assert!((n - u).divisible_by(modulus));
            assert!((u - n).divisible_by(modulus));

            //TODO assert!(n.eq_mod(Integer::from(u), modulus));

            #[cfg(feature = "32_bit_limbs")]
            assert!(integer_to_rug_integer(n).is_congruent_u(u, modulus));
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_unsigned_var_2,
        |&(ref n, u, modulus): &(Integer, Limb, Limb)| {
            assert!(!n.eq_mod(u, modulus));
            assert!(!u.eq_mod(n, modulus));
            assert!(*n != u && (modulus == 0 || n.mod_op(modulus) != u.mod_op(modulus)));
            assert!(!(n - u).divisible_by(modulus));
            assert!(!(u - n).divisible_by(modulus));

            //TODO assert!(!n.eq_mod(Integer::from(u), modulus));

            #[cfg(feature = "32_bit_limbs")]
            assert!(!integer_to_rug_integer(n).is_congruent_u(u, modulus));
        },
    );

    test_properties(pairs_of_integer_and_unsigned, |&(ref n, u)| {
        assert!(n.eq_mod(u, 1 as Limb));
        assert!(u.eq_mod(n, 1 as Limb));
        assert_eq!(n.eq_mod(0 as Limb, u), n.divisible_by(u));
        assert_eq!((0 as Limb).eq_mod(n, u), n.divisible_by(u));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(u, modulus)| {
        assert!(Integer::from(u).eq_mod(u, modulus));
        assert!(u.eq_mod(&Integer::from(u), modulus));
        assert_eq!(Integer::ZERO.eq_mod(u, modulus), u.divisible_by(modulus));
        assert_eq!(u.eq_mod(&Integer::ZERO, modulus), u.divisible_by(modulus));
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(n, u, modulus)| {
        assert_eq!(n.eq_mod(u, modulus), Integer::from(n).eq_mod(u, modulus));
    });

    test_properties(
        triples_of_natural_unsigned_and_unsigned::<Limb>,
        |&(ref n, u, modulus)| {
            assert_eq!(n.eq_mod(u, modulus), Integer::from(n).eq_mod(u, modulus));
        },
    );
}

#[test]
fn limb_eq_limb_mod_integer_properties() {
    test_properties(
        triples_of_unsigned_unsigned_and_integer,
        |&(u, v, ref modulus): &(Limb, Limb, Integer)| {
            let equal = u.eq_mod(v, modulus);
            assert_eq!(v.eq_mod(u, modulus), equal);
            assert_eq!(
                u == v || *modulus != 0 as Limb && u.mod_op(modulus) == v.mod_op(modulus),
                equal
            );

            //TODO assert_eq!(Integer::from(u).eq_mod(v, modulus), equal);
        },
    );

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref n, u)| {
        assert_eq!(u.eq_mod(0, n), u.divisible_by(n));
        assert_eq!(0.eq_mod(u, n), u.divisible_by(n));
        assert!(u.eq_mod(u, n));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(u, v)| {
        assert!(u.eq_mod(v, &Integer::ONE));
        assert!(v.eq_mod(u, &Integer::ONE));
        assert!(u.eq_mod(v, &Integer::NEGATIVE_ONE));
        assert!(v.eq_mod(u, &Integer::NEGATIVE_ONE));
        assert_eq!(u.eq_mod(v, &Integer::ZERO), u == v);
        assert_eq!(v.eq_mod(u, &Integer::ZERO), u == v);
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(u, v, modulus)| {
        let equal = u.eq_mod(v, modulus);
        assert_eq!(EqMod::eq_mod(u, v, &Integer::from(modulus)), equal);
    });

    test_properties(
        triples_of_unsigned_unsigned_and_natural::<Limb>,
        |&(u, v, ref modulus)| {
            assert_eq!(u.eq_mod(v, modulus), u.eq_mod(v, &Integer::from(modulus)));
        },
    );
}
