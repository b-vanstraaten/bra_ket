use std::fmt;

use log::debug;
use nalgebra::matrix;

use crate::state_traits::QuantumStateTraits;
use crate::types::*;
use crate::macros::*;

/// The operations which can be performed on either the state vector or density matrix describing the quantum state
/// as part of a quantum program.
#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    /// Measure a qubit.
    Measure(usize),
    /// Measure all qubits.
    MeasureAll,
    /// Reinitialise all qubits to their ground state.
    ReinitialiseAll,

    X(usize),
    Y(usize),
    Z(usize),
    H(usize),
    ArbitrarySingle(usize, Matrix2x2),
    S(usize),

    RX(usize, Angle),
    RY(usize, Angle),
    RZ(usize, Angle),
    R(usize, Angle, Angle, Angle),

    CNOT(usize, usize),
    CZ(usize, usize),
    CRZ(usize, usize, Angle),
    SISWAP(usize, usize),
    ArbitaryTwo(usize, usize, Matrix4x4),
    ISWAP(usize, usize),
    SWAP(usize, usize)
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operations::Measure(_qubit) => write!(f, "M"),
            Operations::MeasureAll => write!(f, ""),

            Operations::X(qubit) => write!(f, "X_{}", qubit),
            Operations::Y(qubit) => write!(f, "Y_{}", qubit),
            Operations::Z(qubit) => write!(f, "Z_{}", qubit),
            Operations::H(qubit) => write!(f, "H_{}", qubit),
            Operations::S(qubit) => write!(f, "S_{}", qubit),
            Operations::RX(qubit, angle) => write!(f, "RX_{}({})", qubit, angle),
            Operations::RY(qubit, angle) => write!(f, "RY_{}({})", qubit, angle),
            Operations::RZ(qubit, angle) => write!(f, "RZ_{}({})", qubit, angle),

            Operations::ArbitrarySingle(qubit, _) => write!(f, "U_{}", qubit),
            Operations::R(qubit, phi, theta, omega) => {
                write!(f, "R_{}({}, {}, {})", qubit, phi, theta, omega)
            }
            Operations::CNOT(control, target) => write!(f, "CNOT {} -> {}", control, target),
            Operations::CZ(control, target) => write!(f, "CZ {} -> {}", control, target),
            Operations::SISWAP(_, _) => write!(f, ""),
            Operations::ArbitaryTwo(_, _, _) => write!(f, ""),
            Operations::ISWAP(_, _) => write!(f, ""),
            _ => {
                write!(f, "")
            }
        }
    }
}

pub fn implement_gate<
    T: QuantumStateTraits
>(
    state: &mut T,
    gate: &Operations,
) {
    debug!("{:?}", gate);
    match gate {
        Operations::Measure(qubit) => state.measure(qubit),
        Operations::MeasureAll => state.measure_all(),
        Operations::ReinitialiseAll => state.reinitialise_all(),

        Operations::X(qubit) => state.single_qubit_gate(qubit, &SIGMA_X),
        Operations::Y(qubit) => state.single_qubit_gate(qubit, &SIGMA_Y),
        Operations::Z(qubit) => state.single_qubit_gate(qubit, &SIGMA_Z),
        Operations::S(qubit) => state.single_qubit_gate(qubit, &S),
        Operations::H(qubit) => state.single_qubit_gate(qubit, &H),

        Operations::RX(qubit, angle) => {
            let u = &IDENTITY * c!((angle / 2.).cos(), 0.) - &SIGMA_X * c!(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Operations::RY(qubit, angle) => {
            let u = &IDENTITY * c!((angle / 2.).cos(), 0.) - &SIGMA_Y * c!(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }

        Operations::RZ(qubit, angle) => {
            let u = &IDENTITY * c!((angle / 2.).cos(), 0.) - &SIGMA_Z * c!(0., (angle / 2.).sin());
            state.single_qubit_gate(qubit, &u)
        }
        Operations::R(qubit, phi, theta, omega) => {
            let (c_theta, s_theta) = ((theta / 2.).cos(), (theta / 2.).sin());
            let (c_plus, s_plus) = (((phi + omega) / 2.).cos(), ((phi + omega) / 2.).sin());
            let (c_minus, s_minus) = (((phi - omega) / 2.).cos(), ((phi - omega) / 2.).sin());

            let u: Matrix2x2 = matrix![
                c!(c_plus, -s_plus) * c_theta, -c!(c_minus, s_minus) * s_theta;
                c!(c_minus, -s_minus) * s_theta,  c!(c_plus, s_plus) * c_theta
            ];
            state.single_qubit_gate(qubit, &u)
        }
        Operations::ArbitrarySingle(qubit, u) => state.single_qubit_gate(qubit, u),
        Operations::CNOT(control, target) => state.two_qubit_gate(target, control, &CNOT),
        Operations::CZ(control, target) => state.two_qubit_gate(target, control, &CZ),
        Operations::CRZ(control, target, angle) => {
            let u: Matrix4x4 = matrix![
                c!(1., 0.), c!(0., 0.), c!(0., 0.), c!(0., 0.);
                c!(0., 0.), c!(1., 0.), c!(0., 0.), c!(0., 0.);
                c!(0., 0.), c!(0., 0.), c!(1., 0.), c!(0., 0.);
                c!(0., 0.), c!(0., 0.), c!(0., 0.), c!(angle.cos(), angle.sin());
            ];
            state.two_qubit_gate(target, control, &u)
        },
        Operations::ISWAP(control, target) => state.two_qubit_gate(target, control, &ISWAP),
        Operations::SISWAP(control, target) => state.two_qubit_gate(target, control, &SISWAP),
        Operations::SWAP(control, target) => state.two_qubit_gate(target, control, &SWAP),
        Operations::ArbitaryTwo(control, target, u) => state.two_qubit_gate(control, target, u),
    }
}

pub fn which_qubits(gate: &Operations) -> Vec<&usize> {
    match gate {
        Operations::Measure(qubit) => vec![qubit],
        Operations::MeasureAll => vec![],
        Operations::ReinitialiseAll => vec![],

        Operations::X(qubit) => vec![qubit],
        Operations::Y(qubit) => vec![qubit],
        Operations::Z(qubit) => vec![qubit],
        Operations::S(qubit) => vec![qubit],
        Operations::H(qubit) => vec![qubit],

        Operations::RX(qubit, _) => vec![qubit],
        Operations::RY(qubit, _) => vec![qubit],
        Operations::RZ(qubit, _) => vec![qubit],
        Operations::R(qubit, _, _, _) => vec![qubit],
        Operations::ArbitrarySingle(qubit, _) => vec![qubit],

        Operations::CZ(control, target) => vec![control, target],
        Operations::CNOT(control, target) => vec![control, target],
        Operations::CRZ(control, target, _) => vec![control, target],
        Operations::ISWAP(control, target) => vec![control, target],
        Operations::SISWAP(control, target) => vec![control, target],
        Operations::SWAP(control, target) => vec![control, target],
        Operations::ArbitaryTwo(control, target, _) => vec![control, target],
    }
}
