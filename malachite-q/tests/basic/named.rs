use malachite_base::named::Named;
use malachite_q::Rational;

#[test]
fn test_named() {
    assert_eq!(Rational::NAME, "Rational");
}
