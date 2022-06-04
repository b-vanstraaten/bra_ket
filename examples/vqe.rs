use std::iter::zip;
use nalgebra::matrix;
use bra_ket::*;
///https://joshuagoings.com/2020/08/20/VQE/

fn make_ansatz(theta: Angle) -> Program {
    let mut ansatz = Program::new();
    ansatz.reinitialise_all();
    ansatz.x(0);

    ansatz.rx(0, 3. * PI / 2.);
    ansatz.ry(1, PI / 2.);

    ansatz.cnot(1, 0);
    ansatz.rz(0, theta);
    ansatz.cnot(1, 0);

    ansatz.rx(0, PI / 2.);
    ansatz.ry(1,  3. * PI / 2.);
    ansatz
}

fn make_measurement_programs() -> Vec<Program> {
    let mut measure_z0 = Program::new();

    let mut measure_z1  = Program::new();
    measure_z1.swap(0, 1);

    let mut measure_z0_z1 = Program::new();
    measure_z0_z1.cnot(1, 0);

    let mut measure_x0_x1 = Program::new();
    measure_x0_x1.h(0);
    measure_x0_x1.h(1);
    measure_x0_x1.cnot(1, 0);

    let mut measure_y0_y1 = Program::new();
    measure_y0_y1.rz(0, - PI / 2.);
    measure_y0_y1.h(1);
    measure_y0_y1.rz(1, - PI / 2.);
    measure_y0_y1.cnot(1, 0);

    vec![measure_z0, measure_z1, measure_z0_z1, measure_x0_x1, measure_y0_y1]
}

fn main() {
    let evaluate_energy = |theta| {
        let ansatz = make_ansatz(theta);
        let measurement_programs = make_measurement_programs();
        let coefficients = vec![0.3435, -0.4347, 0.5716, 0.0910, 0.0910];
        let mut state = StateVector::new(2);

        let mut energy = 0.7055696146 - 0.4804;
        for (coefficient, measurement_program) in zip(coefficients, measurement_programs) {
            let full_program = ansatz.to_owned() + measurement_program;
            full_program.run(&mut state);
            let expectation = state.get_expectation(&0);
            energy += coefficient * expectation;
        }
        energy
    };

    println!("{}", evaluate_energy(0.11))

}