use std::cmp::Ordering;

use itertools::iproduct;
use log::debug;

use crate::types::*;
use crate::State;

fn _swap_bits(x: usize, bits: (&Qubit, &Qubit)) -> usize {
    let bit_value_0 = x & (1 << bits.0);
    let x = x ^ bit_value_0;
    let bit_value_1 = x & (1 << bits.1);
    let x = x ^ bit_value_1;
    let bit_values = (bit_value_1 >> (bits.1 - bits.0)) ^ (bit_value_0 << (bits.1 - bits.0));
    return x ^ bit_values;
}

fn swap_two_bits(x: usize, bits: (&Qubit, &Qubit)) -> usize {
    match bits.1.cmp(&bits.0) {
        Ordering::Less => return _swap_bits(x, (bits.1, bits.0)),
        Ordering::Equal => return x,
        Ordering::Greater => return _swap_bits(x, (bits.0, bits.1)),
    }
}

fn swap_four_bits(mut x: usize, bit_pairs: [(&Qubit, &Qubit); 2]) -> usize {
    for (bit_0, bit_1) in bit_pairs.iter() {
        x = swap_two_bits(x, (bit_0, bit_1));
    }
    return x;
}

#[derive(Debug)]
pub enum Gate {
    Measure(Qubit),
    X(Qubit, Angle),
    Y(Qubit, Angle),
    Z(Qubit, Angle),
    H(Qubit),
    CNOT(Qubit, Qubit),
}

pub fn implement_gate(state: &mut State, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => measure(state, qubit),
        Gate::X(qubit, angle) => x(state, qubit, angle),
        Gate::Y(qubit, angle) => y(state, qubit, angle),
        Gate::Z(qubit, angle) => z(state, qubit, angle),
        Gate::H(qubit) => h(state, qubit),
        Gate::CNOT(control, target) => cnot(state, control, target),
    }
}

fn single_qubit_gate(state: &mut State, qubit: &Qubit, u: Matrix2x2) {
    debug!("density matrix before:\n{}", state.density_matrix);

    let swap_qubits: Box<dyn Fn(usize) -> Qubit> = match qubit {
        // do nothing it is already correct
        0 => Box::new(|x: usize| x),
        // swap the bit at qubit to bit zero
        _ => Box::new(|x: usize| swap_two_bits(x, (&qubit, &0))),
    };

    let mut rho = Matrix2x2::zeros();
    for (i_offset, j_offset) in iproduct!(
        (0..1 << state.number_of_qubits).step_by(2),
        (0..1 << state.number_of_qubits).step_by(2)
    ) {
        for (i, j) in iproduct!(0..2, 0..2) {
            rho[(i, j)] =
                state.density_matrix[(swap_qubits(i + i_offset), swap_qubits(j + j_offset))];
        }

        rho = u * rho * u.adjoint();

        for (i, j) in iproduct!(0..2, 0..2) {
            state.density_matrix[(swap_qubits(i + i_offset), swap_qubits(j + j_offset))] =
                rho[(i, j)]
        }
    }
    debug!("density matrix after:\n{}", state.density_matrix);
}

fn two_qubit_gate(state: &mut State, qubit_0: &Qubit, qubit_1: &Qubit, u: Matrix4x4) {
    debug!("density matrix before:\n{}", state.density_matrix);

    let swap_qubits: Box<dyn Fn(usize) -> Qubit> = match (qubit_0, qubit_1) {
        // do nothing it is already correct
        (0, 1) => Box::new(|x| x),
        // swap the two bit values
        (1, 0) => Box::new(|x| swap_two_bits(x, (&0, &1))),
        // it is only necessary to swap bit_1
        (0, _) => Box::new(|x: usize| swap_two_bits(x, (&1, &qubit_1))),
        // it is only necessary to swap bit_0
        (_, 1) => Box::new(|x: usize| swap_two_bits(x, (&0, &qubit_0))),
        // swap bits 0 and 1 then swap bit 0 with bit_1
        (1, _) => Box::new(|x: usize| swap_four_bits(x, [(&0, &1), (&0, &qubit_1)])),
        // swap bits 0 and 1 then swap bit 1 with qubit_0
        (_, 0) => Box::new(|x: usize| swap_four_bits(x, [(&0, &1), (&1, &qubit_0)])),
        // swap both bits
        (_, _) => Box::new(|x: usize| swap_four_bits(x, [(&0, &qubit_0), (&1, &qubit_1)]))
    };

    let mut rho = Matrix4x4::zeros();
    for (i_offset, j_offset) in iproduct!(
        (0..1 << state.number_of_qubits).step_by(4),
        (0..1 << state.number_of_qubits).step_by(4)
    ) {
        for (i, j) in iproduct!(0..4, 0..4) {
            rho[(i, j)] =
                state.density_matrix[(swap_qubits(i + i_offset), swap_qubits(j + j_offset))];
        }

        rho = u * rho * u.adjoint();

        for (i, j) in iproduct!(0..4, 0..4) {
            state.density_matrix[(swap_qubits(i + i_offset), swap_qubits(j + j_offset))] =
                rho[(i, j)]
        }
    }
    debug!("density matrix after:\n{}", state.density_matrix);
}

fn measure(state: &mut State, qubit: &Qubit) {
    for (i_offset, j_offset) in iproduct!(
        (0..1 << state.number_of_qubits).step_by(2),
        (0..1 << state.number_of_qubits).step_by(2)
    ) {
        for (i, j) in [(0, 1), (1, 0)] {
            state.density_matrix[(
                swap_two_bits(i + i_offset, (qubit, &0)),
                swap_two_bits(j + j_offset, (qubit, &0)),
            )] = C::new(0., 0.)
        }
    }
    debug!("density matrix after:\n{}", state.density_matrix);
}

fn x(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_X * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn y(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Y * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn z(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Z * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn h(state: &mut State, qubit: &Qubit) {
    debug!("u:\n{}", H);
    single_qubit_gate(state, qubit, H)
}

fn cnot(state: &mut State, control: &Qubit, target: &Qubit) {
    two_qubit_gate(state, target, control, CNOT)
}
