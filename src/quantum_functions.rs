use crate::program::Program;
use crate::types::{PI, Real};

fn qft_no_swap(mut program: Program, n: usize) -> Program {
    if n == 0 {return program}
    let n = n - 1;
    program.h(n);
    for qubit in 0..n {
        let k = n - qubit - 1;
        let angle = PI / (2 << k) as Real;
        program.crz(n, qubit, angle)
    }
    return qft_no_swap(program, n)
}

pub fn qft(n: usize) -> Program {
    let mut program = Program::new();
    program = qft_no_swap(program, n);
    let n_half = n - 1 / 2;
    if n > 1 {
        for i in 0..n_half {
        program.swap(i, n - i - 1)
    }}
    program
}