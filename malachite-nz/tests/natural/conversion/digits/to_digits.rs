use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::slice_leading_zeros;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_nz::natural::conversion::digits::general_digits::{
    PowerTableAlgorithm, _limbs_to_digits_small_base, _limbs_to_digits_small_base_basecase,
    _to_digits_asc_naive,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::generators::*;

fn verify_limbs_to_digits_small_base_basecase(
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

    let result = out;
    let mut out = original_out.to_vec();
    let mut xs = xs.to_vec();
    let out_len_alt = _limbs_to_digits_small_base(&mut out, base, &mut xs, None);
    let sig_out = &result[..out_len];
    let sig_out_alt = &out[..out_len_alt];
    assert_eq!(
        &sig_out[slice_leading_zeros(sig_out)..out_len],
        &sig_out_alt[slice_leading_zeros(sig_out_alt)..out_len_alt]
    );
}

fn verify_limbs_to_digits_small_base(
    original_out: &[u8],
    original_xs: &[Limb],
    base: u64,
    out_len: usize,
    out: &[u8],
) {
    let digits = _to_digits_asc_naive::<u8, _>(&Natural::from_limbs_asc(original_xs), base);
    let mut expected_digits = vec![0; out_len];
    expected_digits[..digits.len()].copy_from_slice(&digits);
    expected_digits.reverse();
    assert_eq!(&out[..out_len], expected_digits);
    assert_eq!(&out[out_len..], &original_out[out_len..]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_to_digits_small_base_basecase() {
    fn test(out_before: &[u8], len: usize, xs: &[Limb], base: u64, out_after: &[u8]) {
        let mut out = out_before.to_vec();
        let out_len = _limbs_to_digits_small_base_basecase(&mut out, len, xs, base);
        assert_eq!(&out[..out_len], out_after);
        verify_limbs_to_digits_small_base_basecase(out_before, len, xs, base, out_len, &out);
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

#[test]
fn limbs_to_digits_small_base_basecase_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1().test_properties_with_config(
        &config,
        |(mut out, len, xs, base)| {
            let old_out = out.clone();
            let out_len = _limbs_to_digits_small_base_basecase(&mut out, len, &xs, base);
            verify_limbs_to_digits_small_base_basecase(&old_out, len, &xs, base, out_len, &out);
        },
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_to_digits_small_base() {
    fn test(out_before: &[u8], xs: &[Limb], base: u64, out_after: &[u8]) {
        let mut out = out_before.to_vec();
        let mut mut_xs = xs.to_vec();
        let out_len = _limbs_to_digits_small_base(&mut out, base, &mut mut_xs, None);
        assert_eq!(&out[..out_len], out_after);
        verify_limbs_to_digits_small_base(out_before, xs, base, out_len, &out);
    };
    // xs_len == 0
    test(&[0; 20], &[], 9, &[0]);
    // 0 < xs_len < GET_STR_PRECOMPUTE_THRESHOLD
    test(&[0; 20], &[1], 9, &[1]);
    test(&[0; 20], &[123456], 3, &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]);
    test(&[0; 20], &[123456], 5, &[1, 2, 4, 2, 2, 3, 1, 1]);
    test(&[0; 20], &[123456], 6, &[2, 3, 5, 1, 3, 2, 0]);
    test(&[0; 20], &[123456], 7, &[1, 0, 2, 2, 6, 3, 4]);
    test(&[0; 20], &[123456], 9, &[2, 0, 7, 3, 1, 3]);
    test(&[0; 20], &[123456], 10, &[1, 2, 3, 4, 5, 6]);
    test(&[0; 20], &[123456], 11, &[8, 4, 8, 3, 3]);
    test(&[0; 20], &[123456], 12, &[5, 11, 5, 4, 0]);
    test(&[0; 20], &[123456], 13, &[4, 4, 2, 6, 8]);
    test(&[0; 20], &[123456], 14, &[3, 2, 13, 12, 4]);
    test(&[0; 20], &[123456], 15, &[2, 6, 8, 10, 6]);
    test(&[0; 20], &[123456], 100, &[12, 34, 56]);
    test(&[0; 20], &[123456], 123, &[8, 19, 87]);
    test(&[0; 20], &[123456], 255, &[1, 229, 36]);
    test(
        &[0; 40],
        &[123456, 789012],
        5,
        &[
            1, 2, 0, 2, 3, 1, 3, 3, 2, 4, 0, 4, 2, 1, 4, 4, 1, 3, 0, 0, 0, 1, 3,
        ],
    );
    test(
        &[0; 40],
        &[123456, 789012],
        10,
        &[3, 3, 8, 8, 7, 8, 0, 7, 3, 6, 2, 7, 5, 0, 0, 8],
    );
    test(
        &[0; 40],
        &[123456, 789012],
        123,
        &[7, 117, 75, 111, 16, 62, 88, 96],
    );
    test(
        &[0; 40],
        &[123456, 789012],
        255,
        &[12, 82, 251, 166, 147, 176, 78],
    );
    // xs_len >= GET_STR_PRECOMPUTE_THRESHOLD
    // power != 1 in _limbs_choose_power_table_algorithm
    // number_of_powers > 1 in _limbs_choose_power_table_algorithm
    // pow.odd() in _limbs_choose_power_table_algorithm
    // n != pow << (i - 1) in _limbs_choose_power_table_algorithm
    // pow.even() in _limbs_choose_power_table_algorithm
    // n == pow << (i - 1) in _limbs_choose_power_table_algorithm
    // n == pow << (i - 1) && pow.odd() in _limbs_choose_power_table_algorithm
    // mul_cost > div_cost in _limbs_choose_power_table_algorithm
    // number_of_powers > 0 in _limbs_compute_power_table_using_div
    // digits_in_base == exp in _limbs_compute_power_table_using_div
    // digits_in_base != exp in _limbs_compute_power_table_using_div
    // remainder[adjust] == 0
    //      && remainder[adjust + 1].divisible_by_power_of_two(big_base_trailing_zeros)
    //      in _limbs_compute_power_table_using_div
    // xs_len >= GET_STR_DC_THRESHOLD in _limbs_to_digits_small_base_divide_and_conquer
    // xs_len > total_len || xs_len == total_len
    //      && limbs_cmp_same_length(&xs[shift..], power.power) == Ordering::Less
    //      in _limbs_to_digits_small_base_divide_and_conquer
    // len == 0 in _limbs_to_digits_small_base_divide_and_conquer
    // xs_len < GET_STR_DC_THRESHOLD in _limbs_to_digits_small_base_divide_and_conquer
    // xs_len != 0 in _limbs_to_digits_small_base_divide_and_conquer
    // xs_len >= GET_STR_DC_THRESHOLD && len != 0
    test(
        &[0; 180],
        &[
            3056215344, 3478498987, 1525628527, 2940636569, 2793889044, 628858201, 4120826843,
            1202551139, 2048559663, 3875755114, 2364980673, 2383732604, 3991426155, 229530160,
            2263981142, 3298086542, 1508261462, 3566023571, 3747437734, 2439671349, 1387876207,
            4019823972, 2986550141, 1187172695, 3025136315, 1183784222, 4211004667, 564539785,
            1644122167, 999423211, 806938404, 10860543, 3458492920, 1199271248, 1169727513,
            2600131941, 1298866326,
        ],
        104,
        &[
            7, 102, 41, 15, 79, 28, 23, 19, 82, 89, 32, 45, 23, 94, 68, 1, 5, 96, 34, 31, 47, 91,
            9, 77, 70, 30, 32, 44, 67, 88, 51, 17, 18, 55, 66, 74, 61, 95, 51, 9, 76, 7, 12, 75,
            82, 103, 16, 83, 92, 84, 48, 98, 79, 55, 50, 30, 40, 36, 56, 86, 60, 54, 92, 39, 68,
            26, 46, 76, 15, 25, 98, 43, 62, 102, 40, 94, 59, 55, 52, 77, 52, 46, 18, 28, 12, 23,
            49, 9, 30, 64, 53, 49, 39, 51, 51, 26, 3, 68, 82, 13, 35, 58, 44, 88, 58, 85, 63, 101,
            42, 103, 20, 55, 48, 10, 64, 21, 48, 48, 10, 58, 5, 78, 48, 69, 20, 76, 63, 67, 50, 96,
            39, 12, 47, 23, 6, 39, 17, 95, 40, 15, 3, 32, 68, 95, 82, 10, 86, 47, 53, 59, 74, 90,
            47, 81, 3, 45, 0, 76, 37, 42, 61, 48, 62, 82, 64, 72, 87, 37, 56, 56, 32, 88, 11, 29,
            52, 74, 32,
        ],
    );
    // n == pow << (i - 1) && pow.even() in _limbs_choose_power_table_algorithm
    test(
        &[0; 180],
        &[
            4069654318, 200234362, 1446122636, 2556884733, 1171369997, 2416514207, 1914789404,
            1066230418, 3700758050, 369490702, 4239134808, 1298432969, 1334078642, 1406451364,
            1566734589, 43717764, 349561564, 1067107870, 957081235, 2721095071, 134596014,
            1764968880, 2804491477, 129578595, 3283664828, 3844511094, 823049706, 3918883322,
            4090685182, 2698902066, 3293373129, 2585973756, 1955397356, 2047755454, 2010607731,
            254977406,
        ],
        98,
        &[
            11, 94, 75, 18, 50, 96, 56, 88, 85, 90, 66, 93, 13, 88, 39, 25, 83, 68, 29, 41, 42, 96,
            90, 41, 59, 94, 15, 83, 29, 33, 61, 34, 20, 3, 95, 79, 36, 55, 65, 18, 5, 32, 85, 65,
            66, 68, 86, 97, 17, 15, 45, 77, 91, 86, 25, 25, 29, 88, 40, 49, 31, 65, 77, 32, 67, 24,
            30, 68, 32, 71, 77, 10, 14, 33, 78, 67, 70, 79, 32, 48, 4, 13, 12, 38, 21, 72, 31, 89,
            96, 14, 90, 76, 40, 85, 8, 42, 9, 17, 58, 88, 97, 2, 61, 27, 41, 88, 66, 96, 27, 57,
            34, 23, 26, 29, 76, 13, 69, 17, 77, 54, 36, 42, 60, 48, 27, 66, 80, 15, 9, 10, 89, 85,
            21, 73, 48, 15, 8, 83, 75, 42, 97, 73, 49, 81, 11, 83, 41, 65, 82, 92, 50, 81, 27, 80,
            19, 31, 58, 11, 63, 52, 58, 89, 56, 15, 19, 1, 88, 97, 54, 92, 60, 81, 85, 64,
        ],
    );
    // mul_cost <= div_cost in _limbs_choose_power_table_algorithm
    // exponents[0] != chars_per_limb << number_of_powers in _limbs_compute_power_table_using_mul
    // (digits_in_base + chars_per_limb) << (power_len - 2) > exponents[0]
    //      in _limbs_compute_power_table_using_mul
    // (digits_in_base + chars_per_limb) << i > exponents[0]
    //      in _limbs_compute_power_table_using_mul
    // row.digits_in_base < exponent in _limbs_compute_power_table_using_mul
    // (digits_in_base + chars_per_limb) << i <= exponents[0]
    //      in _limbs_compute_power_table_using_mul
    // row.digits_in_base >= exponent in _limbs_compute_power_table_using_mul
    test(
        &[0; 150],
        &[
            2679239519, 721774729, 553558153, 2694879530, 315361326, 1002777083, 3532473858,
            3891803964, 3091255938, 1810962291, 792542145, 3504464685, 3414416050, 3265802575,
            165631340, 3322240994, 1491277551, 2663783343, 3865601021, 953928172, 851798883,
            3314281119, 2412275729, 3065107875, 530046998, 3405323797, 3741488431, 151251893,
            3569252307, 689124400, 3633309617, 1796271003, 2766831787,
        ],
        185,
        &[
            1, 177, 172, 157, 179, 24, 121, 151, 101, 53, 20, 0, 32, 16, 183, 70, 103, 158, 81, 44,
            98, 4, 131, 48, 3, 51, 74, 4, 65, 14, 155, 158, 26, 4, 61, 41, 45, 37, 13, 181, 116,
            160, 63, 79, 91, 62, 45, 26, 140, 138, 144, 155, 65, 152, 63, 82, 38, 110, 34, 170,
            107, 154, 167, 88, 45, 183, 23, 18, 75, 80, 104, 181, 46, 180, 172, 14, 30, 30, 37,
            120, 2, 108, 166, 83, 83, 28, 144, 52, 157, 117, 57, 41, 66, 130, 94, 44, 35, 108, 25,
            119, 99, 57, 28, 18, 53, 123, 74, 124, 108, 7, 115, 165, 112, 99, 93, 20, 13, 103, 9,
            168, 57, 104, 133, 95, 140, 54, 118, 45, 116, 40, 105, 24, 179, 184, 15, 170, 168, 145,
            42, 134, 41,
        ],
    );
    // (digits_in_base + chars_per_limb) << (power_len - 2) <= exponents[0]
    //      in _limbs_compute_power_table_using_mul
    test(
        &[0; 210],
        &[
            3460366518, 3248332038, 2411832328, 3680172951, 1648892566, 683827580, 1099145716,
            3806372981, 2081403902, 2042441279, 575787637, 419553684, 2052335552, 545288482,
            448081444, 2074676634, 783644738, 65453313, 1428854749, 3138519856, 870590090,
            1920461474, 1804692757, 2629850054, 3724483390, 2876018746, 592000573, 3317750917,
            3395943485, 823080054, 3857418097, 892494948, 1415289101, 2374957426, 803534376,
            3410480407, 409051133, 4152156958, 1644919284, 1302252976, 2090652159, 3065750551,
            2916695391, 2276338541, 3864821397, 4050961189,
        ],
        152,
        &[
            1, 76, 32, 138, 63, 89, 114, 3, 9, 90, 74, 101, 16, 92, 47, 39, 0, 48, 102, 113, 15,
            123, 139, 33, 129, 140, 4, 93, 65, 38, 108, 102, 118, 40, 45, 109, 94, 140, 133, 120,
            43, 46, 72, 119, 10, 142, 36, 67, 80, 76, 70, 73, 12, 148, 117, 51, 24, 25, 36, 50,
            141, 1, 38, 50, 31, 72, 115, 75, 124, 151, 113, 74, 58, 26, 126, 60, 26, 129, 25, 12,
            94, 98, 77, 26, 32, 150, 26, 51, 141, 89, 63, 81, 15, 114, 11, 82, 3, 25, 129, 54, 111,
            75, 75, 136, 0, 30, 145, 127, 74, 62, 149, 15, 92, 117, 142, 92, 8, 30, 20, 32, 48, 79,
            23, 150, 144, 71, 119, 0, 107, 28, 18, 92, 53, 72, 70, 32, 146, 17, 49, 72, 134, 134,
            31, 40, 35, 76, 15, 53, 31, 47, 93, 51, 120, 125, 49, 149, 54, 55, 26, 149, 39, 86,
            138, 130, 64, 76, 86, 146, 64, 106, 69, 34, 97, 46, 104, 149, 114, 129, 120, 109, 124,
            12, 111, 71, 21, 13, 42, 146, 4, 60, 98, 98, 150, 134, 7, 86, 114, 118, 84, 3, 85, 127,
            102, 102,
        ],
    );
    // exponents[0] == chars_per_limb << number_of_powers in _limbs_compute_power_table_using_mul
    test(
        &[0; 160],
        &[
            2728906224, 1449576955, 3973690369, 849270619, 586255891, 923328784, 2717698803,
            1477432292, 3710905696, 2207709497, 4292599247, 2411645706, 177966862, 3982000026,
            1307696936, 903442, 3277385094, 3213674759, 2739559583, 1152850273, 3029194225,
            1704802500, 2548066116, 1747267099, 2072192542, 3912866034, 1575257763, 2717691639,
            3897187509, 2362053000, 1191544518,
        ],
        80,
        &[
            15, 18, 69, 61, 21, 59, 53, 35, 24, 20, 38, 58, 36, 48, 27, 15, 30, 14, 30, 16, 63, 8,
            66, 12, 57, 9, 78, 68, 12, 13, 47, 9, 77, 51, 3, 72, 15, 59, 71, 67, 20, 50, 65, 15,
            40, 3, 72, 79, 23, 9, 65, 43, 63, 57, 26, 54, 36, 78, 51, 25, 1, 22, 56, 53, 20, 45,
            56, 42, 69, 71, 76, 2, 53, 75, 58, 14, 24, 40, 4, 60, 6, 1, 29, 56, 50, 20, 47, 3, 63,
            52, 45, 66, 43, 7, 58, 49, 4, 73, 35, 51, 14, 35, 37, 28, 40, 76, 12, 60, 17, 65, 67,
            67, 79, 78, 79, 18, 24, 36, 20, 41, 79, 51, 21, 42, 38, 41, 39, 52, 68, 74, 59, 30, 15,
            21, 48, 26, 71, 48, 33, 46, 22, 9, 2, 78, 33, 21, 76, 4, 74, 72, 18, 37, 28, 56, 64,
            44, 48,
        ],
    );
}

#[test]
fn limbs_to_digits_small_base_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut out, base, mut xs)| {
            let old_out = out.clone();
            let old_xs = xs.clone();
            let out_len = _limbs_to_digits_small_base(&mut out, base, &mut xs, None);
            let result = out.clone();
            verify_limbs_to_digits_small_base(&old_out, &old_xs, base, out_len, &result);

            let mut xs = old_xs.clone();
            assert_eq!(
                _limbs_to_digits_small_base(
                    &mut out,
                    base,
                    &mut xs,
                    Some(PowerTableAlgorithm::Mul)
                ),
                out_len
            );
            assert_eq!(out, result);

            let mut xs = old_xs;
            assert_eq!(
                _limbs_to_digits_small_base(
                    &mut out,
                    base,
                    &mut xs,
                    Some(PowerTableAlgorithm::Div)
                ),
                out_len
            );
            assert_eq!(out, result);
        },
    );
}
