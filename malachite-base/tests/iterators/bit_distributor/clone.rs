use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_clone() {
    let bd = BitDistributor::new(&[
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ]);
    assert_eq!(bd.clone(), bd);
}
