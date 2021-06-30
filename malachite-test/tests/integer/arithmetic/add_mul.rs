use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, CheckedAddMul};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{triples_of_signeds, triples_of_signeds_var_2};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};

#[test]
fn add_mul_properties() {
    test_properties(triples_of_integers, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().add_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a + b * c, result);
        assert_eq!(a.add_mul(c, b), result);
        assert_eq!(a.add_mul(&(-b), &(-c)), result);
        assert_eq!((-a).add_mul(&(-b), c), -&result);
        assert_eq!((-a).add_mul(b, -c), -result);
    });

    test_properties(integers, |a| {
        assert_eq!(a.add_mul(a, &Integer::NEGATIVE_ONE), 0);
        assert_eq!(a.add_mul(&(-a), &Integer::ONE), 0);
    });

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.add_mul(&Integer::ONE, b), a + b);
        assert_eq!(Integer::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.add_mul(b, &Integer::ONE), a + b);
        assert_eq!((a * b).add_mul(-a, b), 0);
        assert_eq!((a * b).add_mul(a, -b), 0);
    });

    test_properties(triples_of_signeds_var_2::<SignedLimb>, |&(x, y, z)| {
        assert_eq!(
            SignedLimb::from(x).add_mul(SignedLimb::from(y), SignedLimb::from(z)),
            Integer::from(x).add_mul(Integer::from(y), Integer::from(z))
        );
    });

    test_properties(triples_of_signeds::<SignedLimb>, |&(x, y, z)| {
        let result = Integer::from(x).add_mul(Integer::from(y), Integer::from(z));
        assert_eq!(
            SignedLimb::from(x)
                .checked_add_mul(SignedLimb::from(y), SignedLimb::from(z))
                .is_some(),
            SignedLimb::convertible_from(&result)
        );
    });
}
