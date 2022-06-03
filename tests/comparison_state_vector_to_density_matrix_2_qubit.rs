use bra_ket::*;

// pretty assertions for human readability

#[test]
fn h0_cnot01() {
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h0_cnot10() {
    let mut program = Program::new();

    program.h(0);
    program.cnot(1, 0);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h0_h1() {
    let mut program = Program::new();
    program.h(0);
    program.h(1);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn x0_x1() {
    let mut program = Program::new();

    program.rx(0, PI);
    program.rx(1, PI);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn y1() {
    let mut program = Program::new();

    program.ry(1, PI);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}

#[test]
fn h0_z1() {
    let mut program = Program::new();

    program.h(0);
    program.rz(0, PI);

    let mut state = StateVector::new(2);
    let mut density = DensityMatrix::new(2);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}
