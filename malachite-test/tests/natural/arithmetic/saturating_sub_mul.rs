use malachite_base::num::arithmetic::traits::{
    SaturatingSub, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn saturating_sub_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.saturating_sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a.saturating_sub(b * c), result);
        assert!(result <= *a);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.saturating_sub_mul(n, &Natural::ONE), 0);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(Natural::ZERO.saturating_sub_mul(a, b), 0);
        assert_eq!(a.saturating_sub_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.saturating_sub_mul(b, &Natural::ZERO), *a);
        assert_eq!((a * b).saturating_sub_mul(a, b), 0);
        assert_eq!(a.saturating_sub_mul(&Natural::ONE, b), a.saturating_sub(b));
        assert_eq!(a.saturating_sub_mul(b, &Natural::ONE), a.saturating_sub(b));
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(x, y, z)| {
        assert_eq!(
            Limb::from(x).saturating_sub_mul(Limb::from(y), Limb::from(z)),
            Natural::from(x).saturating_sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
