use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::Natural;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::small_unsigneds;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_vec_of_bool_var_1,
};

#[test]
fn to_limbs_asc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_asc();
        assert_eq!(x.clone().into_limbs_asc(), limbs);
        assert_eq!(x.limbs().collect_vec(), limbs);
        assert_eq!(Natural::from_limbs_asc(&limbs), *x);
        if *x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    });
}

#[test]
fn to_limbs_desc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_desc();
        assert_eq!(x.clone().into_limbs_desc(), limbs);
        assert_eq!(x.limbs().rev().collect_vec(), limbs);
        assert_eq!(Natural::from_limbs_desc(&limbs), *x);
        if *x != 0 {
            assert_ne!(limbs[0], 0);
        }
    });
}

#[test]
fn limbs_properties() {
    test_properties(naturals, |n| {
        let limb_count = usize::exact_from(n.limb_count());
        assert_eq!(n.limbs().size_hint(), (limb_count, Some(limb_count)));
    });

    test_properties(
        pairs_of_natural_and_vec_of_bool_var_1,
        |&(ref n, ref bs)| {
            let mut limbs = n.limbs();
            let mut limb_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    limb_vec.insert(i, limbs.next().unwrap());
                    i += 1;
                } else {
                    limb_vec.insert(i, limbs.next_back().unwrap())
                }
            }
            assert!(limbs.next().is_none());
            assert!(limbs.next_back().is_none());
            assert_eq!(n.to_limbs_asc(), limb_vec);
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        if u < usize::exact_from(n.limb_count()) {
            assert_eq!(n.limbs()[u], n.to_limbs_asc()[u]);
        } else {
            assert_eq!(n.limbs()[u], 0);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(Natural::ZERO.limbs()[u], 0);
    });
}
