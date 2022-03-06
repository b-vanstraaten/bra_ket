use nalgebra::{dmatrix, dvector};
use pretty_assertions::assert_eq;
use test_log::test; // pretty assertions for human readability

use zx::*;

#[test]
fn one_qubit_x_gate_pi() {
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_density_matrix = dmatrix![
        C::new(c * c, 0.), C::new(0.,-c * s);
        C::new(0., c * s), C::new(s * s, 0.);
    ];

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };
    assert_approximately_equal(required_state, program.state)
}

#[test]
fn one_qubit_x_gate_pi_half() {
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let angle = PI / 2.;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_density_matrix = dmatrix![
        C::new(c * c, 0.), C::new(0.,-c * s);
        C::new(0., c * s), C::new(s * s, 0.);
    ];

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn one_qubit_y_gate_pi() {
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.y(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_density_matrix = dmatrix![
        C::new(c * c, 0.), C::new(c * s, 0.);
        C::new(c * s, 0.), C::new(s * s, 0.);
    ];
    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn one_qubit_y_gate_pi_half() {
    /// tests the x gate on a single qubit
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let angle = PI / 2.;

    let mut program = Program::new(number_of_qubits);
    program.y(0, angle);
    program.run();

    let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

    let required_density_matrix = dmatrix![
        C::new(c * c, 0.), C::new(-c * s, 0.);
        C::new(-c * s, 0.), C::new(s * s, 0.);
    ];
    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };

    assert_approximately_equal(required_state, program.state)
}

#[test]
fn one_qubit_z_gate_pi() {
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let mut program = Program::new(number_of_qubits);
    program.z(0, PI);
    program.run();

    let required_density_matrix =
        DensityMatrix::from_diagonal(&dvector!(C::new(1., 0.), C::new(0., 0.)));

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };

    assert_approximately_equal(required_state, program.state)
}

fn one_qubit_z_gate_pi_half() {
    /// tests the x gate on a single qubit
    let number_of_qubits: usize = 1;
    let mut program = Program::new(number_of_qubits);
    program.z(0, PI / 2.);
    program.run();

    let required_density_matrix =
        DensityMatrix::from_diagonal(&dvector!(C::new(1., 0.), C::new(0., 0.)));

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };

    assert_approximately_equal(required_state, program.state)
}
