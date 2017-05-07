use malachite_native::natural as native;
use malachite_gmp::natural as gmp;

#[test]
fn test_limbs_le() {
    let test = |limbs: Vec<u32>, out| {
        let limbs = &limbs;

        let mut x = native::Natural::new();
        x.assign_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let mut x = gmp::Natural::new();
        x.assign_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(vec![], "0");
    test(vec![0], "0");
    test(vec![0, 0, 0], "0");
    test(vec![123], "123");
    test(vec![123, 0], "123");
    test(vec![123, 0, 0, 0], "123");
    test(vec![3567587328, 232], "1000000000000");
    test(vec![3567587328, 232, 0], "1000000000000");
    test(vec![1, 2, 3, 4, 5],
         "1701411834921604967429270619762735448065");
}
