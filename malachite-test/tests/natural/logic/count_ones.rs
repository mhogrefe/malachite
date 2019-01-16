use common::test_properties;
use malachite_base::num::IsPowerOfTwo;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::{unsigneds, vecs_of_unsigned};
use malachite_test::inputs::natural::naturals;
use malachite_test::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_count_ones() {
    let test = |limbs, out| {
        assert_eq!(limbs_count_ones(limbs), out);
    };
    test(&[], 0);
    test(&[0, 1, 2], 2);
    test(&[1, 0xffff_ffff], 33);
}

#[test]
fn test_count_ones() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().count_ones(), out);
        assert_eq!(
            natural_count_ones_alt_1(&Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            natural_count_ones_alt_2(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("105", 4);
    test("1000000000000", 13);
    test("4294967295", 32);
    test("4294967296", 1);
    test("18446744073709551615", 64);
    test("18446744073709551616", 1);
}

#[test]
fn limbs_count_ones_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            limbs_count_ones(limbs),
            Natural::from_limbs_asc(limbs).count_ones()
        );
    });
}

#[test]
fn count_ones_properties() {
    test_properties(naturals, |x| {
        let ones = x.count_ones();
        assert_eq!(natural_count_ones_alt_1(x), ones);
        assert_eq!(natural_count_ones_alt_2(x), ones);
        assert_eq!(ones == 0, *x == 0);
        assert_eq!(ones == 1, x.is_power_of_two());
        assert_eq!((!x).checked_count_zeros(), Some(ones));
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert_eq!(Natural::from(u).count_ones(), u64::from(u.count_ones()));
    });
}
