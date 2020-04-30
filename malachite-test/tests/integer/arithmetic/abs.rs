use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::CheckedInto;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use num::Signed;

use malachite_test::common::test_properties;
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn abs_properties() {
    test_properties(integers, |x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(bigint_to_integer(&integer_to_bigint(x).abs()), abs);

        assert_eq!(
            rug_integer_to_integer(&integer_to_rug_integer(x).abs()),
            abs
        );

        let abs_alt = x.abs();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(abs_alt, abs);

        assert!(abs >= 0);
        assert_eq!(abs == *x, *x >= 0);
        assert_eq!((&abs).abs(), abs);

        let abs_alt = x.clone().unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(abs_alt), (&abs).checked_into());

        let abs_alt = x.unsigned_abs();
        assert!(abs_alt.is_valid());
        assert_eq!(Some(&abs_alt), abs.checked_into().as_ref());

        let internal_abs = x.unsigned_abs_ref();
        assert!(internal_abs.is_valid());
        assert_eq!(*internal_abs, abs_alt);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).abs(),
            Integer::from(SignedDoubleLimb::from(i).abs())
        );
    });
}
