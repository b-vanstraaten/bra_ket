use nalgebra::dmatrix;
use test_log::test; // pretty assertions for human readability
use zx::*;

#[test]
fn h0_cnot01() {
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
fn h0_cnot10() {
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
fn h0_h1() {
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
fn x0_x1() {
    let number_of_qubits = 2;
    let mut program = Program::new(number_of_qubits);

    program.rx(0, PI);
    program.rx(1, PI);
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
fn x0_x1_m0_m1() {
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.rx(0, PI / 2.);
    program.rx(1, PI / 2.);
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
fn y1() {
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.ry(1, PI);
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
fn h0_z1 () {
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.rz(0, PI);
    program.run();
    program.draw();

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

#[test]
fn iswap01 () {

    // do the iswap gate
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);
    program.iswap(0, 1);

    // reference implementation from https://qiskit.org/documentation/stubs/qiskit.circuit.library.iSwapGate.html
    let mut other_program = Program::new(number_of_qubits);
    other_program.s(0);
    other_program.s(1);
    other_program.h(0);
    other_program.cnot(0, 1);
    other_program.cnot(1, 0);
    other_program.h(1);

    assert_approximately_equal(program.state, other_program.state)
}




// https://qiskit.org/documentation/stubs/qiskit.circuit.library.iSwapGate.html
#[test]
fn root_iswap01 () {
    let number_of_qubits: usize = 2;
    let mut program = Program::new(number_of_qubits);

    program.siswap(0, 1);
    program.siswap(0, 1);

    program.draw();

    let mut other_program = Program::new(number_of_qubits);

    other_program.iswap(0, 1);

    assert_approximately_equal(program.state, other_program.state);
}
