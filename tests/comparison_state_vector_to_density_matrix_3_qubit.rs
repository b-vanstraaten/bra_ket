// pretty assertions for human readability
use bra_ket::*;

#[test]
fn h0() {
    let mut program = Program::new();
    program.h(0);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h1() {
    let mut program = Program::new();
    program.h(1);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h2() {
    let mut program = Program::new();
    program.h(2);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h0_cnot01() {
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h0_cnot02() {
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 2);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h2_cnot21() {
    let mut program = Program::new();
    program.h(2);
    program.cnot(2, 1);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h2_cnot20() {
    let mut program = Program::new();
    program.h(2);
    program.cnot(2, 0);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn ghz_0() {
    let mut program = Program::new();

    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn ghz_1() {
    let mut program = Program::new();

    program.h(1);
    program.cnot(1, 0);
    program.cnot(1, 2);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn ghz_2() {
    let mut program = Program::new();

    program.h(2);
    program.cnot(2, 1);
    program.cnot(2, 0);

    let mut state = StateVector::new(3);
    let mut density = DensityMatrix::new(3);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}
