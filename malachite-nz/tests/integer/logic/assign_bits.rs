use std::str::FromStr;

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::_assign_bits_naive;

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
    _assign_bits_naive::<Integer, Natural>(&mut n, start, end, &bits);
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
    test(
        &[123],
        64,
        128,
        &[456],
        &[123, 0, 4_294_966_839, 0xffff_ffff],
    );
    test(&[123], 80, 100, &[456], &[123, 0, 4_265_017_344, 15]);
    test(
        &[123, 456],
        80,
        100,
        &[789, 321],
        &[123, 456, 4_243_193_856, 15],
    );
    test(
        &[1_619_367_413, 294_928_230],
        73,
        89,
        &[
            4_211_621_339,
            3_627_566_573,
            1_208_090_001,
            4_045_783_696,
            2_932_656_682,
            177_881_999,
            898_588_654,
        ],
        &[1_619_367_413, 294_928_230, 25_446_400],
    );
    test(
        &[
            1_404_969_050,
            495_263_765,
            2_378_891_263,
            1_299_524_786,
            1_654_909_014,
            2_724_647_948,
        ],
        21,
        32,
        &[
            3_269_073_749,
            1_170_977_875,
            2_823_122_906,
            144_832_001,
            3_738_801_070,
            1_107_604_886,
            4_260_406_413,
            1_766_163_855,
            592_730_267,
            484_513_503,
            1_204_041_536,
            3_664_297_641,
        ],
        &[
            2_505_973_850,
            495_263_765,
            2_378_891_263,
            1_299_524_786,
            1_654_909_014,
            2_724_647_948,
        ],
    );
    test(
        &[
            4_126_931_041,
            1_467_617_913,
            1_718_397_261,
            904_474_857,
            312_429_577,
            2_397_873_671,
            3_967_827_549,
            3_842_236_128,
            3_414_636_734,
            1_846_949_256,
            1_999_024_107,
            424_639_176,
        ],
        27,
        77,
        &[977_841_009],
        &[
            1_979_447_393,
            4_264_409_764,
            1_718_403_071,
            904_474_857,
            312_429_577,
            2_397_873_671,
            3_967_827_549,
            3_842_236_128,
            3_414_636_734,
            1_846_949_256,
            1_999_024_107,
            424_639_176,
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
        _assign_bits_naive(&mut n, start, end, &Natural::from_str(v).unwrap());
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
