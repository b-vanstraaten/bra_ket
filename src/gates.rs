use itertools::{iproduct, zip};
use log::{debug, info, warn};
use nalgebra::{Complex, SMatrix};

use crate::types::*;
use crate::{Program, State};

#[derive(Debug)]
pub enum Gate {
    Measure(Qubit),
    X(Qubit, Angle),
    Y(Qubit, Angle),
    Z(Qubit, Angle),
    H(Qubit),
}

pub fn implement_gate(state: &mut State, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => measure(state, qubit),
        Gate::X(qubit, angle) => x(state, qubit, angle),
        Gate::Y(qubit, angle) => y(state, qubit, angle),
        Gate::Z(qubit, angle) => z(state, qubit, angle),
        Gate::H(qubit) => h(state, qubit),
    }
}

fn calculate_state_pairs(number_of_qubits: &Qubit, qubit: &Qubit) -> Vec<[Qubit; 2]> {
    let number_of_right_bits = qubit;
    let number_of_left_bits = number_of_qubits - (qubit + 1);

    let left_iterator = 0..((1 as Qubit) << number_of_left_bits);
    let right_iterator = 0..((1 as Qubit) << number_of_right_bits);

    iproduct!(left_iterator, right_iterator)
        .map(|(left_bits, right_bits)| (left_bits << number_of_right_bits + 1) + right_bits)
        .map(|index_zero| [index_zero, index_zero + (1 << qubit)])
        .collect()
}


fn single_qubit_gate(state: &mut State, qubit: &Qubit, u: Matrix2x2) {
    debug!("density matrix before:\n{}", state.density_matrix);

    let mut rho = Matrix2x2::zeros();
    for (i_offset, j_offset) in iproduct!(
        (0..1 << state.number_of_qubits).step_by(2),
        (0..1 << state.number_of_qubits).step_by(2)
    ) {
        for (i, j) in iproduct!(0..2, 0..2) {
            rho[(i, j)] = state.density_matrix[(i_offset + i, j_offset + j)];
        }

        rho = u * rho * u.adjoint();

        for (i, j) in iproduct!(0..2, 0..2) {
            state.density_matrix[(i_offset + i, j_offset + j)] = rho[(i, j)]
        }
    }
    debug!("density matrix after:\n{}", state.density_matrix);
}

fn measure(state: &mut State, qubit: &Qubit) {
    let state_pairs = calculate_state_pairs(&state.number_of_qubits, qubit);
    for states in state_pairs {
        state.density_matrix[(states[0], states[1])] = C::new(0., 0.);
        state.density_matrix[(states[1], states[0])] = C::new(0., 0.);
    }
}

fn x(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) + SIGMA_X * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn y(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) + SIGMA_Y * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn z(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) + SIGMA_Z * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn h(state: &mut State, qubit: &Qubit) {
    debug!("u:\n{}", H);
    single_qubit_gate(state, qubit, H)
}
