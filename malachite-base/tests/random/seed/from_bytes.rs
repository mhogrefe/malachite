use malachite_base::random::Seed;
use malachite_base::random::EXAMPLE_SEED;

#[test]
fn test_from_bytes() {
    assert_eq!(Seed::from_bytes(EXAMPLE_SEED.bytes), EXAMPLE_SEED);
}
