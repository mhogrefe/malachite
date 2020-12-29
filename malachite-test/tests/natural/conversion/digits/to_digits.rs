use malachite_nz::natural::conversion::digits::general_digits::{
    _limbs_to_digits_asc_basecase, _to_digits_asc_naive,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::quadruples_var_1;

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

#[test]
fn limbs_to_digits_asc_basecase_properties() {
    test_properties(quadruples_var_1, |&(ref out, len, ref xs, base)| {
        let old_out = out;
        let mut out = old_out.to_vec();
        let out_len = _limbs_to_digits_asc_basecase(&mut out, len, xs, base);
        verify_limbs_to_digits_asc_basecase(old_out, len, xs, base, out_len, &out);
    });
}
