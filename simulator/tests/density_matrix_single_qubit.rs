use nalgebra::{dmatrix, dvector};
use rand::{thread_rng, Rng};
use test_log::test;

use simulator::*;

// pretty assertions for human readability

/// tests the x gate on a single qubit
#[test]
fn x0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * PI * range.gen::<Angle>();
        let mut state = DensityMatrix::new(1);

        let mut program = Program::new();
        program.rx(0, angle);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());
        let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
            C::new(c * c, 0.), C::new(0.,c * s);
            C::new(0., -c * s), C::new(s * s, 0.);
        ]);
        assert_approximately_equal(&required_state, &state)
    }
}

/// tests the x gate on a single qubit
#[test]
fn y0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * PI * range.gen::<Angle>();

        let mut state = DensityMatrix::new(1);

        let mut program = Program::new();
        program.ry(0, angle);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
            C::new(c * c, 0.), C::new(c * s, 0.);
            C::new(c * s, 0.), C::new(s * s, 0.);
        ]);

        assert_approximately_equal(&required_state, &state)
    }
}

/// tests the z gate on a single qubit
#[test]
fn z0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * PI * range.gen::<Angle>();

        let mut state = DensityMatrix::new(1);

        let mut program = Program::new();
        program.ry(0, angle);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
            C::new(c * c, 0.), C::new(c * s, 0.);
            C::new(c * s, 0.), C::new(s * s, 0.);
        ]);

        assert_approximately_equal(&required_state, &state)
    }
}

#[test]
fn xy_commutation() {
    let number_of_qubits: usize = 1;

    let mut state = DensityMatrix::new(number_of_qubits);
    let mut other_state = DensityMatrix::new(number_of_qubits);

    let mut other_program = Program::new();
    let mut program = Program::new();
    program.x(0);
    program.y(0);
    program.run(&mut state);

    other_program.z(0);
    other_program.run(&mut other_state);

    assert_approximately_equal(&state, &other_state)
}

#[test]
fn xz_commutation() {
    let angle = PI;

    let mut state = DensityMatrix::new(1);
    let mut other_state = DensityMatrix::new(1);

    let mut program = Program::new();
    let mut other_program = Program::new();

    program.rx(0, angle);
    program.rz(0, angle);
    program.run(&mut state);

    other_program.ry(0, 3. * angle);
    other_program.run(&mut other_state);

    assert_approximately_equal(&state, &other_state)
}

/// tests the x gate on a single qubit
#[test]
fn m0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let mut state = DensityMatrix::new(1);
        let mut program = Program::new();

        let angle = 2. * PI * range.gen::<Angle>();

        program.rx(0, angle);
        program.measure(0);
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(c * c, 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(s * s, 0.);]);

        assert_approximately_equal(&required_state, &state)
    }
}

#[test]
fn h0() {
    let mut state = DensityMatrix::new(1);
    let mut program = Program::new();

    program.h(0);
    program.run(&mut state);

    let required_density_matrix = dmatrix![
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
    ];

    let required_state = DensityMatrix::new_from_density_matrix(dmatrix![
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
        C::new(1. / 2., 0.), C::new(1. / 2., 0.);
    ]);

    assert_approximately_equal(&required_state, &state)
}

#[test]
fn r0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let mut state = DensityMatrix::new(1);
        let mut other_state = DensityMatrix::new(1);

        let mut program = Program::new();
        let mut other_program = Program::new();

        let phi = 2. * PI * range.gen::<Angle>();
        let theta = 2. * PI * range.gen::<Angle>();
        let omega = 2. * PI * range.gen::<Angle>();

        program.r(0, phi, theta, omega);
        program.run(&mut state);

        other_program.rz(0, phi);
        other_program.ry(0, theta);
        other_program.rz(0, omega);
        other_program.run(&mut other_state);

        assert_approximately_equal(&state, &other_state)
    }
}

#[test]
fn s() {
    let mut state = DensityMatrix::new(1);
    let mut other_state = DensityMatrix::new(1);

    let mut program = Program::new();
    let mut other_program = Program::new();

    program.s(0);
    program.s(0);
    program.run(&mut state);

    other_program.z(0);
    other_program.run(&mut other_state);

    assert_approximately_equal(&state, &other_state)
}
