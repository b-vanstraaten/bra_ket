use nalgebra::{matrix, point};
use simulator::*;

// pretty assertions for human readability


fn crot(control:usize, target:usize, k: usize) -> Operation {
    let phase =2. * PI / ((2 << k) as R);
    let U: Matrix4x4 = matrix![
        C::new(1., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(1., 0.), C::new(0., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(1., 0.), C::new(0., 0.);
        C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(phase.cos(), phase.sin());
    ];
    Operation::ArbitaryTwo(control, target, U)
}

#[test]
fn quantum_fourier_transform_3_qubit() {
    let n = 3;
    let mut program = Program::new();
    program.h(0);
    program.h(1);
    program.h(2);

    program.h(0);
    program.add_gate(crot(1, 0, 2));
    program.add_gate(crot(2, 0, 3));
    program.h(2);
    program.add_gate(crot(2, 1, 2));
    program.h(2);

    let mut state = StateVector::new(n);
    program.run(&mut state);

    println!("{}", state.state_vector);

}