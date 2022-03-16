use crate::{Program};

pub fn draw_circuit(
    program: &Program
) {
    for n in 0..program.state.number_of_qubits {
        for gate in program.gates {
            println!("{} : ----{}", n, );
        }
    }
}

pub fn plot_gate( gate: &Gate) {
    debug!("{:?}", gate);
    match gate {
        Gate::Measure(qubit) => measure(state, qubit),

        Gate::X(qubit) => x(state, qubit),
        Gate::Y(qubit) => y(state, qubit),
        Gate::Z(qubit) => z(state, qubit),

        Gate::RX(qubit, angle) => rx(state, qubit, angle),
        Gate::RY(qubit, angle) => ry(state, qubit, angle),
        Gate::RZ(qubit, angle) => rz(state, qubit, angle),
        Gate::H(qubit) => h(state, qubit),
        Gate::CNOT(control, target) => cnot(state, control, target),
    }
}