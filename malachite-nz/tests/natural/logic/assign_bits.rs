use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;
use std::str::FromStr;

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
    test(&[123], 80, 100, &[456], &[123, 0, 29884416, 0]);
    test(&[123, 456], 80, 100, &[789, 321], &[123, 456, 51707904, 0]);
    // end_limb > limbs.len()
    test(
        &[1619367413, 294928230],
        73,
        89,
        &[
            4211621339, 3627566573, 1208090001, 4045783696, 2932656682, 177881999, 898588654,
        ],
        &[1619367413, 294928230, 8107520],
    );
    // end_remainder == 0
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
            1790845018, 495263765, 2378891263, 1299524786, 1654909014, 2724647948,
        ],
    );
    // xs_len > ys_len in copy_from_diff_len_slice
    test(
        &[
            4126931041, 1467617913, 1718397261, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
        ],
        27,
        77,
        &[977841009],
        &[
            2382100577, 30557531, 1718394880, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
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
