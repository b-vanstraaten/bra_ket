use simulator::*;

#[test]
fn grover_two_qubit() {
    let mut program = Program::new();

    program.h(0);
    program.h(1);

    program.cz(0, 1);

    program.h(0);
    program.h(1);

    program.z(0);
    program.z(1);
    program.cz(0, 1);
    program.h(0);
    program.h(1);

    // program.measure_all();

    let mut state = StateVector::new(2);
    program.run(&mut state);
    program.draw();
    println!("{}", state.state_vector);
}



