use std::str::FromStr;

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;

use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_neg_assign_bits(xs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]) {
    let old_n = -Natural::from_limbs_asc(xs);
    let mut n = old_n.clone();
    let bits = Natural::from_limbs_asc(bits);
    n.assign_bits(start, end, &bits);
    let result = n;
    assert_eq!(-Natural::from_limbs_asc(out), result);
    let mut n = old_n;
    assign_bits_naive::<Integer, Natural>(&mut n, start, end, &bits);
    assert_eq!(n, result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_assign_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]| {
        let mut limbs = xs.to_vec();
        limbs_neg_assign_bits(&mut limbs, start, end, bits);
        assert_eq!(limbs, out);
        verify_limbs_neg_assign_bits(xs, start, end, bits, out);
    };
    test(&[1], 0, 1, &[1], &[1]);
    test(&[1], 1, 2, &[1], &[1]);
    test(&[1], 0, 1, &[0, 1], &[2]);
    test(&[123], 64, 128, &[456], &[123, 0, 4294966839, u32::MAX]);
    test(&[123], 80, 100, &[456], &[123, 0, 4265017344, 15]);
    test(
        &[123, 456],
        80,
        100,
        &[789, 321],
        &[123, 456, 4243193856, 15],
    );
    test(
        &[1619367413, 294928230],
        73,
        89,
        &[
            4211621339, 3627566573, 1208090001, 4045783696, 2932656682, 177881999, 898588654,
        ],
        &[1619367413, 294928230, 25446400],
    );
    test(
        &[
            1404969050, 495263765, 2378891263, 1299524786, 1654909014, 2724647948,
        ],
        21,
        32,
        &[
            3269073749, 1170977875, 2823122906, 144832001, 3738801070, 1107604886, 4260406413,
            1766163855, 592730267, 484513503, 1204041536, 3664297641,
        ],
        &[
            2505973850, 495263765, 2378891263, 1299524786, 1654909014, 2724647948,
        ],
    );
    test(
        &[
            4126931041, 1467617913, 1718397261, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
        ],
        27,
        77,
        &[977841009],
        &[
            1979447393, 4264409764, 1718403071, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
        ],
    );
    test(&[123, 456], 0, 100, &[], &[0, 0, 0, 16]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_1() {
    let mut xs = vec![123];
    limbs_neg_assign_bits(&mut xs, 10, 5, &[456]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_2() {
    let mut xs = vec![123];
    limbs_neg_assign_bits(&mut xs, 10, 10, &[456]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_3() {
    let mut xs = vec![];
    limbs_neg_assign_bits(&mut xs, 10, 10, &[456]);
}

#[test]
fn test_assign_bits() {
    let test = |u, start, end, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.assign_bits(start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Integer::from_str(out).unwrap());
        let mut n = Integer::from_str(u).unwrap();
        assign_bits_naive(&mut n, start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Integer::from_str(out).unwrap());
    };
    test("123", 10, 10, "456", "123");
    test("123", 5, 7, "456", "27");
    test("123", 64, 128, "456", "8411715297611555537019");
    test("123", 80, 100, "456", "551270173744270903666016379");
    test(
        "1000000000000",
        80,
        100,
        "456",
        "551270173744271903666016256",
    );
    test(
        "456",
        80,
        100,
        "1000000000000",
        "401092572728463209067316249032",
    );
    test(
        "1000000000000",
        80,
        100,
        "2000000000000",
        "802185145456926419134632497152",
    );

    test("-123", 10, 10, "456", "-123");
    test("-123", 5, 7, "456", "-123");
    test(
        "-123",
        64,
        128,
        "456",
        "-340282366920938455033212565746503123067",
    );
    test("-123", 80, 100, "456", "-1267098121128665515963862483067");
    test(
        "-1000000000000",
        80,
        100,
        "456",
        "-1267098121128665516963862482944",
    );
    test(
        "-456",
        80,
        100,
        "1000000000000",
        "-866556818573946577800212251080",
    );
    test(
        "-1000000000000",
        80,
        100,
        "2000000000000",
        "-465464245845483369732896002048",
    );
    test("-123", 0, 100, "0", "-1267650600228229401496703205376");
}

#[test]
#[should_panic]
fn assign_bits_fail() {
    let mut n = Integer::from(123u32);
    n.assign_bits(10, 5, &Natural::from(456u32));
}
