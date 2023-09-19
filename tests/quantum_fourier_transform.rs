
use bra_ket::*;


#[test]
fn quantum_fourier_transform_n_qubit() {
    let n = 5;
    let mut init_program = Program::new();

    for i in 0..n {
        init_program.h(i);
    }
    init_program.z(0);
    init_program.z(1);

    let qft_program = qft(n);

    let mut state = StateVector::new(n);
    init_program.run(&mut state);
    let _init_state = state.clone();
    qft_program.run(&mut state);
}

