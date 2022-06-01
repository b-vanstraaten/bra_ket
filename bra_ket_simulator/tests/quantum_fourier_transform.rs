use nalgebra::matrix;
use simulator::*;

fn qft_no_swap(mut program: Program, n: usize) -> Program {
    if n == 0 {return program}
    let n = n - 1;
    program.h(n);
    for qubit in 0..n {
        let k = n - qubit - 1;
        let angle = PI / (2 << k) as R;
        program.crz(n, qubit, angle)
    }
    return qft_no_swap(program, n)
}

fn qft(mut program: Program, n: usize) -> Program {
    program = qft_no_swap(program, n);
    let n_half = n - 1 / 2;
    for i in 0..n_half {
        program.swap(i, n - i - 1)
    }
    program
}

#[test]
fn quantum_fourier_transform_n_qubit() {
    let n = 2;
    let mut init_program = Program::new();
    let mut qft_program = Program::new();

    for i in 0..n {
        init_program.h(i);
    }

    qft_program = qft(qft_program, n);


    let mut state = StateVector::new(n);
    init_program.run(&mut state);
    let init_state = state.clone();
    qft_program.run(&mut state);

    println!("{}", state.state_vector);

}

