use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_increment_counter() {
    let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
    let mut outputs = Vec::new();
    for _ in 0..20 {
        outputs.push(bd.get_output(0));
        bd.increment_counter();
    }
    assert_eq!(
        outputs,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    );
}
