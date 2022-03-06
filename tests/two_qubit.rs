use nalgebra::dvector;
use test_log::test; // pretty assertions for human readability

use zx::*;

// hello

#[test]
fn two_qubit_x_gate() {
    /// tests the x gate on two qubits
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.x(0, PI);
    program.x(1, PI);
    program.run();

    let required_density_matrix = DensityMatrix::from_diagonal(&dvector!(
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(1., 0.)
    ));

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
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

    let required_density_matrix = DensityMatrix::from_diagonal(&dvector!(
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(1., 0.),
        C::new(0., 0.)
    ));

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };
    assert_approximately_equal(required_state, program.state);
}

#[test]
fn two_qubit_z_gate() {
    /// tests the z gate on two qubis
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);
    program.z(0, PI);
    program.z(1, PI);

    program.run();

    let required_density_matrix = DensityMatrix::from_diagonal(&dvector!(
        C::new(1., 0.),
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(0., 0.)
    ));

    let required_state = State {
        number_of_qubits,
        density_matrix: required_density_matrix,
    };
    assert_approximately_equal(required_state, program.state);
}
