use itertools::iproduct;
use log::debug;
use nalgebra::matrix;

use crate::index_swapping::*;
use crate::types::*;
use crate::State;

#[derive(Debug)]
pub enum Gate {
    Measure(Qubit),
    X(Qubit),
    Y(Qubit),
    Z(Qubit),
    H(Qubit),
    ArbitarySingle(Qubit, Matrix2x2),
    S(Qubit),

    RX(Qubit, Angle),
    RY(Qubit, Angle),
    RZ(Qubit, Angle),
    R(Qubit, Angle, Angle, Angle),

    CNOT(Qubit, Qubit),
    SISWAP(Qubit, Qubit),
    ArbitaryTwo(Qubit, Qubit, Matrix4x4),
    ISWAP(Qubit, Qubit),
}

pub fn implement_gate(state: &mut State, gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => state.measure(qubit),

        Gate::X(qubit) => x(state, qubit),
        Gate::Y(qubit) => y(state, qubit),
        Gate::Z(qubit) => z(state, qubit),
        Gate::S(qubit) => s(state, qubit),
        Gate::H(qubit) => h(state, qubit),

        Gate::RX(qubit, angle) => rx(state, qubit, angle),
        Gate::RY(qubit, angle) => ry(state, qubit, angle),
        Gate::RZ(qubit, angle) => rz(state, qubit, angle),
        Gate::R(qubit, omega, theta, phi) => r(state, qubit, omega, theta, phi),
        Gate::ArbitarySingle(qubit, u) => state.single_qubit_gate(qubit, u),

        Gate::CNOT(control, target) => cnot(state, control, target),
        Gate::ISWAP(control, target) => iswap(state, control, target),
        Gate::SISWAP(control, target) => siswap(state, control, target),
        Gate::ArbitaryTwo(control, target, u) => state.two_qubit_gate(control, target, u),
    }
}

fn x(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_X;
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn rx(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_X * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn y(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_Y;
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn ry(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Y * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn z(state: &mut State, qubit: &Qubit) {
    let u = SIGMA_Z;
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn rz(state: &mut State, qubit: &Qubit, angle: &Angle) {
    let u = IDENTITY * C::new((angle / 2.).cos(), 0.) - SIGMA_Z * C::new(0., (angle / 2.).sin());
    debug!("u:\n{}", u);
    state.single_qubit_gate(qubit, &u)
}

fn h(state: &mut State, qubit: &Qubit) {
    debug!("u:\n{}", H);
    state.single_qubit_gate(qubit, &H)
}

// s gate = root Z gate. Pi/2 rotation around Z axis.
fn s(state: &mut State, qubit: &Qubit) {
    debug!("u:\n{}", S);
    state.single_qubit_gate(qubit, &S)
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
    state.single_qubit_gate(qubit, &u)
}

fn cnot(state: &mut State, control: &Qubit, target: &Qubit) {
    state.two_qubit_gate(target, control, &CNOT);
}

fn siswap(state: &mut State, control: &Qubit, target: &Qubit) {
    state.two_qubit_gate(target, control, &SISWAP)
}

fn iswap(state: &mut State, control: &Qubit, target: &Qubit) {
    state.two_qubit_gate(target, control, &ISWAP)
}
