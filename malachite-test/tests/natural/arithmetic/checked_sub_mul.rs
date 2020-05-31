use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::triples_of_unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn checked_sub_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let result = a.checked_sub_mul(b, c);
        assert!(result.as_ref().map_or(true, |n| n.is_valid()));

        let result_alt = a.clone().checked_sub_mul(b, c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b, c.clone());
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), c.clone());
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        assert_eq!(a.checked_sub(b * c), result);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.checked_sub_mul(n, &Natural::ONE), Some(Natural::ZERO));
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.checked_sub_mul(&Natural::ZERO, b).as_ref(), Some(a));
        assert_eq!(a.checked_sub_mul(b, &Natural::ZERO).as_ref(), Some(a));
        assert_eq!(a.checked_sub_mul(&Natural::ONE, b), a.checked_sub(b));
        assert_eq!(a.checked_sub_mul(b, &Natural::ONE), a.checked_sub(b));
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(x, y, z)| {
        assert_eq!(
            Limb::from(x)
                .checked_sub_mul(Limb::from(y), Limb::from(z))
                .map(Natural::from),
            Natural::from(x).checked_sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
