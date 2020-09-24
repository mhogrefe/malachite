use std::fmt::{Debug, Formatter, Result};

use num::basic::integers::PrimitiveInt;
use num::logic::traits::{BitConvertible, NotAssign};

const MAX_BITS: usize = u64::WIDTH as usize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitDistributorOutputType {
    width: usize, // 0 means a tiny output_type
    max_bits: Option<usize>,
}

impl BitDistributorOutputType {
    pub fn normal(width: usize) -> BitDistributorOutputType {
        assert_ne!(width, 0);
        BitDistributorOutputType {
            width,
            max_bits: None,
        }
    }

    pub fn normal_with_max_bits(width: usize, max_bits: usize) -> BitDistributorOutputType {
        assert_ne!(width, 0);
        BitDistributorOutputType {
            width,
            max_bits: Some(max_bits),
        }
    }

    pub const fn tiny() -> BitDistributorOutputType {
        BitDistributorOutputType {
            width: 0,
            max_bits: None,
        }
    }

    pub const fn tiny_with_max_bits(max_bits: usize) -> BitDistributorOutputType {
        BitDistributorOutputType {
            width: 0,
            max_bits: Some(max_bits),
        }
    }
}

pub struct BitDistributor {
    output_types: Vec<BitDistributorOutputType>,
    bit_map: [usize; MAX_BITS],
    counter: [bool; MAX_BITS],
}

impl PartialEq<BitDistributor> for BitDistributor {
    fn eq(&self, other: &BitDistributor) -> bool {
        self.output_types == other.output_types
            && self.bit_map[..] == other.bit_map[..]
            && self.counter[..] == other.counter[..]
    }
}

impl Eq for BitDistributor {}

impl Clone for BitDistributor {
    fn clone(&self) -> BitDistributor {
        let mut bit_map = [0; MAX_BITS];
        let mut counter = [false; MAX_BITS];
        bit_map.copy_from_slice(&self.bit_map);
        counter.copy_from_slice(&self.counter);
        BitDistributor {
            output_types: self.output_types.clone(),
            bit_map,
            counter,
        }
    }
}

impl Debug for BitDistributor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "BitDistributor {{ output_types: {:?}, bit_map: {:?}, counter: {:?} }}",
            self.output_types,
            &self.bit_map[..],
            &self.counter[..],
        )
    }
}

impl BitDistributor {
    fn new_without_init(output_types: &[BitDistributorOutputType]) -> BitDistributor {
        if output_types
            .iter()
            .all(|output_type| output_type.width == 0)
        {
            panic!("All output_types cannot be tiny");
        }
        BitDistributor {
            output_types: output_types.to_vec(),
            bit_map: [0; MAX_BITS],
            counter: [false; MAX_BITS],
        }
    }

    pub fn new(output_types: &[BitDistributorOutputType]) -> BitDistributor {
        let mut scheme = BitDistributor::new_without_init(output_types);
        scheme.update_bit_map();
        scheme
    }

    pub fn bit_map_as_slice(&self) -> &[usize] {
        self.bit_map.as_ref()
    }

    fn update_bit_map(&mut self) {
        let (mut normal_output_type_indices, mut tiny_output_type_indices): (
            Vec<usize>,
            Vec<usize>,
        ) = (0..self.output_types.len()).partition(|&i| self.output_types[i].width != 0);
        let mut normal_output_types_bits_used = vec![0; normal_output_type_indices.len()];
        let mut tiny_output_types_bits_used = vec![0; tiny_output_type_indices.len()];
        let mut ni = normal_output_type_indices.len() - 1;
        let mut ti = tiny_output_type_indices.len().saturating_sub(1);
        let mut width_counter = self.output_types[normal_output_type_indices[ni]].width;
        for i in 0..MAX_BITS {
            let use_normal_output_type = !normal_output_type_indices.is_empty()
                && (tiny_output_type_indices.is_empty() || !usize::is_power_of_two(i + 1));
            if use_normal_output_type {
                self.bit_map[i] = normal_output_type_indices[ni];
                let output_type = self.output_types[normal_output_type_indices[ni]];
                normal_output_types_bits_used[ni] += 1;
                width_counter -= 1;
                if output_type.max_bits == Some(normal_output_types_bits_used[ni]) {
                    normal_output_type_indices.remove(ni);
                    normal_output_types_bits_used.remove(ni);
                    if normal_output_type_indices.is_empty() {
                        continue;
                    }
                    width_counter = 0;
                }
                if width_counter == 0 {
                    if ni == 0 {
                        ni = normal_output_type_indices.len() - 1;
                    } else {
                        ni -= 1;
                    }
                    width_counter = self.output_types[normal_output_type_indices[ni]].width;
                }
            } else {
                if tiny_output_type_indices.is_empty() {
                    self.bit_map[i] = usize::MAX;
                    continue;
                }
                self.bit_map[i] = tiny_output_type_indices[ti];
                let output_type = self.output_types[tiny_output_type_indices[ti]];
                if output_type.max_bits == Some(tiny_output_types_bits_used[ti]) {
                    tiny_output_type_indices.remove(ti);
                    tiny_output_types_bits_used.remove(ni);
                    if tiny_output_type_indices.is_empty() {
                        continue;
                    }
                } else if ti == 0 {
                    ti = tiny_output_type_indices.len() - 1;
                } else {
                    ti -= 1;
                }
            }
        }
    }

    pub fn set_max_bits(&mut self, output_type_indices: &[usize], max_bits: usize) {
        assert_ne!(max_bits, 0);
        for &index in output_type_indices {
            self.output_types[index].max_bits = Some(max_bits);
        }
        self.update_bit_map();
    }

    pub fn increment_counter(&mut self) {
        for b in self.counter.iter_mut() {
            b.not_assign();
            if *b {
                break;
            }
        }
    }

    pub fn get_output(&self, index: usize) -> usize {
        assert!(index < self.output_types.len());
        usize::from_bit_iterator_asc(
            self.bit_map
                .iter()
                .zip(self.counter.iter())
                .filter_map(|(&m, &c)| if m == index { Some(c) } else { None }),
        )
    }
}
