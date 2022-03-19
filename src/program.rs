use crate::density_matrix::*;
use crate::gates::*;
use crate::helpers::*;
use crate::types::*;

/// A struct to contain the quantum program. The density_matrix describes the quantum state
/// and the vector of gates describe the operations to be performed on the density matrix.
#[derive(Debug)]
pub struct Program {
    pub gates: Vec<Gate>,
}

impl Program {
    pub fn new() -> Program {
        return Program { gates: vec![] };
    }

    pub fn run(&mut self, state: &mut State) {
        for gate in &self.gates {
            implement_gate(state, gate)
        }
    }

    pub fn which_qubits(&self) -> Vec<&Qubit> {
        let mut qubits: Vec<&Qubit> = vec![];
        for gate in &self.gates {
            qubits.append(which_qubits(gate).as_mut());
        }
        qubits
    }

    pub fn draw(&mut self) {
        draw_circuit(&self)
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn add_gates(&mut self, mut gates: Vec<Gate>) {
        self.gates.append(&mut gates)
    }

    pub fn measure(&mut self, qubit: Qubit) {
        self.add_gate(Gate::Measure(qubit))
    }

    pub fn x(&mut self, qubit: Qubit) {
        self.add_gate(Gate::X(qubit))
    }

    pub fn rx(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::RX(qubit, angle))
    }

    pub fn y(&mut self, qubit: Qubit) {
        self.add_gate(Gate::Y(qubit))
    }

    pub fn ry(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::RY(qubit, angle))
    }

    pub fn z(&mut self, qubit: Qubit) {
        self.add_gate(Gate::Z(qubit))
    }

    pub fn rz(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::RZ(qubit, angle))
    }

    pub fn r(&mut self, qubit: Qubit, phi: Angle, theta: Angle, omega: Angle) {
        self.add_gate(Gate::R(qubit, phi, theta, omega))
    }

    pub fn h(&mut self, qubit: Qubit) {
        self.add_gate(Gate::H(qubit))
    }

    pub fn cnot(&mut self, control: Qubit, target: Qubit) {
        self.add_gate(Gate::CNOT(control, target))
    }

    pub fn siswap(&mut self, control: Qubit, target: Qubit) {
        self.add_gate(Gate::SISWAP(control, target))
    }

    pub fn iswap(&mut self, control: Qubit, target: Qubit) {
        self.add_gate(Gate::ISWAP(control, target))
    }

    pub fn s(&mut self, qubit: Qubit) {
        self.add_gate(Gate::S(qubit))
    }
}
