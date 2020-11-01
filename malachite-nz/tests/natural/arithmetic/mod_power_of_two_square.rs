use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, Square};
use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::basic::integers::PrimitiveInt;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_two_square::_limbs_square_low_basecase;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_square_low_basecase() {
    let test = |out_before: &[Limb], xs: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        _limbs_square_low_basecase(&mut out, xs);
        assert_eq!(out, out_after);

        let len = xs.len();
        let x = Natural::from_limbs_asc(xs);
        let pow = u64::exact_from(len) << Limb::LOG_WIDTH;
        let expected_square = (&x).square().mod_power_of_two(pow);
        assert_eq!(x.mod_power_of_two_square(pow), expected_square);
        let square = Natural::from_limbs_asc(&out_after[..len]);
        assert_eq!(square, expected_square);
        assert_eq!(&out_before[len..], &out_after[len..]);
    };
    // n == 1
    test(&[10; 3], &[1], &[1, 10, 10]);
    // n == 2
    test(&[10; 3], &[123, 456], &[15129, 112176, 10]);
    // n > 2
    // n.odd() in _limbs_square_low_diagonal
    test(&[10; 4], &[123, 456, 789], &[15129, 112176, 402030, 10]);
    // n.even() in _limbs_square_low_diagonal
    test(
        &[10; 5],
        &[123, 456, 789, 987],
        &[15129, 112176, 402030, 962370, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_basecase_fail_1() {
    _limbs_square_low_basecase(&mut [10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_square_low_basecase_fail_2() {
    _limbs_square_low_basecase(&mut [10, 10], &[]);
}

#[test]
fn test_mod_power_of_two_square() {
    let test = |u, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        let n = Natural::from_str(u).unwrap().mod_power_of_two_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_two_is_reduced(pow));

        let n = (&Natural::from_str(u).unwrap()).mod_power_of_two_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_square_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "0");
    test("2", 3, "4");
    test("5", 3, "1");
    test("100", 8, "16");
    test("12345678987654321", 64, "16556040056090124897");
}
