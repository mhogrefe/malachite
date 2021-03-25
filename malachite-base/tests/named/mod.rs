use malachite_base::named::Named;
use malachite_base::rounding_modes::RoundingMode;

#[test]
pub fn test_named() {
    fn test<T: Named>(out: &str) {
        assert_eq!(T::NAME, out);
    }
    test::<String>("String");
    test::<RoundingMode>("RoundingMode");
}
