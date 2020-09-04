use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::eq_mod::{
    _limbs_limb_mod_exact_odd_limb, _limbs_mod_exact_odd_limb, limbs_eq_limb_mod,
    limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref, limbs_eq_limb_mod_ref_val,
    limbs_eq_limb_mod_val_ref, limbs_eq_mod_limb_ref_ref, limbs_eq_mod_limb_ref_val,
    limbs_eq_mod_limb_val_ref, limbs_eq_mod_ref_ref_ref, limbs_eq_mod_ref_ref_val,
    limbs_eq_mod_ref_val_ref, limbs_eq_mod_ref_val_val,
};
use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::natural::arithmetic::eq_mod::{
    _combined_limbs_eq_limb_mod_limb, limbs_eq_limb_mod_naive_1, limbs_eq_limb_mod_naive_2,
    limbs_eq_mod_limb_naive_1, limbs_eq_mod_limb_naive_2, limbs_eq_mod_naive_1,
    limbs_eq_mod_naive_2,
};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_limb_vec_limb_and_limb_vec_var_2, triples_of_limb_vec_limb_and_limb_vec_var_3,
    triples_of_limb_vec_limb_and_positive_limb_var_3,
    triples_of_limb_vec_limb_and_positive_limb_var_4, triples_of_limb_vec_limb_vec_and_limb_var_10,
    triples_of_limb_vec_limb_vec_and_limb_var_9, triples_of_limb_vec_var_56,
    triples_of_limb_vec_var_57, triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8, triples_of_unsigned_vec_var_55,
    triples_of_unsigneds, triples_of_unsigneds_var_6,
};
use malachite_test::inputs::natural::{
    pairs_of_naturals, triples_of_naturals, triples_of_naturals_var_2, triples_of_naturals_var_3,
};

#[test]
fn limbs_limb_mod_exact_odd_limb_properties() {
    test_properties(triples_of_unsigneds_var_6, |&(n, d, carry)| {
        let r = Natural::from(_limbs_limb_mod_exact_odd_limb(n, d, carry));
        assert_eq!(n.eq_mod(carry, d), r == 0);

        assert!(r <= d);
        if carry < d {
            assert!(r < d);
        }

        let a = Natural::from(n);
        let d = Natural::from(d);
        let carry = Natural::from(carry);
        assert!(((&r << Limb::WIDTH) + &a).eq_mod(&carry, &d) || (r + a).eq_mod(carry, d));
    });
}

#[test]
fn limbs_mod_exact_odd_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_var_1,
        |&(ref ns, d, carry)| {
            let r = Natural::from(_limbs_mod_exact_odd_limb(ns, d, carry));

            assert!(r <= d);
            if carry < d {
                assert!(r < d);
            }

            let a = Natural::from_limbs_asc(ns);
            let d = Natural::from(d);
            let carry = Natural::from(carry);
            assert_eq!((&a).eq_mod(&carry, &d), r == 0 || r == d);

            let p_1 = &r << (u64::exact_from(ns.len()) * Limb::WIDTH);
            let p_2 = r << (u64::exact_from(ns.len() - 1) * Limb::WIDTH);
            assert!((p_1 + &a).eq_mod(&carry, &d) || (p_2 + a).eq_mod(carry, d));
        },
    );
}

#[test]
fn limbs_eq_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, m)| {
            let equal = limbs_eq_limb_mod_limb(limbs, limb, m);
            assert_eq!(
                Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(m)),
                equal
            );
            assert_eq!(limbs_mod_limb(limbs, m) == limb % m, equal);
            assert_eq!(_combined_limbs_eq_limb_mod_limb(limbs, limb, m), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_3,
        |&(ref limbs, limb, m)| {
            assert!(limbs_eq_limb_mod_limb(limbs, limb, m));
            assert!(Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(m)));
            assert_eq!(limbs_mod_limb(limbs, m), limb % m);
            assert!(_combined_limbs_eq_limb_mod_limb(limbs, limb, m));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_4,
        |&(ref limbs, limb, m)| {
            assert!(!limbs_eq_limb_mod_limb(limbs, limb, m));
            assert!(!Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(m)));
            assert_ne!(limbs_mod_limb(limbs, m), limb % m);
            assert!(!_combined_limbs_eq_limb_mod_limb(limbs, limb, m));
        },
    );
}

#[test]
fn limbs_eq_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
        |&(ref xs, y, ref ms)| {
            let equal = limbs_eq_limb_mod_ref_ref(xs, y, ms);
            let mut mut_xs = xs.clone();
            let mut mut_ms = ms.clone();
            assert_eq!(limbs_eq_limb_mod(&mut mut_xs, y, &mut mut_ms), equal);
            let mut mut_xs = xs.clone();
            assert_eq!(limbs_eq_limb_mod_val_ref(&mut mut_xs, y, ms), equal);
            let mut mut_ms = ms.clone();
            assert_eq!(limbs_eq_limb_mod_ref_val(xs, y, &mut mut_ms), equal);
            assert_eq!(
                Natural::from_limbs_asc(xs).eq_mod(Natural::from(y), Natural::from_limbs_asc(ms)),
                equal
            );
            assert_eq!(limbs_eq_limb_mod_naive_1(xs, y, ms), equal);
            assert_eq!(limbs_eq_limb_mod_naive_2(xs, y, ms), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_2,
        |&(ref xs, y, ref ms)| {
            assert!(
                Natural::from_limbs_asc(xs).eq_mod(Natural::from(y), Natural::from_limbs_asc(ms))
            );
            assert!(limbs_eq_limb_mod_ref_ref(xs, y, ms));
            assert!(limbs_eq_limb_mod_naive_1(xs, y, ms));
            assert!(limbs_eq_limb_mod_naive_2(xs, y, ms));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_3,
        |&(ref xs, y, ref ms)| {
            assert!(
                !Natural::from_limbs_asc(xs).eq_mod(Natural::from(y), Natural::from_limbs_asc(ms))
            );
            assert!(!limbs_eq_limb_mod_ref_ref(xs, y, ms));
            assert!(!limbs_eq_limb_mod_naive_1(xs, y, ms));
            assert!(!limbs_eq_limb_mod_naive_2(xs, y, ms));
        },
    );
}

#[test]
fn limbs_eq_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
        |&(ref xs, ref ys, m)| {
            let equal = limbs_eq_mod_limb_ref_ref(xs, ys, m);
            let mut mut_xs = xs.clone();
            assert_eq!(limbs_eq_mod_limb_val_ref(&mut mut_xs, ys, m), equal);
            let mut mut_ys = ys.clone();
            assert_eq!(limbs_eq_mod_limb_ref_val(xs, &mut mut_ys, m), equal);
            assert_eq!(
                Natural::from_limbs_asc(xs).eq_mod(Natural::from_limbs_asc(ys), Natural::from(m)),
                equal
            );
            assert_eq!(limbs_eq_mod_limb_naive_1(xs, ys, m), equal);
            assert_eq!(limbs_eq_mod_limb_naive_2(xs, ys, m), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_9,
        |&(ref xs, ref ys, m)| {
            assert!(
                Natural::from_limbs_asc(xs).eq_mod(Natural::from_limbs_asc(ys), Natural::from(m))
            );
            assert!(limbs_eq_mod_limb_ref_ref(xs, ys, m));
            assert!(limbs_eq_mod_limb_naive_1(xs, ys, m));
            assert!(limbs_eq_mod_limb_naive_2(xs, ys, m));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_10,
        |&(ref xs, ref ys, m)| {
            assert!(
                !Natural::from_limbs_asc(xs).eq_mod(Natural::from_limbs_asc(ys), Natural::from(m))
            );
            assert!(!limbs_eq_mod_limb_ref_ref(xs, ys, m));
            assert!(!limbs_eq_mod_limb_naive_1(xs, ys, m));
            assert!(!limbs_eq_mod_limb_naive_2(xs, ys, m));
        },
    );
}

#[test]
fn limbs_eq_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_var_55,
        |&(ref xs, ref ys, ref ms)| {
            let equal = limbs_eq_mod_ref_ref_ref(xs, ys, ms);
            let mut mut_ys = ys.clone();
            let mut mut_ms = ms.clone();
            assert_eq!(
                limbs_eq_mod_ref_val_val(xs, &mut mut_ys, &mut mut_ms),
                equal
            );
            let mut mut_ys = ys.clone();
            assert_eq!(limbs_eq_mod_ref_val_ref(xs, &mut mut_ys, ms), equal);
            let mut mut_ms = ms.clone();
            assert_eq!(limbs_eq_mod_ref_ref_val(xs, ys, &mut mut_ms), equal);
            assert_eq!(
                Natural::from_limbs_asc(xs)
                    .eq_mod(Natural::from_limbs_asc(ys), Natural::from_limbs_asc(ms)),
                equal
            );
            assert_eq!(limbs_eq_mod_naive_1(xs, ys, ms), equal);
            assert_eq!(limbs_eq_mod_naive_2(xs, ys, ms), equal);
        },
    );

    test_properties(triples_of_limb_vec_var_56, |&(ref xs, ref ys, ref ms)| {
        assert!(Natural::from_limbs_asc(xs)
            .eq_mod(Natural::from_limbs_asc(ys), Natural::from_limbs_asc(ms)));
        assert!(limbs_eq_mod_ref_ref_ref(xs, ys, ms));
        assert!(limbs_eq_mod_naive_1(xs, ys, ms));
        assert!(limbs_eq_mod_naive_2(xs, ys, ms));
    });

    test_properties(triples_of_limb_vec_var_57, |&(ref xs, ref ys, ref ms)| {
        assert!(!Natural::from_limbs_asc(xs)
            .eq_mod(Natural::from_limbs_asc(ys), Natural::from_limbs_asc(ms)));
        assert!(!limbs_eq_mod_ref_ref_ref(xs, ys, ms));
        assert!(!limbs_eq_mod_naive_1(xs, ys, ms));
        assert!(!limbs_eq_mod_naive_2(xs, ys, ms));
    });
}

#[test]
fn eq_mod_properties() {
    test_properties(triples_of_naturals, |&(ref x, ref y, ref m)| {
        let equal = x.eq_mod(y, m);
        assert_eq!(y.eq_mod(x, m), equal);

        assert_eq!(x.eq_mod(y, m.clone()), equal);
        assert_eq!(x.eq_mod(y.clone(), m), equal);
        assert_eq!(x.eq_mod(y.clone(), m.clone()), equal);
        assert_eq!(x.clone().eq_mod(y, m), equal);
        assert_eq!(x.clone().eq_mod(y, m.clone()), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), m), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), m.clone()), equal);

        assert_eq!(
            (Integer::from(x) - Integer::from(y)).divisible_by(Integer::from(m)),
            equal
        );
        assert_eq!(
            (Integer::from(y) - Integer::from(x)).divisible_by(Integer::from(m)),
            equal
        );
        assert_eq!(
            natural_to_rug_integer(x)
                .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(m)),
            equal
        );
    });

    test_properties(triples_of_naturals_var_2, |&(ref x, ref y, ref m)| {
        assert!(x.eq_mod(y, m));
        assert!(y.eq_mod(x, m));
        assert!(natural_to_rug_integer(x)
            .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(m)));
    });

    test_properties(triples_of_naturals_var_3, |&(ref x, ref y, ref m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
        assert!(!natural_to_rug_integer(x)
            .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(m)));
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert!(x.eq_mod(y, Natural::ONE));
        assert_eq!(x.eq_mod(Natural::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
        assert_eq!(x.eq_mod(y, Natural::ZERO), x == y);
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(x, y, m)| {
        assert_eq!(
            Natural::from(x).eq_mod(Natural::from(y), Natural::from(m)),
            x.eq_mod(y, m)
        );
    });
}
