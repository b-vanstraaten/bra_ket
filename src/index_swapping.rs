use crate::types::*;
use bitvec::prelude::*;

fn swap(x: usize, bits: (&Qubit, &Qubit)) -> usize {
    let bit_value_0 = (x & (1 << bits.0)) >> bits.0;
    let bit_value_1 = (x & (1 << bits.1)) >> bits.1;
    let difference = {
        let mut difference = bit_value_0 ^ bit_value_1;
        (difference << bits.0) | (difference << bits.1)
    };
    return x ^ difference;
}

pub fn swap_pair(x: usize, target: &Qubit) -> usize {
    swap(x, (&0, target))
}

pub fn swap_two_pairs(mut x: usize, target: &Qubit, control: &Qubit) -> usize {
    match (target, control) {
        // do nothing it is already correct
        (0, 1) => x,
        // swap the two bit values
        (1, 0) => swap(x, (&0, &1)),
        // it is only necessary to swap bit_1
        (0, _) => swap(x, (&1, &control)),
        // it is only necessary to swap bit_0
        (_, 1) => swap(x, (&0, &target)),
        // swap bits 0 and 1 then swap bit 0 with bit_1
        (1, _) => swap(swap(x, (&0, &1)), (&0, &control)),
        // swap bits 0 and 1 then swap bit 1 with qubit_0
        (_, 0) => swap(swap(x, (&0, &1)), (&1, &target)),
        // swap both bits
        (_, _) => swap(swap(x, (&0, &target)), (&1, &control)),
    }
}
