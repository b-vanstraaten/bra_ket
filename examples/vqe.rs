
use itertools_num::linspace;
use bra_ket::*;


/// makes the ansatz circuit to prepare the quantum state
fn make_ansatz(theta: &Real) -> Program {
    let mut ansatz = Program::new();
    ansatz.reinitialise_all();
    ansatz.x(0);

    ansatz.rx(0, 3. * PI / 2.);
    ansatz.ry(1, PI / 2.);

    ansatz.cnot(1, 0);
    ansatz.rz(0, theta.to_owned());
    ansatz.cnot(1, 0);

    ansatz.rx(0, PI / 2.);
    ansatz.ry(1,  3. * PI / 2.);
    ansatz
}

/// calculates the energy of the ansatz state for a given theta in the ansatz circuit.
fn evaluate_energy(theta: &Real) -> Real {
    let ansatz = make_ansatz(theta);
    let mut state = StateVector::new(2);
    let mut energy = 0.;

    energy += 0.7055696146; // adding the nuclear repulsion
    energy -= 0.4804; // adding energy from the I0_I1 component of the hamiltonian

    energy += { // adding the energy from the Z0 components of the hamiltonian
        let coefficient = 0.3435; // the weighting for the pauli term in the hamiltonian
        // running the ansatz circuit
        ansatz.run(&mut state);
        // measuring the expectation value of the 0th qubit in the Z basis
        let expectation = state.get_expectation(&0);
        coefficient * expectation
    };

    energy += { // adding the energy from the Z1 components of the hamiltonian
        let coefficient = -0.4347; // the weighting for the pauli term in the hamiltonian
        ansatz.run(&mut state); // running the ansatz circuit and then the measurement
        // measuring the expectation value of the 1st qubit in the Z basis
        let expectation = state.get_expectation(&1);
        coefficient * expectation
    };

    energy += { // adding the energy from the Z0_Z1 components of the hamiltonian
        // creating a measurement program which maps the eigenstates of Z0_Z1 to Z eigenstates
        // of the 0th qubit.
        let mut measure_z0_z1 = Program::new();
        measure_z0_z1.cnot(1, 0);

        let coefficient = 0.5716; // the weighting for the pauli term in the hamiltonian
        // running the ansatz circuit and then the measurement program
        (ansatz.to_owned() + measure_z0_z1).run(&mut state);
        // measuring the expectation value of the 0th qubit in the Z basis
        let expectation = state.get_expectation(&0);
        coefficient * expectation
    };

    energy += { // adding the energy from the X0_X1 components of the hamiltonian
        // creating a measurement program which maps the eigenstates of X0_X1 to Z eigenstates
        // of the 0th qubit.
        let mut measure_x0_x1 = Program::new();
        measure_x0_x1.h(0);
        measure_x0_x1.h(1);
        measure_x0_x1.cnot(1, 0);

        let coefficient = 0.0910; // the weighting for the pauli term in the hamiltonian
        // running the ansatz circuit and then the measurement program
        (ansatz.to_owned() + measure_x0_x1).run(&mut state);
        // measuring the expectation value of the 0th qubit in the Z basis
        let expectation = state.get_expectation(&0);
        coefficient * expectation
    };


    energy += { // adding the energy from the Y0_Y1 components of the hamiltonian
        // creating a measurement program which maps the eigenstates of Y0_Y1 to Z eigenstates
        // of the 0th qubit.
        let mut measure_y0_y1 = Program::new();
        measure_y0_y1.h(0);
        measure_y0_y1.rz(0, - PI / 2.);
        measure_y0_y1.h(1);
        measure_y0_y1.rz(1, - PI / 2.);
        measure_y0_y1.cnot(1, 0);

        let coefficient = 0.0910; // the weighting for the pauli term in the hamiltonian
        // running the ansatz circuit and then the measurement program
        (ansatz.to_owned() + measure_y0_y1).run(&mut state);
        // measuring the expectation value of the 0th qubit in the Z basis
        let expectation = state.get_expectation(&0);
        coefficient * expectation
    };
    energy
}

fn main() {
    let n = 100;
    let theta_s = RVector::from_iterator(n, linspace::<Real>(- PI, PI, n));
    let energies =  RVector::from_iterator(n,theta_s
        .iter()
        .map(|theta| evaluate_energy(theta))
    );


    let (arg_min, energy_min) = energies.argmin();
    let theta_min = theta_s[arg_min];

    assert!(-1.15 < energy_min && energy_min < -1.14, "wrong energy");
    println!("Minimum theta: {} (radians)", theta_min as f32);
    println!("Minimum energy: {} (hartrees)", energy_min as f32)
}