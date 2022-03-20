use nalgebra::dmatrix;
use test_log::test;
// pretty assertions for human readability
use zx::*;

#[test]
fn h0() {

    let mut state = State::new(3);
    let mut program = Program::new();
    program.h(0);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h1() {

    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(1);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h2() {

    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(2);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h0_cnot01() {

    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(0);
    program.cnot(0, 1);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h0_cnot02() {

    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(0);
    program.cnot(0, 2);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h2_cnot21() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(2);
    program.cnot(2, 1);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn h2_cnot20() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(2);
    program.cnot(2, 0);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn ghz_0() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn ghz_1() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(1);
    program.cnot(1, 0);
    program.cnot(1, 2);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn ghz_2() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(2);
    program.cnot(2, 1);
    program.cnot(2, 0);
    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn hhh_measure() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.h(0);
    program.h(1);
    program.h(2);

    program.measure(0);
    program.measure(1);
    program.measure(2);

    program.run(&mut state);

    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.125, 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}

#[test]
fn deutsch_jozsa_balanced() {
    let mut state = State::new(3);
    let mut program = Program::new();
    
    program.rx(2, PI);
    program.h(0);
    program.h(1);
    program.h(2);

    program.cnot(0, 2);
    program.cnot(1, 2);

    program.h(0);
    program.h(1);

    program.measure(0);
    program.measure(1);

    program.run(&mut state);
    
    let required_state = State::new_from_density_matrix(
        dmatrix![
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(-0.5, 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
                C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(-0.5, 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0.5, 0.);
        ],
    );
    assert_approximately_equal(&required_state, &state);
}
