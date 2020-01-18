use malachite_base::comparison::Max;
use malachite_base::num::logic::traits::HammingDistance;

#[test]
pub fn test_hamming_distance() {
    fn test<T: HammingDistance<T>>(x: T, y: T, out: u64) {
        assert_eq!(x.hamming_distance(y), out);
    };

    test(123u32, 456u32, 6);
    test(0i8, -1i8, 8);
    test(0i128, -1i128, 128);
    test(0xffffu32, 0xffff_0000u32, 32);
    test(0xffffu32, u32::MAX, 16);
    test(0xffff_0000u32, u32::MAX, 16);
}
