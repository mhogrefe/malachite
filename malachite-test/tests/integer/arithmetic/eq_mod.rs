use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, Mod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{integer_to_rug_integer, natural_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_limb_limb_and_limb_vec_var_2, triples_of_limb_vec_limb_and_limb_vec_var_4,
    triples_of_limb_vec_limb_and_limb_vec_var_5, triples_of_limb_vec_limb_vec_and_limb_var_11,
    triples_of_limb_vec_limb_vec_and_limb_var_12, triples_of_limb_vec_var_58,
    triples_of_limb_vec_var_59, triples_of_unsigned_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8, triples_of_unsigned_vec_var_55,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_integers, triples_of_integer_integer_and_natural,
    triples_of_integer_integer_and_natural_var_1, triples_of_integer_integer_and_natural_var_2,
};

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, m)| {
            let equal = limbs_eq_neg_limb_mod_limb(limbs, limb, m);
            assert_eq!(
                (-Natural::from_limbs_asc(limbs)).eq_mod(Integer::from(limb), Natural::from(m)),
                equal
            );
        },
    );
}

#[test]
fn limbs_pos_limb_eq_neg_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_unsigned_and_unsigned_vec_var_1,
        |&(x, y, ref ms)| {
            let equal = limbs_pos_limb_eq_neg_limb_mod(x, y, ms);
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let m = Natural::from_limbs_asc(ms);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    test_properties(
        triples_of_limb_limb_and_limb_vec_var_2,
        |&(x, y, ref ms)| {
            assert!(!limbs_pos_limb_eq_neg_limb_mod(x, y, ms));
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let m = Natural::from_limbs_asc(ms);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
        |&(ref xs, y, ref ms)| {
            let equal = limbs_pos_eq_neg_limb_mod_ref(xs, y, ms);
            let mut mut_ms = ms.clone();
            assert_eq!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_ms), equal);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let m = Natural::from_limbs_asc(ms);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_4,
        |&(ref xs, y, ref ms)| {
            assert!(limbs_pos_eq_neg_limb_mod_ref(xs, y, ms));
            let mut mut_ms = ms.clone();
            assert!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_ms));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let m = Natural::from_limbs_asc(ms);
            assert!((&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
            assert!((x - y).divisible_by(m));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_5,
        |&(ref xs, y, ref ms)| {
            assert!(!limbs_pos_eq_neg_limb_mod_ref(xs, y, ms));
            let mut mut_ms = ms.clone();
            assert!(!limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_ms));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let m = Natural::from_limbs_asc(ms);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
        |&(ref xs, ref ys, m)| {
            let equal = limbs_pos_eq_neg_mod_limb(xs, ys, m);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let m = Natural::from(m);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_11,
        |&(ref xs, ref ys, m)| {
            assert!(limbs_pos_eq_neg_mod_limb(xs, ys, m));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let m = Natural::from(m);
            assert!((&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
            assert!((x - y).divisible_by(m));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_12,
        |&(ref xs, ref ys, m)| {
            assert!(!limbs_pos_eq_neg_mod_limb(xs, ys, m));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let m = Natural::from(m);
            assert!(!(&x).eq_mod(&y, &m));
            let m = Integer::from(m);
            assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
            assert!(!(x - y).divisible_by(m));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_var_55,
        |&(ref xs, ref ys, ref ms)| {
            let equal = limbs_pos_eq_neg_mod_ref(xs, ys, ms);
            let mut mut_ms = ms.clone();
            assert_eq!(limbs_pos_eq_neg_mod(xs, ys, &mut mut_ms), equal);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let m = Natural::from_limbs_asc(ms);
            assert_eq!((&x).eq_mod(&y, &m), equal);
            let m = Integer::from(m);
            assert_eq!(
                x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
                equal
            );
            assert_eq!((x - y).divisible_by(m), equal);
        },
    );

    test_properties(triples_of_limb_vec_var_58, |&(ref xs, ref ys, ref ms)| {
        assert!(limbs_pos_eq_neg_mod_ref(xs, ys, ms));
        let mut mut_ms = ms.clone();
        assert!(limbs_pos_eq_neg_mod_ref(xs, ys, &mut mut_ms));
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from_limbs_asc(ms);
        assert!((&x).eq_mod(&y, &m));
        let m = Integer::from(m);
        assert!(x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m));
        assert!((x - y).divisible_by(m));
    });

    test_properties(triples_of_limb_vec_var_59, |&(ref xs, ref ys, ref ms)| {
        assert!(!limbs_pos_eq_neg_mod_ref(xs, ys, ms));
        let mut mut_ms = ms.clone();
        assert!(!limbs_pos_eq_neg_mod_ref(xs, ys, &mut mut_ms));
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from_limbs_asc(ms);
        assert!(!(&x).eq_mod(&y, &m));
        let m = Integer::from(m);
        assert!(x != y && (m == 0 || (&x).mod_op(&m) != (&y).mod_op(&m)));
        assert!(!(x - y).divisible_by(m));
    });
}

#[test]
fn eq_mod_properties() {
    test_properties(
        triples_of_integer_integer_and_natural,
        |&(ref x, ref y, ref m)| {
            let equal = x.eq_mod(y, m);
            assert_eq!(y.eq_mod(x, m), equal);

            assert_eq!(x.eq_mod(y, m.clone()), equal);
            assert_eq!(x.eq_mod(y.clone(), m), equal);
            assert_eq!(x.eq_mod(y.clone(), m.clone()), equal);
            assert_eq!(x.clone().eq_mod(y, m), equal);
            assert_eq!(x.clone().eq_mod(y, m.clone()), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), m), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), m.clone()), equal);

            assert_eq!((-x).eq_mod(-y, m), equal);
            assert_eq!((x - y).divisible_by(Integer::from(m)), equal);
            assert_eq!((y - x).divisible_by(Integer::from(m)), equal);
            assert_eq!(
                integer_to_rug_integer(x)
                    .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(m)),
                equal
            );
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_1,
        |&(ref x, ref y, ref m)| {
            assert!(x.eq_mod(y, m));
            assert!(y.eq_mod(x, m));
            assert!(integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(m)));
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_2,
        |&(ref x, ref y, ref m)| {
            assert!(!x.eq_mod(y, m));
            assert!(!y.eq_mod(x, m));
            assert!(!integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(m)));
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
