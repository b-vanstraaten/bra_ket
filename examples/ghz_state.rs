use bra_ket::*;
fn main() {

    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);

    // drawing the program
    program.draw();

    // creating the state vector to hold the quantum state
    let mut state = StateVector::new(3);

    // running the program to evolve the state vector from its
    //initial state of |000> to (1 / SQRT_2) *(|000> + |111>)
    program.run(&mut state);

    // printing the final state
    println!("{}", state);
}