use nalgebra::dmatrix;
use simulator::*;
use test_log::test; // pretty assertions for human readability

#[test]
fn h0_cnot01() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
    ]);
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h0_cnot10() {
    let number_of_qubits: usize = 2;

    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.h(0);
    program.cnot(1, 0);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    ]);
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h0_h1() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.h(0);
    program.h(1);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
        C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
        C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
        C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.), C::new(0.25, 0.);
    ]);
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn x0_x1() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.rx(0, PI);
    program.rx(1, PI);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.);
    ]);
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn x0_x1_m0_m1() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.rx(0, PI / 2.);
    program.rx(1, PI / 2.);
    program.measure(0);
    program.measure(1);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0.25, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0.25, 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0.25, 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.25, 0.);
    ]);

    assert_approximately_equal(&required_state, &state);
}

#[test]
fn y1() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.ry(1, PI);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(1., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    ]);

    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h0_z1() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    program.h(0);
    program.rz(0, PI);
    program.run(&mut state);

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(0.5, 0.), C::new(-0.5, 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(-0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    ]);
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn iswap01() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    let mut other_state = DensityMatrix::new(2);
    let mut other_program = Program::new();

    program.iswap(0, 1);
    program.run(&mut state);

    // reference implementation from https://qiskit.org/documentation/stubs/qiskit.circuit.library.iSwapGate.html
    other_program.s(0);
    other_program.s(1);
    other_program.h(0);
    other_program.cnot(0, 1);
    other_program.cnot(1, 0);
    other_program.h(1);
    other_program.run(&mut other_state);

    assert_approximately_equal(&other_state, &state);
}

// https://qiskit.org/documentation/stubs/qiskit.circuit.library.iSwapGate.html
#[test]
fn root_iswap01() {
    let mut state = DensityMatrix::new(2);
    let mut program = Program::new();

    let mut other_state = DensityMatrix::new(2);
    let mut other_program = Program::new();

    program.siswap(0, 1);
    program.siswap(0, 1);
    program.run(&mut state);

    other_program.iswap(0, 1);
    other_program.run(&mut other_state);

    assert_approximately_equal(&other_state, &state);
}
