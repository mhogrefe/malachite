use malachite_base::num::{NegativeOne, One, PrimitiveFloat, Two, Zero};

//TODO move
#[test]
fn test_f32() {
    let test = |f: f32, out| {
        assert_eq!(f.to_bits(), out);
    };
    test(f32::NAN, 0x7fc0_0000);
    test(f32::ZERO, 0);
    test(f32::NEGATIVE_ZERO, 0x8000_0000);
    test(f32::MIN_POSITIVE, 1);
    test(f32::MAX_SUBNORMAL, 0x7f_ffff);
    test(f32::MIN_POSITIVE_NORMAL, 0x80_0000);
    test(f32::ONE, 0x3f80_0000);
    test(f32::NEGATIVE_ONE, 0xbf80_0000);
    test(f32::TWO, 0x4000_0000);
    test(f32::MAX_FINITE, 0x7f7f_ffff);
    test(f32::MIN_FINITE, 0xff7f_ffff);
    test(f32::POSITIVE_INFINITY, 0x7f80_0000);
    test(f32::NEGATIVE_INFINITY, 0xff80_0000);
}

#[test]
fn test_f64() {
    let test = |f: f64, out| {
        assert_eq!(f.to_bits(), out);
    };
    test(f64::NAN, 0x7ff8_0000_0000_0000);
    test(f64::ZERO, 0);
    test(f64::NEGATIVE_ZERO, 0x8000_0000_0000_0000);
    test(f64::MIN_POSITIVE, 1);
    test(f64::MAX_SUBNORMAL, 0xf_ffff_ffff_ffff);
    test(f64::MIN_POSITIVE_NORMAL, 0x10_0000_0000_0000);
    test(f64::ONE, 0x3ff0_0000_0000_0000);
    test(f64::NEGATIVE_ONE, 0xbff0_0000_0000_0000);
    test(f64::TWO, 0x4000_0000_0000_0000);
    test(f64::MAX_FINITE, 0x7fef_ffff_ffff_ffff);
    test(f64::MIN_FINITE, 0xffef_ffff_ffff_ffff);
    test(f64::POSITIVE_INFINITY, 0x7ff0_0000_0000_0000);
    test(f64::NEGATIVE_INFINITY, 0xfff0_0000_0000_0000);
}
