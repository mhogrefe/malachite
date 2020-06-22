use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::slices::slice_test_zero;
use malachite_nz::natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_ref_ref,
    limbs_divisible_by_ref_val, limbs_divisible_by_val_ref,
};
use malachite_nz::natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_nz_test_util::natural::arithmetic::divisible_by::{
    combined_limbs_divisible_by_limb, num_divisible_by,
};

use malachite_test::common::{test_properties, test_properties_custom_scale};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_var_14, pairs_of_limb_vec_var_15,
    pairs_of_unsigned_vec_and_positive_unsigned_var_1, pairs_of_unsigned_vec_var_13,
    pairs_of_unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural_var_1,
    pairs_of_natural_and_positive_natural_var_2, pairs_of_naturals, positive_naturals,
};

fn verify_limbs_divisible_by(ns: &[Limb], ds: &[Limb], divisible: bool) {
    assert_eq!(
        Natural::from_limbs_asc(ns).divisible_by(Natural::from_limbs_asc(ds)),
        divisible
    );
}

#[test]
fn limbs_divisible_by_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let divisible = limbs_divisible_by_limb(limbs, limb);
            assert_eq!(
                (&Natural::from_limbs_asc(limbs)).divisible_by(Natural::from(limb)),
                divisible
            );
            assert_eq!(limbs_mod_limb(limbs, limb) == 0, divisible);
            assert_eq!(combined_limbs_divisible_by_limb(limbs, limb), divisible);
        },
    );
}

#[test]
fn limbs_divisible_by_properties() {
    test_properties_custom_scale(512, pairs_of_unsigned_vec_var_13, |&(ref ns, ref ds)| {
        let mut mut_ns = ns.to_vec();
        let mut mut_ds = ds.to_vec();
        let divisible = limbs_divisible_by(&mut mut_ns, &mut mut_ds);

        let mut mut_ns = ns.to_vec();
        assert_eq!(limbs_divisible_by_val_ref(&mut mut_ns, ds), divisible);

        let mut mut_ds = ds.to_vec();
        assert_eq!(limbs_divisible_by_ref_val(ns, &mut mut_ds), divisible);

        assert_eq!(limbs_divisible_by_ref_ref(ns, ds), divisible);

        verify_limbs_divisible_by(ns, ds, divisible);
    });

    test_properties_custom_scale(512, pairs_of_limb_vec_var_15, |&(ref ns, ref ds)| {
        let mut mut_ns = ns.to_vec();
        let mut mut_ds = ds.to_vec();
        assert!(limbs_divisible_by(&mut mut_ns, &mut mut_ds));

        let mut mut_ns = ns.to_vec();
        assert!(limbs_divisible_by_val_ref(&mut mut_ns, ds));

        let mut mut_ds = ds.to_vec();
        assert!(limbs_divisible_by_ref_val(ns, &mut mut_ds));

        assert!(limbs_divisible_by_ref_ref(ns, ds));
        verify_limbs_divisible_by(ns, ds, true);
    });

    test_properties_custom_scale(512, pairs_of_limb_vec_var_14, |&(ref ns, ref ds)| {
        let divisible = limbs_divisible_by_ref_ref(ns, ds);
        assert_eq!(slice_test_zero(&limbs_mod(ns, ds)), divisible);
    });
}

#[test]
fn divisible_by_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(x.divisible_by(y.clone()), divisible);
        assert_eq!(x.clone().divisible_by(y), divisible);
        assert_eq!(x.clone().divisible_by(y.clone()), divisible);

        assert_eq!(*x == 0 || *y != 0 && x % y == 0, divisible);
        assert_eq!(
            num_divisible_by(&natural_to_biguint(x), &natural_to_biguint(y)),
            divisible
        );
        assert_eq!(
            natural_to_rug_integer(x).is_divisible(&natural_to_rug_integer(y)),
            divisible
        );
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            assert!(x.divisible_by(y));
            assert!(*x == 0 || *y != 0 && x % y == 0);
            assert!(num_divisible_by(
                &natural_to_biguint(x),
                &natural_to_biguint(y)
            ));
            assert!(natural_to_rug_integer(x).is_divisible(&natural_to_rug_integer(y)));
        },
    );

    test_properties(
        pairs_of_natural_and_positive_natural_var_2,
        |&(ref x, ref y)| {
            assert!(!x.divisible_by(y));
            assert!(*x != 0 && (*y == 0 || x % y != 0));
            assert!(!num_divisible_by(
                &natural_to_biguint(x),
                &natural_to_biguint(y)
            ));
            assert!(!natural_to_rug_integer(x).is_divisible(&natural_to_rug_integer(y)));
        },
    );

    test_properties(naturals, |n| {
        assert!(n.divisible_by(Natural::ONE));
    });

    test_properties(positive_naturals, |n| {
        assert!(!n.divisible_by(Natural::ZERO));
        assert!(Natural::ZERO.divisible_by(n));
        if *n > 1 {
            assert!(!Natural::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(x).divisible_by(Natural::from(y)),
            x.divisible_by(y)
        );
    });
}
