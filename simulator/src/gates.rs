use itertools::iproduct;
use log::debug;
use nalgebra::matrix;

use crate::index_swapping::*;
use crate::types::*;
use crate::{DensityMatrix, Program};

use crate::state::{Measure, SingleQubitGate, TwoQubitGate};

#[derive(Debug, Clone)]
pub enum Gate {
    Measure(usize),
    X(usize),
    Y(usize),
    Z(usize),
    H(usize),
    ArbitarySingle(usize, Matrix2x2),
    S(usize),

    RX(usize, Angle),
    RY(usize, Angle),
    RZ(usize, Angle),
    R(usize, Angle, Angle, Angle),

    CNOT(usize, usize),
    SISWAP(usize, usize),
    ArbitaryTwo(usize, usize, Matrix4x4),
    ISWAP(usize, usize),
}

pub fn implement_gate<T: Measure + SingleQubitGate + TwoQubitGate>(state: &mut T, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => state.measure(qubit),
        Gate::X(qubit) => state.single_qubit_gate(qubit, &SIGMA_X),
        Gate::Y(qubit) => state.single_qubit_gate(qubit, &SIGMA_Y),
        Gate::Z(qubit) => state.single_qubit_gate(qubit, &SIGMA_Z),
        Gate::S(qubit) => state.single_qubit_gate(qubit, &S),
        Gate::H(qubit) => state.single_qubit_gate(qubit, &H),

        Gate::RX(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_X * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Gate::RY(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_Y * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Gate::RZ(qubit, angle) => {
            let u = IDENTITY * C::new((angle / 2.).cos(), 0.)
                - SIGMA_Z * C::new(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }
        Gate::R(qubit, phi, theta, omega) => {
            let (c_theta, s_theta) = ((theta / 2.).cos(), (theta / 2.).sin());
            let (c_plus, s_plus) = (((phi + omega) / 2.).cos(), ((phi + omega) / 2.).sin());
            let (c_minus, s_minus) = (((phi - omega) / 2.).cos(), ((phi - omega) / 2.).sin());

            let u: Matrix2x2 = matrix![
                C::new(c_plus, -s_plus) * c_theta, -C::new(c_minus, s_minus) * s_theta;
                C::new(c_minus, -s_minus) * s_theta,  C::new(c_plus, s_plus) * c_theta
            ];
            state.single_qubit_gate(qubit, &u)
        }
        Gate::ArbitarySingle(qubit, u) => state.single_qubit_gate(qubit, u),

        Gate::CNOT(control, target) => state.two_qubit_gate(target, control, &CNOT),
        Gate::ISWAP(control, target) => state.two_qubit_gate(target, control, &ISWAP),
        Gate::SISWAP(control, target) => state.two_qubit_gate(target, control, &SISWAP),
        Gate::ArbitaryTwo(control, target, u) => state.two_qubit_gate(control, target, u),
    }
}

pub fn which_qubits(gate: &Gate) -> Vec<&usize> {
    match gate {
        Gate::Measure(qubit) => vec![qubit],
        Gate::X(qubit) => vec![qubit],
        Gate::Y(qubit) => vec![qubit],
        Gate::Z(qubit) => vec![qubit],
        Gate::S(qubit) => vec![qubit],
        Gate::H(qubit) => vec![qubit],

        Gate::RX(qubit, _) => vec![qubit],
        Gate::RY(qubit, _) => vec![qubit],
        Gate::RZ(qubit, _) => vec![qubit],
        Gate::R(qubit, _, _, _) => vec![qubit],
        Gate::ArbitarySingle(qubit, _) => vec![qubit],

        Gate::CNOT(control, target) => vec![control, target],
        Gate::ISWAP(control, target) => vec![control, target],
        Gate::SISWAP(control, target) => vec![control, target],
        Gate::ArbitaryTwo(control, target, _) => vec![control, target],
    }
}
