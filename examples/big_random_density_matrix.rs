use bra_ket::*;
fn main() {
    let number_of_qubits = 14;

    let mut program = Program::new();

    for i in 0..number_of_qubits {
        program.h(i);
    }

    program.draw();

    let mut density_matrix = DensityMatrix::new(number_of_qubits);

    program.run(&mut density_matrix);

    println!("{}", density_matrix);
}