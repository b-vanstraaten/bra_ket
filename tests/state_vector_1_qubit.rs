use nalgebra::{dvector, ComplexField};
use rand::{thread_rng, Rng};

use bra_ket::*;
use std::f64::consts::{PI, SQRT_2};

// pretty assertions for human readability

/// tests the x gate on a single qubit
#[test]
fn x0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * &PI * range.gen::<Angle>();
        let mut state = StateVector::new(1);

        let mut program = Program::new();
        program.rx(0, angle);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let other_state =
            StateVector::from(dvector![c!(c, 0.), c!(0., -s)]);

        assert_eq!(&state, &other_state);
    }
}

#[test]
fn y0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * &PI * range.gen::<Angle>();
        let mut state = StateVector::new(1);

        let mut program = Program::new();
        program.ry(0, angle);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let other_state =
            StateVector::from(dvector![c!(c, 0.), c!(s, 0.)]);

        assert_eq!(&state, &other_state);
    }
}

#[test]
fn measure() {
    let mut program = Program::new();

    program.reinitialise_all();
    program.h(0);
    program.measure(0);
    let mut state = StateVector::new(1);

    let n = 100;
    let mut p_estimated = 0.;

    for _ in 1..n {
        program.run(&mut state);
        p_estimated += state.state_vector[0].modulus_squared() / (n as Real);
    }

    let p_required = 0.5;
    let std = (p_required * (1. - &p_required) / (n as Real)).sqrt();

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
    let mut state = StateVector::new(1);

    program.h(0);
    program.measure_all();
    program.reinitialise_all();

    let n = 100;
    for _ in 1..n {
        program.run(&mut state);


    }
    let p_required = 0.5;
    let _std = (p_required * (1. - &p_required) / (n as Real)).sqrt();
    // let p_estimated = state.classical_register[0] / (n as R);

    // assert!(
    //     (p_estimated - &p_required).abs() < 3. * std,
    //     "estimated probabily {} not consistent with the required {}",
    //     p_estimated,
    //     p_required
    // )
}


