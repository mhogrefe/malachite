use malachite_base::named::Named;
use malachite_base::round::RoundingMode;

#[test]
pub fn test_named() {
    fn test<T: Named>(out: &str) {
        assert_eq!(T::NAME, out);
    };
    test::<u8>("u8");
    test::<u16>("u16");
    test::<u32>("u32");
    test::<u64>("u64");
    test::<u128>("u128");
    test::<usize>("usize");
    test::<i8>("i8");
    test::<i16>("i16");
    test::<i32>("i32");
    test::<i64>("i64");
    test::<i128>("i128");
    test::<isize>("isize");
    test::<f32>("f32");
    test::<f64>("f64");

    test::<bool>("bool");
    test::<char>("char");

    test::<String>("String");

    test::<RoundingMode>("RoundingMode");
}
