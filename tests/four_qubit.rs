use zx::*;

use nalgebra::dmatrix;
use test_log::test;

#[test]
fn h0() {
    let number_of_qubits: usize = 10;
    let mut program = Program::new(number_of_qubits);
    program.h(0);
    program.run();
}
