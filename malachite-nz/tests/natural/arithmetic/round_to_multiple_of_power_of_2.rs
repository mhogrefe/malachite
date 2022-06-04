use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleByPowerOf2, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_rounding_mode_pair_gen, unsigned_unsigned_rounding_mode_triple_gen_var_3,
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_20,
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_2::{
    limbs_round_to_multiple_of_power_of_2, limbs_round_to_multiple_of_power_of_2_down,
    limbs_round_to_multiple_of_power_of_2_down_in_place,
    limbs_round_to_multiple_of_power_of_2_in_place, limbs_round_to_multiple_of_power_of_2_nearest,
    limbs_round_to_multiple_of_power_of_2_nearest_in_place,
    limbs_round_to_multiple_of_power_of_2_up, limbs_round_to_multiple_of_power_of_2_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_rounding_mode_pair_gen, natural_unsigned_pair_gen_var_10,
    natural_unsigned_pair_gen_var_13, natural_unsigned_pair_gen_var_4,
    natural_unsigned_rounding_mode_triple_gen_var_1,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_down() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_round_to_multiple_of_power_of_2_down(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 1, &[]);
    test(&[], 100, &[]);
    test(&[0, 0, 0], 0, &[0, 0, 0]);
    test(&[0, 0, 0], 1, &[0, 0, 0]);
    test(&[0, 0, 0], 100, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[0]);
    test(&[3], 1, &[2]);
    test(&[122, 456], 1, &[122, 456]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[122, 456]);
    test(&[123, 455], 1, &[122, 455]);
    test(&[123, 456], 31, &[0, 456]);
    test(&[123, 456], 32, &[0, 456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[256, 456]);
    test(&[u32::MAX, 1], 1, &[u32::MAX - 1, 1]);
    test(&[u32::MAX, u32::MAX], 32, &[0, u32::MAX]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_up_in_place() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_round_to_multiple_of_power_of_2_up(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[1], 0, &[1]);
    test(&[1], 1, &[2]);
    test(&[3], 1, &[4]);
    test(&[122, 456], 1, &[122, 456]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[124, 456]);
    test(&[123, 455], 1, &[124, 455]);
    test(&[123, 456], 31, &[0x80000000, 456]);
    test(&[123, 456], 32, &[0, 457]);
    test(&[123, 456], 100, &[0, 0, 0, 16]);
    test(&[256, 456], 8, &[256, 456]);
    test(&[u32::MAX, 1], 1, &[0, 2]);
    test(&[u32::MAX, u32::MAX], 32, &[0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2_nearest() {
    let test = |xs: &[Limb], pow: u64, out: &[Limb]| {
        assert_eq!(limbs_round_to_multiple_of_power_of_2_nearest(xs, pow), out);

        let mut xs = xs.to_vec();
        limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, pow);
        assert_eq!(xs, out);
    };
    test(&[], 0, &[]);
    test(&[], 1, &[]);
    test(&[], 100, &[]);
    test(&[0, 0, 0], 0, &[0, 0, 0]);
    test(&[0, 0, 0], 1, &[0, 0, 0]);
    test(&[0, 0, 0], 100, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[0]);
    test(&[3], 1, &[4]);
    test(&[122, 456], 1, &[122, 456]);
    test(&[123, 456], 0, &[123, 456]);
    test(&[123, 456], 1, &[124, 456]);
    test(&[123, 455], 1, &[124, 455]);
    test(&[123, 456], 31, &[0, 456]);
    test(&[123, 456], 32, &[0, 456]);
    test(&[123, 456], 100, &[]);
    test(&[256, 456], 8, &[256, 456]);
    test(&[u32::MAX, 1], 1, &[0, 2]);
    test(&[u32::MAX, u32::MAX], 32, &[0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_round_to_multiple_of_power_of_2() {
    let test = |xs: &[Limb], pow: u64, rm: RoundingMode, out: Option<Vec<Limb>>| {
        assert_eq!(limbs_round_to_multiple_of_power_of_2(xs, pow, rm), out);

        let mut xs = xs.to_vec();
        if limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, pow, rm) {
            assert_eq!(Some(xs), out);
        } else {
            assert_eq!(None, out)
        }
    };
    test(&[1], 0, RoundingMode::Nearest, Some(vec![1]));
    test(&[1], 1, RoundingMode::Up, Some(vec![2]));
    test(&[3], 1, RoundingMode::Nearest, Some(vec![4]));
    test(&[122, 456], 1, RoundingMode::Floor, Some(vec![122, 456]));
    test(&[123, 456], 0, RoundingMode::Floor, Some(vec![123, 456]));
    test(&[123, 456], 1, RoundingMode::Down, Some(vec![122, 456]));
    test(&[123, 455], 1, RoundingMode::Floor, Some(vec![122, 455]));
    test(
        &[123, 456],
        31,
        RoundingMode::Ceiling,
        Some(vec![0x80000000, 456]),
    );
    test(&[123, 456], 32, RoundingMode::Up, Some(vec![0, 457]));
    test(&[123, 456], 100, RoundingMode::Down, Some(vec![]));
    test(&[256, 456], 8, RoundingMode::Exact, Some(vec![256, 456]));
    test(&[u32::MAX, 1], 1, RoundingMode::Exact, None);
    test(
        &[u32::MAX, u32::MAX],
        32,
        RoundingMode::Down,
        Some(vec![0, u32::MAX]),
    );
}

#[test]
fn test_round_to_multiple_of_power_of_2() {
    let test = |u, v: u64, rm: RoundingMode, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.round_to_multiple_of_power_of_2_assign(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u)
            .unwrap()
            .round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            (Natural::from_str(u).unwrap().shr_round(v, rm) << v).to_string(),
            out
        );
    };
    test("0", 0, RoundingMode::Down, "0");
    test("0", 0, RoundingMode::Up, "0");
    test("0", 0, RoundingMode::Floor, "0");
    test("0", 0, RoundingMode::Ceiling, "0");
    test("0", 0, RoundingMode::Nearest, "0");
    test("0", 0, RoundingMode::Exact, "0");

    test("0", 10, RoundingMode::Down, "0");
    test("0", 10, RoundingMode::Up, "0");
    test("0", 10, RoundingMode::Floor, "0");
    test("0", 10, RoundingMode::Ceiling, "0");
    test("0", 10, RoundingMode::Nearest, "0");
    test("0", 10, RoundingMode::Exact, "0");

    test("123", 0, RoundingMode::Down, "123");
    test("123", 0, RoundingMode::Up, "123");
    test("123", 0, RoundingMode::Floor, "123");
    test("123", 0, RoundingMode::Ceiling, "123");
    test("123", 0, RoundingMode::Nearest, "123");
    test("123", 0, RoundingMode::Exact, "123");

    test("245", 1, RoundingMode::Down, "244");
    test("245", 1, RoundingMode::Up, "246");
    test("245", 1, RoundingMode::Floor, "244");
    test("245", 1, RoundingMode::Ceiling, "246");
    test("245", 1, RoundingMode::Nearest, "244");

    test("246", 1, RoundingMode::Down, "246");
    test("246", 1, RoundingMode::Up, "246");
    test("246", 1, RoundingMode::Floor, "246");
    test("246", 1, RoundingMode::Ceiling, "246");
    test("246", 1, RoundingMode::Nearest, "246");
    test("246", 1, RoundingMode::Exact, "246");

    test("247", 1, RoundingMode::Down, "246");
    test("247", 1, RoundingMode::Up, "248");
    test("247", 1, RoundingMode::Floor, "246");
    test("247", 1, RoundingMode::Ceiling, "248");
    test("247", 1, RoundingMode::Nearest, "248");

    test("491", 2, RoundingMode::Down, "488");
    test("491", 2, RoundingMode::Up, "492");
    test("491", 2, RoundingMode::Floor, "488");
    test("491", 2, RoundingMode::Ceiling, "492");
    test("491", 2, RoundingMode::Nearest, "492");

    test("492", 2, RoundingMode::Down, "492");
    test("492", 2, RoundingMode::Up, "492");
    test("492", 2, RoundingMode::Floor, "492");
    test("492", 2, RoundingMode::Ceiling, "492");
    test("492", 2, RoundingMode::Nearest, "492");
    test("492", 2, RoundingMode::Exact, "492");

    test("493", 2, RoundingMode::Down, "492");
    test("493", 2, RoundingMode::Up, "496");
    test("493", 2, RoundingMode::Floor, "492");
    test("493", 2, RoundingMode::Ceiling, "496");
    test("493", 2, RoundingMode::Nearest, "492");

    test("4127195135", 25, RoundingMode::Down, "4093640704");
    test("4127195135", 25, RoundingMode::Up, "4127195136");
    test("4127195135", 25, RoundingMode::Floor, "4093640704");
    test("4127195135", 25, RoundingMode::Ceiling, "4127195136");
    test("4127195135", 25, RoundingMode::Nearest, "4127195136");

    test("4127195136", 25, RoundingMode::Down, "4127195136");
    test("4127195136", 25, RoundingMode::Up, "4127195136");
    test("4127195136", 25, RoundingMode::Floor, "4127195136");
    test("4127195136", 25, RoundingMode::Ceiling, "4127195136");
    test("4127195136", 25, RoundingMode::Nearest, "4127195136");
    test("4127195136", 25, RoundingMode::Exact, "4127195136");

    test("4127195137", 25, RoundingMode::Down, "4127195136");
    test("4127195137", 25, RoundingMode::Up, "4160749568");
    test("4127195137", 25, RoundingMode::Floor, "4127195136");
    test("4127195137", 25, RoundingMode::Ceiling, "4160749568");
    test("4127195137", 25, RoundingMode::Nearest, "4127195136");

    test("8254390271", 26, RoundingMode::Down, "8187281408");
    test("8254390271", 26, RoundingMode::Up, "8254390272");
    test("8254390271", 26, RoundingMode::Floor, "8187281408");
    test("8254390271", 26, RoundingMode::Ceiling, "8254390272");
    test("8254390271", 26, RoundingMode::Nearest, "8254390272");

    test("8254390272", 26, RoundingMode::Down, "8254390272");
    test("8254390272", 26, RoundingMode::Up, "8254390272");
    test("8254390272", 26, RoundingMode::Floor, "8254390272");
    test("8254390272", 26, RoundingMode::Ceiling, "8254390272");
    test("8254390272", 26, RoundingMode::Nearest, "8254390272");
    test("8254390272", 26, RoundingMode::Exact, "8254390272");

    test("8254390273", 26, RoundingMode::Down, "8254390272");
    test("8254390273", 26, RoundingMode::Up, "8321499136");
    test("8254390273", 26, RoundingMode::Floor, "8254390272");
    test("8254390273", 26, RoundingMode::Ceiling, "8321499136");
    test("8254390273", 26, RoundingMode::Nearest, "8254390272");
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Down,
        "154653373227843986982597791055872",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Up,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Floor,
        "154653373227843986982597791055872",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Ceiling,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261247",
        100,
        RoundingMode::Nearest,
        "155921023828072216384094494261248",
    );

    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Down,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Up,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Floor,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Ceiling,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Nearest,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261248",
        100,
        RoundingMode::Exact,
        "155921023828072216384094494261248",
    );

    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Down,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Up,
        "157188674428300445785591197466624",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Floor,
        "155921023828072216384094494261248",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Ceiling,
        "157188674428300445785591197466624",
    );
    test(
        "155921023828072216384094494261249",
        100,
        RoundingMode::Nearest,
        "155921023828072216384094494261248",
    );

    test("4294967295", 1, RoundingMode::Down, "4294967294");
    test("4294967295", 1, RoundingMode::Up, "4294967296");
    test("4294967295", 1, RoundingMode::Floor, "4294967294");
    test("4294967295", 1, RoundingMode::Ceiling, "4294967296");
    test("4294967295", 1, RoundingMode::Nearest, "4294967296");

    test("4294967296", 1, RoundingMode::Down, "4294967296");
    test("4294967296", 1, RoundingMode::Up, "4294967296");
    test("4294967296", 1, RoundingMode::Floor, "4294967296");
    test("4294967296", 1, RoundingMode::Ceiling, "4294967296");
    test("4294967296", 1, RoundingMode::Nearest, "4294967296");
    test("4294967296", 1, RoundingMode::Exact, "4294967296");

    test("4294967297", 1, RoundingMode::Down, "4294967296");
    test("4294967297", 1, RoundingMode::Up, "4294967298");
    test("4294967297", 1, RoundingMode::Floor, "4294967296");
    test("4294967297", 1, RoundingMode::Ceiling, "4294967298");
    test("4294967297", 1, RoundingMode::Nearest, "4294967296");

    test("1000000000000", 0, RoundingMode::Down, "1000000000000");
    test("1000000000000", 0, RoundingMode::Up, "1000000000000");
    test("1000000000000", 0, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 0, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 0, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 0, RoundingMode::Exact, "1000000000000");

    test("7999999999999", 3, RoundingMode::Down, "7999999999992");
    test("7999999999999", 3, RoundingMode::Up, "8000000000000");
    test("7999999999999", 3, RoundingMode::Floor, "7999999999992");
    test("7999999999999", 3, RoundingMode::Ceiling, "8000000000000");
    test("7999999999999", 3, RoundingMode::Nearest, "8000000000000");

    test("8000000000000", 3, RoundingMode::Down, "8000000000000");
    test("8000000000000", 3, RoundingMode::Up, "8000000000000");
    test("8000000000000", 3, RoundingMode::Floor, "8000000000000");
    test("8000000000000", 3, RoundingMode::Ceiling, "8000000000000");
    test("8000000000000", 3, RoundingMode::Nearest, "8000000000000");
    test("8000000000000", 3, RoundingMode::Exact, "8000000000000");

    test("8000000000001", 3, RoundingMode::Down, "8000000000000");
    test("8000000000001", 3, RoundingMode::Up, "8000000000008");
    test("8000000000001", 3, RoundingMode::Floor, "8000000000000");
    test("8000000000001", 3, RoundingMode::Ceiling, "8000000000008");
    test("8000000000001", 3, RoundingMode::Nearest, "8000000000000");

    test(
        "16777216000000000000",
        24,
        RoundingMode::Down,
        "16777216000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Up,
        "16777216000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Floor,
        "16777216000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Ceiling,
        "16777216000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Nearest,
        "16777216000000000000",
    );
    test(
        "16777216000000000000",
        24,
        RoundingMode::Exact,
        "16777216000000000000",
    );

    test(
        "33554432000000000000",
        25,
        RoundingMode::Down,
        "33554432000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Up,
        "33554432000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Floor,
        "33554432000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Ceiling,
        "33554432000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Nearest,
        "33554432000000000000",
    );
    test(
        "33554432000000000000",
        25,
        RoundingMode::Exact,
        "33554432000000000000",
    );

    test(
        "2147483648000000000000",
        31,
        RoundingMode::Down,
        "2147483648000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Up,
        "2147483648000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Floor,
        "2147483648000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Ceiling,
        "2147483648000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Nearest,
        "2147483648000000000000",
    );
    test(
        "2147483648000000000000",
        31,
        RoundingMode::Exact,
        "2147483648000000000000",
    );

    test(
        "4294967296000000000000",
        32,
        RoundingMode::Down,
        "4294967296000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Up,
        "4294967296000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Floor,
        "4294967296000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Ceiling,
        "4294967296000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Nearest,
        "4294967296000000000000",
    );
    test(
        "4294967296000000000000",
        32,
        RoundingMode::Exact,
        "4294967296000000000000",
    );

    test(
        "8589934592000000000000",
        33,
        RoundingMode::Down,
        "8589934592000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Up,
        "8589934592000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Floor,
        "8589934592000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Ceiling,
        "8589934592000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Nearest,
        "8589934592000000000000",
    );
    test(
        "8589934592000000000000",
        33,
        RoundingMode::Exact,
        "8589934592000000000000",
    );

    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Down,
        "1267650600228229401496703205376000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Up,
        "1267650600228229401496703205376000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Floor,
        "1267650600228229401496703205376000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Ceiling,
        "1267650600228229401496703205376000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Nearest,
        "1267650600228229401496703205376000000000000",
    );
    test(
        "1267650600228229401496703205376000000000000",
        100,
        RoundingMode::Exact,
        "1267650600228229401496703205376000000000000",
    );

    test("1000000000000", 10, RoundingMode::Down, "1000000000000");
    test("1000000000000", 10, RoundingMode::Up, "1000000000000");
    test("1000000000000", 10, RoundingMode::Floor, "1000000000000");
    test("1000000000000", 10, RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", 10, RoundingMode::Nearest, "1000000000000");
    test("1000000000000", 10, RoundingMode::Exact, "1000000000000");

    test("980657949", 72, RoundingMode::Down, "0");
    test("980657949", 72, RoundingMode::Up, "4722366482869645213696");
    test("980657949", 72, RoundingMode::Floor, "0");
    test(
        "980657949",
        72,
        RoundingMode::Ceiling,
        "4722366482869645213696",
    );
    test("980657949", 72, RoundingMode::Nearest, "0");

    test("4294967295", 31, RoundingMode::Down, "2147483648");
    test("4294967295", 31, RoundingMode::Up, "4294967296");
    test("4294967295", 31, RoundingMode::Floor, "2147483648");
    test("4294967295", 31, RoundingMode::Ceiling, "4294967296");
    test("4294967295", 31, RoundingMode::Nearest, "4294967296");

    test("4294967295", 32, RoundingMode::Down, "0");
    test("4294967295", 32, RoundingMode::Up, "4294967296");
    test("4294967295", 32, RoundingMode::Floor, "0");
    test("4294967295", 32, RoundingMode::Ceiling, "4294967296");
    test("4294967295", 32, RoundingMode::Nearest, "4294967296");

    test("4294967296", 32, RoundingMode::Down, "4294967296");
    test("4294967296", 32, RoundingMode::Up, "4294967296");
    test("4294967296", 32, RoundingMode::Floor, "4294967296");
    test("4294967296", 32, RoundingMode::Ceiling, "4294967296");
    test("4294967296", 32, RoundingMode::Nearest, "4294967296");
    test("4294967296", 32, RoundingMode::Exact, "4294967296");

    test("4294967296", 33, RoundingMode::Down, "0");
    test("4294967296", 33, RoundingMode::Up, "8589934592");
    test("4294967296", 33, RoundingMode::Floor, "0");
    test("4294967296", 33, RoundingMode::Ceiling, "8589934592");
    test("4294967296", 33, RoundingMode::Nearest, "0");
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Natural::from(123u32).round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Natural::from(123u32).round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Natural::from(123u32).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Natural::from(123u32).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Natural::from_str("1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Natural::from(123u32)).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Natural::from(123u32)).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Natural::from_str("1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Natural::from_str("1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_down(&xs, pow)),
            Natural::from_owned_limbs_asc(xs) >> pow << pow
        );
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_up(&xs, pow)),
            Natural::from_owned_limbs_asc(xs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
        );
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(&config, |(xs, pow)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_round_to_multiple_of_power_of_2_nearest(&xs, pow)),
            Natural::from_owned_limbs_asc(xs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest)
        );
    });
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(xs, pow, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            limbs_round_to_multiple_of_power_of_2(&xs, pow, rm).map_or_else(
                || {
                    assert_eq!(rm, RoundingMode::Exact);
                    assert!(!n.divisible_by_power_of_2(pow));
                },
                |result_limbs| {
                    let m = (&n).round_to_multiple_of_power_of_2(pow, rm);
                    assert_eq!(Natural::from_owned_limbs_asc(result_limbs), m);
                    if rm == RoundingMode::Exact {
                        assert_eq!(m, n);
                    }
                },
            );
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_down_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, pow);
            let n = Natural::from_owned_limbs_asc(old_xs) >> pow << pow;
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_up_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, pow);
            let n = Natural::from_owned_limbs_asc(old_xs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Up);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_nearest_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_16().test_properties_with_config(
        &config,
        |(mut xs, pow)| {
            let old_xs = xs.clone();
            limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, pow);
            let n = Natural::from_owned_limbs_asc(old_xs)
                .round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
        },
    );
}

#[test]
fn limbs_round_to_multiple_of_power_of_2_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties_with_config(
        &config,
        |(mut xs, pow, rm)| {
            let n = Natural::from_limbs_asc(&xs);
            if limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, pow, rm) {
                let m = (&n).round_to_multiple_of_power_of_2(pow, rm);
                assert_eq!(Natural::from_owned_limbs_asc(xs), m);
                if rm == RoundingMode::Exact {
                    assert_eq!(m, n);
                }
            } else {
                assert_eq!(rm, RoundingMode::Exact);
                assert!(!n.divisible_by_power_of_2(pow));
            }
        },
    );
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    natural_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let r = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert!(r.is_valid());

        let r_alt = n.clone().round_to_multiple_of_power_of_2(pow, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        let mut mut_n = n.clone();
        mut_n.round_to_multiple_of_power_of_2_assign(pow, rm);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        assert!(r.divisible_by_power_of_2(pow));
        assert_eq!((&n).shr_round(pow, rm) << pow, r);
        assert!((Integer::from(&r) - Integer::from(&n)).abs() <= Natural::power_of_2(pow));
        assert_eq!((&n).round_to_multiple(Natural::power_of_2(pow), rm), r);
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert!(r <= n),
            RoundingMode::Ceiling | RoundingMode::Up => assert!(r >= n),
            RoundingMode::Exact => assert_eq!(r, n),
            RoundingMode::Nearest => {
                let k = Natural::power_of_2(pow);
                let closest;
                let second_closest;
                if r <= n {
                    closest = &n - &r;
                    second_closest = &r + k - n;
                } else {
                    closest = &r - &n;
                    second_closest = n + k - &r;
                }
                assert!(closest <= second_closest);
                if closest == second_closest {
                    assert!(!r.get_bit(pow));
                }
            }
        }
    });

    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(n, pow)| {
        let shifted = n << pow;
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
            shifted
        );
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
            shifted
        );
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
            shifted
        );
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            shifted
        );
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
            shifted
        );
        assert_eq!(
            (&shifted).round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
            shifted
        );
    });

    natural_unsigned_pair_gen_var_10().test_properties(|(n, pow)| {
        let down = (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Down);
        let up = &down + Natural::power_of_2(pow);
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
            up
        );
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
            down
        );
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            up
        );
        let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    natural_unsigned_pair_gen_var_13::<u64>().test_properties(|(n, pow)| {
        if let Some(shift) = pow.checked_add(n.significant_bits()) {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(shift, RoundingMode::Down),
                0
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(shift, RoundingMode::Floor),
                0
            );
            if let Some(extra_shift) = shift.checked_add(1) {
                assert_eq!(
                    n.round_to_multiple_of_power_of_2(extra_shift, RoundingMode::Nearest),
                    0
                );
            }
        }
    });

    natural_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!((&n).round_to_multiple_of_power_of_2(0, rm), n);
    });

    unsigned_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(Natural::ZERO.round_to_multiple_of_power_of_2(pow, rm), 0);
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_3::<Limb>().test_properties(|(n, pow, rm)| {
        assert_eq!(
            n.round_to_multiple_of_power_of_2(pow, rm),
            Natural::from(n).round_to_multiple_of_power_of_2(pow, rm)
        );
    });
}
