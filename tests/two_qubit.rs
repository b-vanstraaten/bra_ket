use nalgebra::{dmatrix, dvector, DMatrix};
use test_log::test; // pretty assertions for human readability
use zx::*;

#[test]
fn two_qubit_cnot_control_0() {
    let number_of_qubits: usize = 2;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.cnot(0, 1);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    };
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_cnot_control_1() {
    let number_of_qubits: usize = 2;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.cnot(1, 0);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    };
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_hadamard() {
    let number_of_qubits: usize = 2;

    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.h(1);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
            C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
            C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
            C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
        ],
    };
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_x_gate() {
    /// tests the x gate on two qubits
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.x(0, PI);
    program.x(1, PI);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.);
        ],
    };
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_measure() {
    /// tests the x gate on two qubits
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.x(0, PI / 2.);
    program.x(1, PI / 2.);
    program.measure(0);
    program.measure(1);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0.25, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0.25, 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0.25, 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.25, 0.);
        ],
    };

    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_y_gate() {
    /// tests the y gate an two qubits
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.y(1, PI);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(1., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    };

    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_z_gate() {
    /// tests the z gate on two qubis
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.z(0, PI);
    program.run();

    let required_state = State {
        number_of_qubits,
        density_matrix: dmatrix![
            C::new(0.5, 0.), C::new(-0.5, 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(-0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    };
    assert_approximately_equal(required_state, program.state);
}
