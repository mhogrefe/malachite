// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::integers::PrimitiveInt;
use crate::num::logic::traits::{BitConvertible, NotAssign};
use alloc::vec::Vec;
use core::fmt::Debug;

const COUNTER_WIDTH: usize = u64::WIDTH as usize;

/// This struct is used to configure [`BitDistributor`]s.
///
/// See the [`BitDistributor`] documentation for more.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BitDistributorOutputType {
    weight: usize, // 0 means a tiny output_type
    max_bits: Option<usize>,
}

impl BitDistributorOutputType {
    /// Creates a normal output with a specified weight.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `weight` is zero.
    ///
    /// The corresponding element grows as a power of $i$. See the [`BitDistributor`] documentation
    /// for more.
    pub fn normal(weight: usize) -> BitDistributorOutputType {
        assert_ne!(weight, 0);
        BitDistributorOutputType {
            weight,
            max_bits: None,
        }
    }

    /// Creates a tiny output.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// The corresponding element grows logarithmically. See the [`BitDistributor`] documentation
    /// for more.
    pub const fn tiny() -> BitDistributorOutputType {
        BitDistributorOutputType {
            weight: 0,
            max_bits: None,
        }
    }
}

/// Helps generate tuples exhaustively.
///
/// Think of `counter` as the bits of an integer. It's initialized to zero (all `false`s), and as
/// it's repeatedly incremented, it eventually takes on every 64-bit value.
///
/// `output_types` is a list of $n$ configuration structs that, together, specify how to generate an
/// n-element tuple of unsigned integers. Calling `get_output` repeatedly, passing in 0 through $n -
/// 1$ as `index`, distributes the bits of `counter` into a tuple.
///
/// This is best shown with an example. If `output_types` is set to
/// `[BitDistributorOutputType::normal(1); 2]`, the distributor will generate all pairs of unsigned
/// integers. A pair may be extracted by calling `get_output(0)` and `get_output(1)`; then `counter`
/// may be incremented to create the next pair. In this case, the pairs will be $(0, 0), (0, 1), (1,
/// 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1), \ldots$.
///
/// If you think of these pairs as coordinates in the $xy$-plane, they are traversed along a
/// [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve). Every pair of unsigned integers
/// will be generated exactly once.
///
/// In general, setting `output_types` to `[BitDistributorOutputType::normal(1); n]` will generate
/// $n$-tuples. The elements of the tuples will be very roughly the same size, in the sense that
/// each element will grow as $O(\sqrt\[n\]{i})$, where $i$ is the counter. Sometimes we want the
/// elements to grow at different rates. To accomplish this, we can change the weights of the output
/// types. For example, if we set `output_types` to `[BitDistributorOutputType::normal(1),
/// BitDistributorOutputType::normal(2)]`, the first element of the generated pairs will grow as
/// $O(\sqrt\[3\]{i})$ and the second as $O(i^{2/3})$. In general, if the weights are $w_0, w_1,
/// \\ldots, w_{n-1}$, then the $k$th element of the output tuples will grow as
/// $O(i^{w_i/\sum_{j=0}^{n-1}w_j})$.
///
/// Apart from creating _normal_ output types with different weights, we can create _tiny_ output
/// types, which indicate that the corresponding tuple element should grow especially slowly. If
/// `output_types` contains $m$ tiny output types, each tiny tuple element grows as
/// $O(\sqrt\[m\]{\log i})$. The growth of the other elements is unaffected. Having only tiny types
/// in `output_types` is disallowed.
///
/// The above discussion of growth rates assumes that `max_bits` is not specified for any output
/// type. But if `max_bits` is set to $b$, then the corresponding element will start growing just as
/// if `max_bits` wasn't specified, but will stop growing once it reaches $2^b-1$.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BitDistributor {
    #[cfg(feature = "test_build")]
    pub output_types: Vec<BitDistributorOutputType>,
    #[cfg(not(feature = "test_build"))]
    output_types: Vec<BitDistributorOutputType>,
    bit_map: [usize; COUNTER_WIDTH],
    counter: [bool; COUNTER_WIDTH],
}

impl BitDistributor {
    fn new_without_init(output_types: &[BitDistributorOutputType]) -> BitDistributor {
        if output_types
            .iter()
            .all(|output_type| output_type.weight == 0)
        {
            panic!("All output_types cannot be tiny");
        }
        BitDistributor {
            output_types: output_types.to_vec(),
            bit_map: [0; COUNTER_WIDTH],
            counter: [false; COUNTER_WIDTH],
        }
    }

    /// Creates a new [`BitDistributor`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `output_types.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::bit_distributor::{
    ///     BitDistributor, BitDistributorOutputType,
    /// };
    ///
    /// BitDistributor::new(&[
    ///     BitDistributorOutputType::normal(2),
    ///     BitDistributorOutputType::tiny(),
    /// ]);
    /// ```
    pub fn new(output_types: &[BitDistributorOutputType]) -> BitDistributor {
        let mut distributor = BitDistributor::new_without_init(output_types);
        distributor.update_bit_map();
        distributor
    }

    /// Returns a reference to the internal bit map as a slice.
    ///
    /// The bit map determines which output gets each bit of the counter. For example, if the bit
    /// map is $[0, 1, 0, 1, 0, 1, \ldots]$, then the first element of the output pair gets the bits
    /// with indices $0, 2, 4, \ldots$ and the second element gets the bits with indices $1, 3, 5,
    /// \ldots$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::bit_distributor::{
    ///     BitDistributor, BitDistributorOutputType,
    /// };
    ///
    /// let bd = BitDistributor::new(&[
    ///     BitDistributorOutputType::normal(2),
    ///     BitDistributorOutputType::tiny(),
    /// ]);
    /// assert_eq!(
    ///     bd.bit_map_as_slice(),
    ///     &[
    ///         1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///         0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///         0, 0, 0, 0, 0, 0, 0, 1
    ///     ][..]
    /// );
    /// ```
    pub fn bit_map_as_slice(&self) -> &[usize] {
        self.bit_map.as_ref()
    }

    fn update_bit_map(&mut self) {
        let (mut normal_output_type_indices, mut tiny_output_type_indices): (
            Vec<usize>,
            Vec<usize>,
        ) = (0..self.output_types.len()).partition(|&i| self.output_types[i].weight != 0);
        let mut normal_output_types_bits_used = vec![0; normal_output_type_indices.len()];
        let mut tiny_output_types_bits_used = vec![0; tiny_output_type_indices.len()];
        let mut ni = normal_output_type_indices.len() - 1;
        let mut ti = tiny_output_type_indices.len().saturating_sub(1);
        let mut weight_counter = self.output_types[normal_output_type_indices[ni]].weight;
        for i in 0..COUNTER_WIDTH {
            let use_normal_output_type = !normal_output_type_indices.is_empty()
                && (tiny_output_type_indices.is_empty() || !usize::is_power_of_two(i + 1));
            if use_normal_output_type {
                self.bit_map[i] = normal_output_type_indices[ni];
                let output_type = self.output_types[normal_output_type_indices[ni]];
                normal_output_types_bits_used[ni] += 1;
                weight_counter -= 1;
                if output_type.max_bits == Some(normal_output_types_bits_used[ni]) {
                    normal_output_type_indices.remove(ni);
                    normal_output_types_bits_used.remove(ni);
                    if normal_output_type_indices.is_empty() {
                        continue;
                    }
                    weight_counter = 0;
                }
                if weight_counter == 0 {
                    if ni == 0 {
                        ni = normal_output_type_indices.len() - 1;
                    } else {
                        ni -= 1;
                    }
                    weight_counter = self.output_types[normal_output_type_indices[ni]].weight;
                }
            } else {
                if tiny_output_type_indices.is_empty() {
                    self.bit_map[i] = usize::MAX;
                    continue;
                }
                self.bit_map[i] = tiny_output_type_indices[ti];
                let output_type = self.output_types[tiny_output_type_indices[ti]];
                tiny_output_types_bits_used[ti] += 1;
                if output_type.max_bits == Some(tiny_output_types_bits_used[ti]) {
                    tiny_output_type_indices.remove(ti);
                    tiny_output_types_bits_used.remove(ti);
                    if tiny_output_type_indices.is_empty() {
                        continue;
                    }
                }
                if ti == 0 {
                    ti = tiny_output_type_indices.len() - 1;
                } else {
                    ti -= 1;
                }
            }
        }
    }

    /// Sets the maximum bits for several outputs.
    ///
    /// Given slice of output indices, sets the maximum bits for each of the outputs and rebuilds
    /// the bit map.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `output_type_indices.len()`.
    ///
    /// # Panics
    /// Panics if `max_bits` is 0 or if any index is greater than or equal to
    /// `self.output_types.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::bit_distributor::{
    ///     BitDistributor, BitDistributorOutputType,
    /// };
    ///
    /// let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(2); 3]);
    /// assert_eq!(
    ///     bd.bit_map_as_slice(),
    ///     &[
    ///         2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1,
    ///         0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2,
    ///         1, 1, 0, 0, 2, 2, 1, 1
    ///     ][..]
    /// );
    ///
    /// bd.set_max_bits(&[0, 2], 5);
    /// assert_eq!(
    ///     bd.bit_map_as_slice(),
    ///     &[
    ///         2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ///         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ///         1, 1, 1, 1, 1, 1, 1, 1
    ///     ][..]
    /// );
    /// ```
    pub fn set_max_bits(&mut self, output_type_indices: &[usize], max_bits: usize) {
        assert_ne!(max_bits, 0);
        for &index in output_type_indices {
            self.output_types[index].max_bits = Some(max_bits);
        }
        self.update_bit_map();
    }

    /// Increments the counter in preparation for a new set of outputs.
    ///
    /// If the counter is incremented $2^{64}$ times, it rolls back to 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::bit_distributor::{
    ///     BitDistributor, BitDistributorOutputType,
    /// };
    ///
    /// let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
    /// let mut outputs = Vec::new();
    /// for _ in 0..20 {
    ///     outputs.push(bd.get_output(0));
    ///     bd.increment_counter();
    /// }
    /// assert_eq!(
    ///     outputs,
    ///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    /// );
    /// ```
    pub fn increment_counter(&mut self) {
        for b in &mut self.counter {
            b.not_assign();
            if *b {
                break;
            }
        }
    }

    /// Gets the output at a specified index.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to `self.output_types.len()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::iterators::bit_distributor::{
    ///     BitDistributor, BitDistributorOutputType,
    /// };
    ///
    /// let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]);
    /// let mut outputs = Vec::new();
    /// for _ in 0..10 {
    ///     outputs.push((0..2).map(|i| bd.get_output(i)).collect_vec());
    ///     bd.increment_counter();
    /// }
    /// let expected_outputs: &[&[usize]] = &[
    ///     &[0, 0],
    ///     &[0, 1],
    ///     &[1, 0],
    ///     &[1, 1],
    ///     &[0, 2],
    ///     &[0, 3],
    ///     &[1, 2],
    ///     &[1, 3],
    ///     &[2, 0],
    ///     &[2, 1],
    /// ];
    /// assert_eq!(outputs, expected_outputs);
    /// ```
    pub fn get_output(&self, index: usize) -> usize {
        assert!(index < self.output_types.len());
        usize::from_bits_asc(
            self.bit_map
                .iter()
                .zip(self.counter.iter())
                .filter_map(|(&m, &c)| if m == index { Some(c) } else { None }),
        )
    }
}
