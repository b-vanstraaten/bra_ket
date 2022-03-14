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

    let other_program = Program::new(number_of_qubits);

    assert_approximately_equal(program.state, other_program.state)
}

#[test]
fn pure_state() {
    let number_of_qubits: usize = 1;
    let angle = PI / 3.;

    let mut program = Program::new(number_of_qubits);
    program.x(0, angle);
    program.run();

    // assert the state is pure
    assert!(program.state.is_pure());

    program.reset(); // reset the program to empty
    program.x(0, angle);
    program.measure(0);
    program.run();

    // assert that the state is not pure
    assert!(!program.state.is_pure())
}