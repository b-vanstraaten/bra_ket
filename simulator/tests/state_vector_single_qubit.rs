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