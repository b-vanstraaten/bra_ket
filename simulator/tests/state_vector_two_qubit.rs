use nalgebra::dvector;
use test_log::test;

use simulator::*;

// pretty assertions for human readability

#[test]
fn h0_cnot01() {
    let mut state = StateVector::new(2);
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);
    program.run(&mut state);

    let required_state = StateVector::new_from_state_vector(dvector![
            C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.)
        ]);

    assert_approximately_equal_vector(&required_state, &state);
}