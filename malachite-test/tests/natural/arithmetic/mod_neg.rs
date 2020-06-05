use malachite_base::num::arithmetic::traits::{Mod, ModAdd, ModIsReduced, ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigneds_var_5;
use malachite_test::inputs::natural::pairs_of_naturals_var_2;

#[test]
fn mod_neg_properties() {
    test_properties(pairs_of_naturals_var_2, |(n, m)| {
        assert!(n.mod_is_reduced(m));
        let neg = n.mod_neg(m);
        assert!(neg.is_valid());
        assert!(neg.mod_is_reduced(m));

        let neg_alt = n.mod_neg(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(m);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(m);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg, (-n).mod_op(Integer::from(m)));
        assert_eq!((&neg).mod_neg(m), *n);
        assert_eq!(n.mod_add(&neg, m), 0);
        assert_eq!(*n == neg, *n == Natural::ZERO || n << 1 == *m);
    });

    test_properties(pairs_of_unsigneds_var_5::<Limb>, |&(n, m)| {
        assert_eq!(n.mod_neg(m), Natural::from(n).mod_neg(Natural::from(m)));
    });
}
