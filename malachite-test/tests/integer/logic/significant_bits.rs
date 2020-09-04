use malachite_base::num::arithmetic::traits::{Abs, PowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz_test_util::common::{integer_to_bigint, integer_to_rug_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::naturals;

#[test]
fn significant_bits_properties() {
    test_properties(integers, |x| {
        let significant_bits = x.significant_bits();
        assert_eq!(
            u64::wrapping_from(integer_to_bigint(x).bits()),
            significant_bits
        );
        assert_eq!(
            u64::from(integer_to_rug_integer(x).significant_bits()),
            significant_bits
        );

        let x_abs = x.abs();
        assert_eq!(x_abs <= Limb::MAX, significant_bits <= Limb::WIDTH);
        if x_abs != 0 {
            assert!(Natural::power_of_two(significant_bits - 1) <= x_abs);
            assert!(x_abs < Natural::power_of_two(significant_bits));
        }
    });

    test_properties(naturals, |n| {
        assert_eq!(Integer::from(n).significant_bits(), n.significant_bits());
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(Integer::from(i).significant_bits(), i.significant_bits());
    });
}
