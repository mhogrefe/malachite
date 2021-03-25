use malachite_base::num::arithmetic::traits::EqModPowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::eq_mod_power_of_two::{
    limbs_eq_limb_mod_power_of_two, limbs_eq_mod_power_of_two,
};
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod_power_of_two() {
    let test = |xs, y, pow, out| {
        assert_eq!(limbs_eq_limb_mod_power_of_two(xs, y, pow), out);
    };
    test(&[0b1111011, 0b111001000], 0b101011, 4, true);
    test(&[0b1111011, 0b111001000], 0b101011, 5, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 35, true);
    test(&[0b1111011, 0b111001000], 0b1111011, 36, false);
    test(&[0b1111011, 0b111001000], 0b1111011, 100, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_two() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_two(xs, ys, pow), out);
    };
    test(&[0b1111011, 0b111001000], &[0b101011], 4, true);
    test(&[0b1111011, 0b111001000], &[0b101011], 5, false);
    test(&[0b1111011, 0b111001000], &[0b1111011], 35, true);
    test(&[0b1111011, 0b111001000], &[0b1111011], 36, false);
    test(&[0b1111011, 0b111001000], &[0b1111011], 100, false);
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        37,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        38,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b1111011, 0b111101000],
        100,
        false,
    );
}

#[test]
fn test_eq_mod_power_of_two() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .eq_mod_power_of_two(&Natural::from_str(y).unwrap(), pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x)
                .unwrap()
                .is_congruent_2pow(&rug::Integer::from_str(y).unwrap(), u32::exact_from(pow)),
            out
        );
    };
    test("0", "256", 8, true);
    test("0", "256", 9, false);
    test("13", "21", 0, true);
    test("13", "21", 1, true);
    test("13", "21", 2, true);
    test("13", "21", 3, true);
    test("13", "21", 4, false);
    test("13", "21", 100, false);
    test("1000000000001", "1", 12, true);
    test("1000000000001", "1", 13, false);
    test("4294967295", "4294967295", 32, true);
    test("281474976710672", "844424930131984", 49, true);
    test("281474976710672", "844424930131984", 50, false);
}
