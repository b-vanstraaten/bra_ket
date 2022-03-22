use nalgebra::{dmatrix, dvector};
use rand::{thread_rng, Rng};
use test_log::test;

use zx::*;

// pretty assertions for human readability

/// tests the x gate on a single qubit
#[test]
fn measure() {
    let number_of_qubits: usize = 1;
    let angle = PI;

    let mut program = ProgramStateVector::new(number_of_qubits);
    program.measure(0);
    program.run();
}