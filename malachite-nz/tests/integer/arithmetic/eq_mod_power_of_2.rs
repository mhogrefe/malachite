use malachite_base::num::arithmetic::traits::EqModPowerOf2;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::basic::integers::PrimitiveInt;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::arithmetic::eq_mod_power_of_2::{
    limbs_eq_mod_power_of_2_neg_limb, limbs_eq_mod_power_of_2_neg_pos,
};
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_2_neg_limb() {
    let test = |xs, y, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_2_neg_limb(xs, y, pow), out);
    };
    let width = Limb::WIDTH;
    test(&[1, 1], 3, 0, true);
    test(&[1, 1], 3, 1, true);
    test(&[1, 1], 3, 2, true);
    test(&[1, 1], 3, 3, false);
    test(&[1, 1], u32::MAX, 0, true);
    test(&[1, 1], u32::MAX, 1, true);
    test(&[1, 1], u32::MAX, width, true);
    test(&[1, 1], u32::MAX, width + 1, true);
    test(&[1, 2], u32::MAX, width + 1, false);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, width + 1, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 2 * width, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 3 * width - 1, true);
    test(&[1, u32::MAX, u32::MAX], u32::MAX, 3 * width, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_power_of_2_neg_pos() {
    let test = |xs, ys, pow, out| {
        assert_eq!(limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow), out);
    };
    test(&[0b1111011, 0b111001000], &[0b10101], 4, true);
    test(&[0b1111011, 0b111001000], &[0b10101], 5, false);
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        35,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        36,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b1111],
        100,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        37,
        true,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        38,
        false,
    );
    test(
        &[0b1111011, 0b111001000],
        &[0b11111111111111111111111110000101, 0b10111],
        100,
        false,
    );

    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        64,
        true,
    );
    test(&[0xabcdabcd, 0x12341234], &[0, 0xedcbedcb], 64, false);
    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        65,
        false,
    );
    test(
        &[0xabcdabcd, 0x12341234],
        &[0x54325433, 0xedcbedcb],
        128,
        false,
    );
    test(&[0, 0, 0x12341234], &[0, 0, 0x1234edcc], 80, true);

    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        64,
        true,
    );
    test(&[0, 0xedcbedcb], &[0xabcdabcd, 0x12341234], 64, false);
    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        65,
        false,
    );
    test(
        &[0x54325433, 0xedcbedcb],
        &[0xabcdabcd, 0x12341234],
        128,
        false,
    );
    test(&[0, 0, 0x1234edcc], &[0, 0, 0x12341234], 80, true);
}

#[test]
fn test_eq_mod_power_of_2() {
    let test = |x, y, pow, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .eq_mod_power_of_2(&Integer::from_str(y).unwrap(), pow),
            out
        );
        #[cfg(feature = "32_bit_limbs")]
        assert_eq!(
            rug::Integer::from_str(x)
                .unwrap()
                .is_congruent_2pow(&rug::Integer::from_str(y).unwrap(), Limb::exact_from(pow)),
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

    test("0", "-256", 8, true);
    test("0", "-256", 9, false);
    test("-13", "27", 0, true);
    test("-13", "27", 1, true);
    test("-13", "27", 2, true);
    test("-13", "27", 3, true);
    test("-13", "27", 4, false);
    test("-13", "27", 100, false);
    test("13", "-27", 0, true);
    test("13", "-27", 1, true);
    test("13", "-27", 2, true);
    test("13", "-27", 3, true);
    test("13", "-27", 4, false);
    test("13", "-27", 100, false);
    test("-1000000000001", "4095", 13, true);
    test("-1000000000001", "4095", 14, false);
    test("1000000000001", "-4095", 13, true);
    test("1000000000001", "-4095", 14, false);
    test("4294967295", "-1", 32, true);
    test("-1", "4294967295", 32, true);

    test("-13", "-21", 0, true);
    test("-13", "-21", 1, true);
    test("-13", "-21", 2, true);
    test("-13", "-21", 3, true);
    test("-13", "-21", 4, false);
    test("-13", "-21", 100, false);
    test("-1000000000001", "-1", 12, true);
    test("-1000000000001", "-1", 13, false);
    test("-4294967295", "-4294967295", 32, true);
    test("-281474976710672", "-844424930131984", 49, true);
    test("-281474976710672", "-844424930131984", 50, false);

    test("1311693408901639117", "-17135050664807912499", 64, true);
    test("1311693408901639117", "-17135050663395328000", 64, false);
    test("1311693408901639117", "-17135050664807912499", 65, false);
    test("1311693408901639117", "-17135050664807912499", 128, false);
    test(
        "5633680281231555440641310720",
        "-5634717283396403096794955776",
        80,
        true,
    );

    test("-1311693408901639117", "17135050664807912499", 64, true);
    test("-1311693408901639117", "17135050663395328000", 64, false);
    test("-1311693408901639117", "17135050664807912499", 65, false);
    test("-1311693408901639117", "17135050664807912499", 128, false);
    test(
        "-5633680281231555440641310720",
        "5634717283396403096794955776",
        80,
        true,
    );
}
