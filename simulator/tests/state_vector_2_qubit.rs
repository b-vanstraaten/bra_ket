use nalgebra::{dvector, ComplexField};

use simulator::*;

#[test]
fn x0_cnot01() {
    let mut state = StateVector::new(2);
    let mut program = Program::new();
    program.x(0);
    program.cnot(0, 1);
    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(0., 0.),
        C::new(1., 0.)
    ]);

    assert_eq!(&required_state, &state);
}

#[test]
fn h0_h1() {
    let mut state = StateVector::new(2);
    let mut program = Program::new();
    program.h(0);
    program.h(1);
    program.run(&mut state);

    let required_state = StateVector::from(dvector![
        C::new(1. / 2., 0.),
        C::new(1. / 2., 0.),
        C::new(1. / 2., 0.),
        C::new(1. / 2., 0.)
    ]);

    assert_eq!(&required_state, &state);
}

#[test]
fn h0_h1_measure() {
    let mut program = Program::new();

    program.reset_all();
    program.h(0);
    program.h(1);
    program.measure(0);
    let mut state = StateVector::new(2);

    let n = 100;
    let mut p_estimated = 0.;

    for _ in 1..n {
        program.run(&mut state);
        p_estimated += state.state_vector[0].modulus_squared() / (n as R);
    }

    let p_required = 0.25;
    let std = (p_required * (1. - &p_required) / (n as R)).sqrt();

    assert!(
        (p_estimated - &p_required).abs() < 3. * std,
        "estimated probabily {} not consistent with the required {}",
        p_estimated,
        p_required
    )
}

#[test]
fn h0_cnot_measure_all() {
    let mut program = Program::new();

    program.reset_all();
    program.h(0);
    program.cnot(0, 1);
    program.measure_all();
    let mut state = StateVector::new(2);

    let n = 100;
    let mut p_estimated = 0.;

    for _ in 1..n {
        program.run(&mut state);
        p_estimated += state.state_vector[0].modulus_squared() / (n as R);
    }

    let p_required = 0.5;
    let std = (p_required * (1. - &p_required) / (n as R)).sqrt();

    assert!(
        (p_estimated - &p_required).abs() < 3. * std,
        "estimated probabily {} not consistent with the required {}",
        p_estimated,
        p_required
    )
}

#[test]
fn measure_all() {
    let mut program = Program::new();

    program.reset_all();
    program.h(0);
    program.h(1);
    program.measure_all();
    let mut state = StateVector::new(2);

    let n = 100;
    let mut p_estimated = 0.;

    for _ in 1..n {
        program.run(&mut state);
        p_estimated += state.state_vector[0].modulus_squared() / (n as R);
    }

    let p_required = 0.25;
    let std = (p_required * (1. - &p_required) / (n as R)).sqrt();

    assert!(
        (p_estimated - &p_required).abs() < 3. * std,
        "estimated probabily {} not consistent with the required {}",
        p_estimated,
        p_required
    )
}
