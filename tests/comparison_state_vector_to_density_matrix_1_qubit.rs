use rand::{thread_rng, Rng};
use simulator::*;

/// tests the x gate on a single qubit
#[test]
fn x0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * &PI * range.gen::<Angle>();
        let mut program = Program::new();
        program.rx(0, angle);

        let mut state = StateVector::new(1);
        let mut density = DensityMatrix::new(1);

        program.run(&mut state);
        program.run(&mut density);

        assert_eq!(&density, &DensityMatrix::from(state))
    }
}

/// tests the x gate on a single qubit
#[test]
fn y0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * &PI * range.gen::<Angle>();
        let mut program = Program::new();
        program.ry(0, angle);

        let mut state = StateVector::new(1);
        let mut density = DensityMatrix::new(1);

        program.run(&mut state);
        program.run(&mut density);

        assert_eq!(&density, &DensityMatrix::from(state))
    }
}

#[test]
fn h0() {
    let mut program = Program::new();
    program.h(0);

    let mut state = StateVector::new(1);
    let mut density = DensityMatrix::new(1);

    program.run(&mut state);
    program.run(&mut density);

    assert_eq!(&density, &DensityMatrix::from(state))
}
