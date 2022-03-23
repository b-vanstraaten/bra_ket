use std::os::linux::raw::stat;
use nalgebra::{dmatrix, dvector};
use rand::{thread_rng, Rng};
use test_log::test;

use simulator::*;

// pretty assertions for human readability

/// tests the x gate on a single qubit
// #[test]
fn x0() {
    let mut range = thread_rng();
    for _ in 1..10 {
        let angle = 2. * PI * range.gen::<Angle>();
        let mut state = StateVector::new(1);

        let mut program = Program::new();
        program.run(&mut state);

        let (c, s) = ((angle / 2.).cos(), (angle / 2.).sin());

        let other_state = StateVector::new_from_state_vector(dvector![
            C::new(c, 0.), C::new(s, 0.)
        ]);

        assert_approximately_equal_vector(&state, &other_state);

    }

}

#[test]
fn measure() {
    let mut state = StateVector::new(3);

    let mut program = Program::new();
    program.measure(1);
    program.run(&mut state);
    // program.draw();

    println!("State vector {}", state.state_vector);
}

#[test]
fn x1() {
    let mut state = StateVector::new(2);

    let mut program = Program::new();
    program.x(1);
    program.run(&mut state);

    let other_state = StateVector::new_from_state_vector(dvector![
            C::new(0., 0.), C::new(0., 0.), C::new(1., 0.), C::new(0., 0.)
        ]);

    assert_approximately_equal_vector(&state, &other_state);

    println!("State vector {}", state.state_vector);
    println!("Other state {}", other_state.state_vector);
}
