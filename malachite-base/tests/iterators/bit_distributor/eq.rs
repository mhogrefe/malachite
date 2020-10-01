use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_eq() {
    let bd_1 = BitDistributor::new(&[
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ]);
    let bd_2 = BitDistributor::new(&[
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ]);
    let bd_3 = BitDistributor::new(&[
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(2),
    ]);
    assert_eq!(bd_1, bd_2);
    assert_ne!(bd_1, bd_3);
    assert_ne!(bd_2, bd_3);
}
