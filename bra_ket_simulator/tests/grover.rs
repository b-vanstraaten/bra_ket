use simulator::*;

#[test]
fn grover_two_qubit() {
    let mut state = DensityMatrix::new(2);
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

    program.measure_all();

    program.run(&mut state);
    // program.draw();
    println!("{}", state.density_matrix);
}



