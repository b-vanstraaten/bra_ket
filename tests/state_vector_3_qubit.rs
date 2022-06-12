use nalgebra::dvector;
use bra_ket::*;

#[test]
fn ghz_0() {
    let mut state = StateVector::new(3);
    let mut program = Program::new();

    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);
    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        c!(1. / SQRT_2, 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(1. / SQRT_2, 0.)
    ]);
    assert_eq!(&required_state, &state);
}

#[test]
fn ghz_1() {
    let mut state = StateVector::new(3);
    let mut program = Program::new();

    program.h(1);
    program.cnot(1, 0);
    program.cnot(1, 2);
    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        c!(1. / SQRT_2, 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(1. / SQRT_2, 0.)
    ]);
    assert_eq!(&required_state, &state);
}

#[test]
fn ghz_2() {
    let mut state = StateVector::new(3);
    let mut program = Program::new();

    program.h(2);
    program.cnot(2, 0);
    program.cnot(2, 1);
    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        c!(1. / SQRT_2, 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(1. / SQRT_2, 0.)
    ]);
    assert_eq!(&required_state, &state);
}

#[test]
fn deutsch_jozsa_balanced() {
    let mut state = StateVector::new(3);
    let mut program = Program::new();

    program.x(2);
    program.h(0);
    program.h(1);
    program.h(2);

    program.cnot(0, 2);
    program.cnot(1, 2);

    program.h(0);
    program.h(1);

    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(1. / SQRT_2, 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(0., 0.),
        c!(-1. / SQRT_2, 0.)
    ]);
    assert_eq!(&required_state, &state);
}
