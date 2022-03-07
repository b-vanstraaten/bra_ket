use nalgebra::{dmatrix, dvector};
use test_log::test;

use zx::*;

// pretty assertions for human readability

/// tests the x gate on a single qubit
#[test]
fn one_qubit_x_gate_pi() {
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

/// tests the x gate on a single qubit
#[test]
fn one_qubit_x_gate_pi_half() {
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

/// tests the x gate on a single qubit
#[test]
fn one_qubit_y_gate_pi() {
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

/// tests the y gate on a single qubit
#[test]
fn one_qubit_y_gate_pi_half() {
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

/// tests the z gate on a single qubit
#[test]
fn one_qubit_z_gate_pi() {
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

/// tests the x gate on a single qubit
#[test]
fn one_qubit_z_gate_pi_half() {
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

#[test]
fn single_qubit_xy_commutation() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.y(0, angle);
    program.run();

    let mut other_program = Program::new(number_of_qubits);
    other_program.z(0, angle);
    other_program.run();

    assert_approximately_equal(program.state, other_program.state)
}

#[test]
fn single_qubit_xz_commutation() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.z(0, angle);
    program.run();

    let mut other_program = Program::new(number_of_qubits);
    other_program.y(0, 3. * angle);
    other_program.run();

    assert_approximately_equal(program.state, other_program.state)
}
