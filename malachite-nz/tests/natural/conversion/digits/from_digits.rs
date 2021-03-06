use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_nz::natural::conversion::digits::general_digits::_from_digits_desc_naive_primitive;
use malachite_nz::natural::conversion::digits::general_digits::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::generators::unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2;

fn verify_limbs_from_digits_small_base_basecase<T: PrimitiveUnsigned>(
    original_out: &[Limb],
    xs: &[T],
    base: u64,
    out_len: usize,
    out: &[Limb],
) where
    Natural: From<T>,
{
    let mut expected_limbs =
        _from_digits_desc_naive_primitive(xs, T::exact_from(base)).into_limbs_asc();
    assert!(expected_limbs.len() <= out_len);
    expected_limbs.resize(out_len, 0);
    assert_eq!(expected_limbs, &out[..out_len]);
    assert_eq!(&original_out[out_len..], &out[out_len..]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_from_digits_small_base_basecase() {
    fn test(out_before: &[Limb], xs: &[u8], base: u64, out_after: &[Limb]) {
        let mut out = out_before.to_vec();
        let out_len = _limbs_from_digits_small_base_basecase(&mut out, xs, base);
        assert_eq!(&out[..out_len], out_after);
        verify_limbs_from_digits_small_base_basecase(out_before, xs, base, out_len, &out);
    };
    // res_digit == 0
    test(&[10; 2], &[0], 9, &[]);
    // base != 10
    // size == 0 second time
    // res_digit != 0
    test(&[10; 2], &[1], 9, &[1]);
    test(&[10; 2], &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0], 3, &[123456]);
    test(&[10; 2], &[1, 2, 4, 2, 2, 3, 1, 1], 5, &[123456]);
    test(&[10; 2], &[2, 3, 5, 1, 3, 2, 0], 6, &[123456]);
    test(&[10; 2], &[1, 0, 2, 2, 6, 3, 4], 7, &[123456]);
    test(&[10; 2], &[2, 0, 7, 3, 1, 3], 9, &[123456]);
    // base == 10
    test(&[10; 2], &[1, 2, 3, 4, 5, 6], 10, &[123456]);
    test(&[10; 2], &[8, 4, 8, 3, 3], 11, &[123456]);
    test(&[10; 2], &[5, 11, 5, 4, 0], 12, &[123456]);
    test(&[10; 2], &[4, 4, 2, 6, 8], 13, &[123456]);
    test(&[10; 2], &[3, 2, 13, 12, 4], 14, &[123456]);
    test(&[10; 2], &[2, 6, 8, 10, 6], 15, &[123456]);
    test(&[10; 2], &[12, 34, 56], 100, &[123456]);
    test(&[10; 2], &[8, 19, 87], 123, &[123456]);
    test(&[10; 2], &[1, 229, 36], 255, &[123456]);
    // j < xs_len
    // size == 0 first time
    // y != 0
    // size != 0 second time
    // cy_limb != 0 second time
    test(&[10; 4], &[27, 90, 34, 55, 72, 93], 104, &[4056409437, 78]);
    // size != 0 first time
    // cy_limb != 0 first time
    test(
        &[10; 13],
        &[93, 88, 7, 58, 53, 12, 72, 49, 56, 91],
        98,
        &[419414959, 1047946650, 4],
    );
    // cy_limb == 0 second time
    test(
        &[10; 7],
        &[
            14, 117, 113, 119, 39, 15, 71, 111, 83, 69, 36, 65, 47, 105, 10, 57, 101,
        ],
        121,
        &[3340886628, 3137387930, 2143550403, 399066],
    );
    // cy_limb == 0 first time
    test(
        &[10; 15],
        &[
            116, 130, 52, 119, 76, 102, 131, 43, 138, 137, 117, 78, 23, 136, 111, 20, 132, 103,
            126, 38, 96, 23, 23, 128, 17, 123, 30, 135, 39, 114, 36, 49, 22, 90, 79, 87, 32, 21,
            67, 55, 138, 33, 44, 84, 72, 104, 10, 118, 28, 63, 85, 36, 110, 108, 61, 16, 115, 75,
            26,
        ],
        144,
        &[
            3776613706, 1267825649, 3972333409, 93554931, 973182892, 3884362880, 1799682409,
            3576731056, 1717911507, 3669574309, 502648924, 825650249, 3326096263, 105,
        ],
    );
    // y == 0
    test(&[10; 3], &[0; 8], 17, &[]);
}

fn limbs_from_digits_small_base_basecase_properties_helper<T: PrimitiveUnsigned>()
where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    let mut config = GenConfig::new();
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    config.insert("mean_digit_count_n", 32);
    config.insert("mean_excess_limb_count_n", 32);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>().test_properties_with_config(
        &config,
        |(mut out, xs, base)| {
            let old_out = out.clone();
            let out_len = _limbs_from_digits_small_base_basecase(&mut out, &xs, base);
            verify_limbs_from_digits_small_base_basecase(&old_out, &xs, base, out_len, &out);
        },
    );
}

#[test]
fn limbs_from_digits_small_base_basecase_properties() {
    apply_fn_to_unsigneds!(limbs_from_digits_small_base_basecase_properties_helper);
}
