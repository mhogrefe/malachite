use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_exhaustive_rounding_modes() {
    assert_eq!(
        exhaustive_rounding_modes().collect::<Vec<RoundingMode>>(),
        &[
            RoundingMode::Down,
            RoundingMode::Up,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
            RoundingMode::Nearest,
            RoundingMode::Exact,
        ]
    );
}
