use itertools::Itertools;
use malachite_base::slices::slice_test_zero;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[test]
fn from_limbs_asc_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let x = Natural::from_limbs_asc(limbs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_asc(limbs.clone()), x);
        let mut trimmed_limbs = limbs
            .iter()
            .cloned()
            .rev()
            .skip_while(|&limb| limb == 0)
            .collect_vec();
        trimmed_limbs.reverse();
        assert_eq!(x.to_limbs_asc(), trimmed_limbs);
        assert_eq!(
            Natural::from_limbs_desc(&limbs.iter().cloned().rev().collect_vec()),
            x
        );
        if !limbs.is_empty() && *limbs.last().unwrap() != 0 {
            assert_eq!(x.to_limbs_asc(), *limbs);
        }
        assert_eq!(slice_test_zero(limbs), x == 0);
    });
}

#[test]
fn from_limbs_desc_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let x = Natural::from_limbs_desc(limbs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_desc(limbs.clone()), x);
        assert_eq!(
            x.to_limbs_desc(),
            limbs
                .iter()
                .cloned()
                .skip_while(|&limb| limb == 0)
                .collect_vec()
        );
        assert_eq!(
            Natural::from_limbs_asc(&limbs.iter().cloned().rev().collect_vec()),
            x
        );
        if !limbs.is_empty() && limbs[0] != 0 {
            assert_eq!(x.to_limbs_desc(), *limbs);
        }
        assert_eq!(slice_test_zero(limbs), x == 0);
    });
}
