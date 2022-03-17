use std::cmp::Ordering;
use nalgebra::{ComplexField, matrix};

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
    X(Qubit),
    Y(Qubit),
    Z(Qubit),
    H(Qubit),
    S(Qubit),

    RX(Qubit, Angle),
    RY(Qubit, Angle),
    RZ(Qubit, Angle),
    R(Qubit, Angle, Angle, Angle),

    CNOT(Qubit, Qubit),
    ISWAP(Qubit, Qubit),
    SISWAP(Qubit, Qubit)
}

pub fn implement_gate(state: &mut State, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => measure(state, qubit),

        Gate::X(qubit) => x(state, qubit),
        Gate::Y(qubit) => y(state, qubit),
        Gate::Z(qubit) => z(state, qubit),
        Gate::S(qubit) => s(state, qubit),

        Gate::RX(qubit, angle) => rx(state, qubit, angle),
        Gate::RY(qubit, angle) => ry(state, qubit, angle),
        Gate::RZ(qubit, angle) => rz(state, qubit, angle),
        Gate::R(qubit, omega, theta , phi) => r(state, qubit, omega, theta, phi),
        Gate::H(qubit) => h(state, qubit),

        Gate::CNOT(control, target) => cnot(state, control, target),
        Gate::ISWAP(control, target) => iswap(state, control, target),
        Gate::SISWAP(control, target) => siswap(state, control, target),
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

fn two_qubit_gate(state: &mut State, target: &Qubit, control: &Qubit, u: Matrix4x4) {
    debug!("density matrix before:\n{}", state.density_matrix);

    let swap_qubits: Box<dyn Fn(usize) -> Qubit> = match (target, control) {
        // do nothing it is already correct
        (0, 1) => Box::new(|x| x),
        // swap the two bit values
        (1, 0) => Box::new(|x| swap_two_bits(x, (&0, &1))),
        // it is only necessary to swap bit_1
        (0, _) => Box::new(|x: usize| swap_two_bits(x, (&1, &control))),
        // it is only necessary to swap bit_0
        (_, 1) => Box::new(|x: usize| swap_two_bits(x, (&0, &target))),
        // swap bits 0 and 1 then swap bit 0 with bit_1
        (1, _) => Box::new(|x: usize| swap_four_bits(x, [(&0, &1), (&0, &control)])),
        // swap bits 0 and 1 then swap bit 1 with qubit_0
        (_, 0) => Box::new(|x: usize| swap_four_bits(x, [(&0, &1), (&1, &target)])),
        // swap both bits
        (_, _) => Box::new(|x: usize| swap_four_bits(x, [(&0, &target), (&1, &control)]))
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

fn x(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_X;
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn rx(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_X * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn y(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_Y;
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn ry(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Y * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn z(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_Z;
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn rz(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Z * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn h(state: &mut State, qubit: &Qubit) {
    debug!("u:\n{}", H);
    single_qubit_gate(state, qubit, H)
}

fn r(state: &mut State, qubit: &Qubit, phi: &Angle, theta: &Angle, omega: &Angle) {



    let (c_theta, s_theta) = ((theta / 2.).cos(), (theta / 2.).sin());
    let (c_plus, s_plus) = (((phi + omega) / 2.).cos(), ((phi + omega) / 2.).sin());
    let (c_minus, s_minus) = (((phi - omega) / 2.).cos(), ((phi - omega) / 2.).sin());

    let u: Matrix2x2 = matrix![
        C::new(c_plus, -s_plus) * c_theta, -C::new(c_minus, s_minus) * s_theta;
        C::new(c_minus, -s_minus) * s_theta,  C::new(c_plus, s_plus) * c_theta
    ];

    debug!("u:\n{}", u);
    single_qubit_gate(state, qubit, u)
}

fn cnot(state: &mut State, control: &Qubit, target: &Qubit) {
    two_qubit_gate(state, target, control, CNOT);
}

fn siswap(state: &mut State, control: &Qubit, target: &Qubit) {
    two_qubit_gate(state, target, control, SISWAP)
}

fn iswap(state: &mut State, control: &Qubit, target: &Qubit) {
    two_qubit_gate(state, target, control, ISWAP)
}

// s gate = root Z gate. Pi/2 rotation around Z axis.
fn s(state: &mut State, qubit: &Qubit) {
    single_qubit_gate(state, qubit, S)
}

