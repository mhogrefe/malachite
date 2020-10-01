use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use malachite_base::strings::ToDebugString;

#[test]
fn test_debug() {
    let bd = BitDistributor::new(&[
        BitDistributorOutputType::normal(2),
        BitDistributorOutputType::tiny(),
    ]);
    assert_eq!(
        bd.to_debug_string(),
        "BitDistributor { \
             output_types: [\
                 BitDistributorOutputType { weight: 2, max_bits: None }, \
                 BitDistributorOutputType { weight: 0, max_bits: None }\
             ], \
             bit_map: [\
                 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
                 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
                 0, 0, 0, 0, 0, 0, 0, 0, 0, 1\
             ], \
             counter: [\
                 false, false, false, false, false, false, false, false, false, false, false, \
                 false, false, false, false, false, false, false, false, false, false, false, \
                 false, false, false, false, false, false, false, false, false, false, false, \
                 false, false, false, false, false, false, false, false, false, false, false, \
                 false, false, false, false, false, false, false, false, false, false, false, \
                 false, false, false, false, false, false, false, false, false\
             ] \
         }"
    );
}
