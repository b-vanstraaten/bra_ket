use nalgebra::{dmatrix, dvector};
use test_log::test;

use zx::*;


#[test]
fn reset_test() {
    let number_of_qubits: usize = 2;
    let angle = PI;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.x(1, angle);
    program.run();
    program.reset();

    let mut other_program = Program::new(number_of_qubits);

    assert_approximately_equal(program.state, other_program.state)
}