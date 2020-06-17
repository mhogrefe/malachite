use std::str::FromStr;

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;

use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_assign_bits(xs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]) {
    let old_n = Natural::from_limbs_asc(xs);
    let mut n = old_n.clone();
    let bits = Natural::from_limbs_asc(bits);
    n.assign_bits(start, end, &bits);
    let result = n;
    assert_eq!(Natural::from_limbs_asc(out), result);
    let mut n = old_n;
    assign_bits_naive::<Natural, Natural>(&mut n, start, end, &bits);
    assert_eq!(n, result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_assign_bits() {
    let test = |xs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]| {
        let xs_old = xs;
        let mut xs = xs.to_vec();
        limbs_assign_bits(&mut xs, start, end, bits);
        assert_eq!(xs, out);
        verify_limbs_assign_bits(xs_old, start, end, bits, out);
    };
    // bits_limb_width >= bits.len()
    // end_limb <= limbs.len()
    // xs_len <= ys_len in copy_from_diff_len_slice
    // start_remainder == 0
    // end_remainder != 0
    test(&[1], 0, 1, &[1], &[1]);
    // start_remainder != 0
    test(&[1], 1, 2, &[1], &[3]);
    // bits_limb_width < bits.len()
    test(&[1], 0, 1, &[0, 1], &[0]);
    test(&[123], 64, 128, &[456], &[123, 0, 456, 0]);
    test(&[123], 80, 100, &[456], &[123, 0, 29_884_416, 0]);
    test(
        &[123, 456],
        80,
        100,
        &[789, 321],
        &[123, 456, 51_707_904, 0],
    );
    // end_limb > limbs.len()
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
        &[1_619_367_413, 294_928_230, 8_107_520],
    );
    // end_remainder == 0
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
            1_790_845_018,
            495_263_765,
            2_378_891_263,
            1_299_524_786,
            1_654_909_014,
            2_724_647_948,
        ],
    );
    // xs_len > ys_len in copy_from_diff_len_slice
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
            2_382_100_577,
            30_557_531,
            1_718_394_880,
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
}

#[test]
#[should_panic]
fn limbs_assign_bits_fail_1() {
    let mut xs = vec![123];
    limbs_assign_bits(&mut xs, 10, 5, &[456]);
}

#[test]
#[should_panic]
fn limbs_assign_bits_fail_2() {
    let mut xs = vec![123];
    limbs_assign_bits(&mut xs, 10, 10, &[456]);
}

#[test]
fn test_assign_bits() {
    let test = |u, start, end, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.assign_bits(start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Natural::from_str(out).unwrap());
        let mut n = Natural::from_str(u).unwrap();
        assign_bits_naive(&mut n, start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Natural::from_str(out).unwrap());
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
}

#[test]
#[should_panic]
fn assign_bits_fail() {
    let mut n = Natural::from(123u32);
    n.assign_bits(10, 5, &Natural::from(456u32));
}
