use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz_test_util::integer::logic::trailing_zeros::integer_trailing_zeros_alt;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::nonzero_signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn trailing_zeros_properties() {
    test_properties(integers, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(integer_trailing_zeros_alt(x), trailing_zeros);
        assert_eq!(trailing_zeros.is_none(), *x == 0);
        assert_eq!((-x).trailing_zeros(), trailing_zeros);
        if *x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            assert_ne!((!x).trailing_zeros() == Some(0), trailing_zeros == 0);
            if trailing_zeros <= u64::from(Limb::MAX) {
                assert!((x >> trailing_zeros).odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });

    test_properties(nonzero_signeds::<SignedLimb>, |&i| {
        assert_eq!(
            Integer::from(i).trailing_zeros(),
            Some(TrailingZeros::trailing_zeros(i))
        );
    });
}
