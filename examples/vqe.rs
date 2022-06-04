use std::iter::zip;
use itertools::enumerate;
use nalgebra::{DVector};
use bra_ket::*;

use textplots::{Chart, Plot, Shape};


///https://joshuagoings.com/2020/08/20/VQE/

/// makes the ansatz circuit to prepare the quantum state
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

/// makes the measurement circuits which permit measuring the pauli string in the hamiltonian.
fn make_measurement_programs() -> Vec<Program> {
    let measure_z0 = Program::new();

    let mut measure_z1  = Program::new();
    measure_z1.swap(0, 1);

    let mut measure_z0_z1 = Program::new();
    measure_z0_z1.cnot(1, 0);

    let mut measure_x0_x1 = Program::new();
    measure_x0_x1.h(0);
    measure_x0_x1.h(1);
    measure_x0_x1.cnot(1, 0);

    let mut measure_y0_y1 = Program::new();
    measure_y0_y1.h(0);
    measure_y0_y1.rz(0, - PI / 2.);
    measure_y0_y1.h(1);
    measure_y0_y1.rz(1, - PI / 2.);
    measure_y0_y1.cnot(1, 0);

    vec![measure_z0, measure_z1, measure_z0_z1, measure_x0_x1, measure_y0_y1]
}

/// calculates the energy of the ansatz state for a given theta in the ansatz circuit.
fn evaluate_energy(theta: Real) -> Real {
    let ansatz = make_ansatz(theta);
    let measurement_programs = make_measurement_programs();
    let coefficients = vec![0.3435, -0.4347, 0.5716, 0.0910, 0.0910];

    let mut state = StateVector::new(2);
    let expectation: Real = zip(coefficients, measurement_programs).map(
        |(coefficient, measurement_program)| {
            let full_program = ansatz.to_owned() + measurement_program;
            full_program.run(&mut state);
            let expectation = state.get_expectation(&0);
            coefficient * expectation
        }
    ).sum();
    expectation + 0.7055696146 - 0.4804
}

fn linspace(start: Real, end: Real, n: Int) -> DVector<Real> {
    let mut x = DVector::zeros(n);
    for i in 0..n {
        x[i] = start + (end - start) * (i as Real / n as Real)
    }
    x
}

fn main() {
    let mut ansatz = make_ansatz(0.);
    ansatz.draw();


    println!("Energy as function of theta");
    Chart::new(200, 32, -PI as f32, PI as f32)
        .lineplot(
            &Shape::Continuous(
                Box::new(|theta| evaluate_energy(theta as Real) as f32)
            ))
        .display();

    let n = 100;
    let theta_s = linspace(- PI, PI, n);
    let mut energies = DVector::zeros(n);

    for (i, theta) in enumerate(theta_s.iter()) {
        energies[i] =  evaluate_energy(theta.to_owned());
    }
    let (arg_min, energy_min) = energies.argmin();
    let theta_min = theta_s[arg_min];

    println!("Minimum theta: {} (radians)", theta_min as f32);
    println!("Minimum energy: {} (hartrees)", energy_min as f32)
}