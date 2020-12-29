#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::conversion::digits::general_digits::{
    _limbs_to_digits_asc_basecase, _to_digits_asc_naive,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_to_digits_asc_basecase(
    original_out: &[u8],
    len: usize,
    xs: &[Limb],
    base: u64,
    out_len: usize,
    out: &[u8],
) {
    if len != 0 {
        assert_eq!(len, out_len);
    }
    let digits = _to_digits_asc_naive::<u8, _>(&Natural::from_limbs_asc(xs), base);
    let mut expected_digits = vec![0; out_len];
    expected_digits[..digits.len()].copy_from_slice(&digits);
    expected_digits.reverse();
    assert_eq!(&out[..out_len], expected_digits);
    assert_eq!(&out[out_len..], &original_out[out_len..]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_to_digits_asc_basecase() {
    fn test(out_before: &[u8], len: usize, xs: &[Limb], base: u64, out_after: &[u8]) {
        let mut out = out_before.to_vec();
        let out_len = _limbs_to_digits_asc_basecase(&mut out, len, xs, base);
        assert_eq!(&out[..out_len], out_after);
        verify_limbs_to_digits_asc_basecase(out_before, len, xs, base, out_len, &out);
    };
    test(&[0; 20], 0, &[], 9, &[]);
    // base != 10
    test(&[0; 20], 0, &[1], 9, &[1]);
    test(
        &[0; 20],
        0,
        &[123456],
        3,
        &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0],
    );
    test(&[0; 20], 0, &[123456], 5, &[1, 2, 4, 2, 2, 3, 1, 1]);
    test(&[0; 20], 0, &[123456], 6, &[2, 3, 5, 1, 3, 2, 0]);
    test(&[0; 20], 0, &[123456], 7, &[1, 0, 2, 2, 6, 3, 4]);
    test(&[0; 20], 0, &[123456], 9, &[2, 0, 7, 3, 1, 3]);
    // base == 10
    test(&[0; 20], 0, &[123456], 10, &[1, 2, 3, 4, 5, 6]);
    test(&[0; 20], 0, &[123456], 11, &[8, 4, 8, 3, 3]);
    test(&[0; 20], 0, &[123456], 12, &[5, 11, 5, 4, 0]);
    test(&[0; 20], 0, &[123456], 13, &[4, 4, 2, 6, 8]);
    test(&[0; 20], 0, &[123456], 14, &[3, 2, 13, 12, 4]);
    test(&[0; 20], 0, &[123456], 15, &[2, 6, 8, 10, 6]);
    test(&[0; 20], 0, &[123456], 100, &[12, 34, 56]);
    test(&[0; 20], 0, &[123456], 123, &[8, 19, 87]);
    test(&[0; 20], 0, &[123456], 255, &[1, 229, 36]);
    // base != 10 && xs_len > 1
    test(
        &[0; 40],
        0,
        &[123456, 789012],
        5,
        &[
            1, 2, 0, 2, 3, 1, 3, 3, 2, 4, 0, 4, 2, 1, 4, 4, 1, 3, 0, 0, 0, 1, 3,
        ],
    );
    // base == 10 && xs_len > 1
    test(
        &[0; 40],
        0,
        &[123456, 789012],
        10,
        &[3, 3, 8, 8, 7, 8, 0, 7, 3, 6, 2, 7, 5, 0, 0, 8],
    );
    test(
        &[0; 40],
        0,
        &[123456, 789012],
        123,
        &[7, 117, 75, 111, 16, 62, 88, 96],
    );
    test(
        &[0; 40],
        0,
        &[123456, 789012],
        255,
        &[12, 82, 251, 166, 147, 176, 78],
    );

    // zero_len != 0
    test(&[0; 20], 8, &[123456], 9, &[0, 0, 2, 0, 7, 3, 1, 3]);
    test(&[0; 20], 8, &[123456], 10, &[0, 0, 1, 2, 3, 4, 5, 6]);
}
