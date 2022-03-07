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
}

pub fn implement_gate(state: &mut State, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => measure(state, qubit),
        Gate::X(qubit, angle) => x(state, qubit, angle),
        Gate::Y(qubit, angle) => y(state, qubit, angle),
        Gate::Z(qubit, angle) => z(state, qubit, angle),
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

fn state_iter(states: [Qubit; 2]) -> Vec<((Qubit, Qubit), (Qubit, Qubit))> {
    let qubit_index_iter = iproduct!(states, states);
    let slice_index_iter = iproduct!(0..2 as Qubit, 0..2 as Qubit);
    zip(qubit_index_iter, slice_index_iter).collect()
}

fn single_qubit_gate(state: &mut State, qubit: &Qubit, u: Matrix2x2) {
    let state_pairs = calculate_state_pairs(&state.number_of_qubits, qubit);
    for states in state_pairs {
        let mut rho = Matrix2x2::zeros();
        for (qubit_index, slice_index) in state_iter(states) {
            rho[slice_index] = state.density_matrix[qubit_index];
        }
        debug!("rho before:\n{}", rho);
        rho = u * rho * u.adjoint();
        debug!("rho after:\n{}", rho);

        for (qubit_index, slice_index) in state_iter(states) {
            state.density_matrix[qubit_index] = rho[slice_index];
        }
    }
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
