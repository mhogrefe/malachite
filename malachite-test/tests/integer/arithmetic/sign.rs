use malachite_base::num::arithmetic::traits::Sign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz_test_util::integer::arithmetic::sign::num_sign;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn sign_properties() {
    test_properties(integers, |n| {
        let sign = n.sign();
        assert_eq!(integer_to_rug_integer(n).cmp0(), sign);
        assert_eq!(num_sign(&integer_to_bigint(n)), sign);
        assert_eq!(n.partial_cmp(&0), Some(sign));
        assert_eq!((-n).sign(), sign.reverse());
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(Integer::from(i).sign(), i.sign());
    });
}
