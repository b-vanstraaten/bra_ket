use crate::draw::*;
use crate::qubit_operations::*;
use crate::state_traits::{QuantumStateTraits};
use crate::types::*;
use std::ops::Add;

/// A struct to contain the quantum program. The density_matrix describes the quantum state
/// and the vector of gates describe the operations to be performed on the density matrix.
#[derive(Debug, PartialEq)]
pub struct Program {
    pub gates: Vec<Operation>,
}

impl Add for Program {
    type Output = Program;
    fn add(self, other: Self) -> Self {
        let mut program = Program::new();
        let gates = [&self.gates[..], &other.gates[..]].concat();
        program.add_gates(gates);
        program
    }
}


impl Program {
    pub fn new() -> Program {
        return Program { gates: vec![] };
    }

    pub fn run<T: QuantumStateTraits>(
        &mut self,
        state: &mut T,
    ) {
        // logic to panic if the program requires more qubits than present in the state
        state.check_qubit_number(self.which_qubits());
        // iterate through the gates and implement them
        for gate in &self.gates {
            implement_gate(state, gate)
        }
    }

    pub fn which_qubits(&self) -> Vec<&usize> {
        let mut qubits: Vec<&usize> = vec![];
        for gate in self.gates.iter() {
            qubits.append(which_qubits(gate).as_mut());
        }
        qubits.sort(); // sorting
        qubits.dedup(); // removes consecutive equal elements according the the partial eq trait
        qubits
    }

    pub fn draw(&mut self) {
        draw_circuit(&self)
    }

    pub fn add_gate(&mut self, gate: Operation) {
        self.gates.push(gate);
    }

    pub fn add_gates(&mut self, mut gates: Vec<Operation>) {
        self.gates.append(&mut gates)
    }

    pub fn measure(&mut self, qubit: usize) {
        self.add_gate(Operation::Measure(qubit))
    }

    pub fn measure_all(&mut self) {
        self.add_gate(Operation::MeasureAll)
    }

    pub fn reset_all(&mut self) {
        self.add_gate(Operation::ResetAll)
    }

    pub fn x(&mut self, qubit: usize) {
        self.add_gate(Operation::X(qubit))
    }

    pub fn rx(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operation::RX(qubit, angle))
    }

    pub fn y(&mut self, qubit: usize) {
        self.add_gate(Operation::Y(qubit))
    }

    pub fn ry(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operation::RY(qubit, angle))
    }

    pub fn z(&mut self, qubit: usize) {
        self.add_gate(Operation::Z(qubit))
    }

    pub fn rz(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operation::RZ(qubit, angle))
    }

    pub fn r(&mut self, qubit: usize, phi: Angle, theta: Angle, omega: Angle) {
        self.add_gate(Operation::R(qubit, phi, theta, omega))
    }

    pub fn h(&mut self, qubit: usize) {
        self.add_gate(Operation::H(qubit))
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        self.add_gate(Operation::CZ(control, target))
    }

    pub fn crz(&mut self, control: usize, target: usize, angle: Angle) {
        self.add_gate(Operation::CZ(control, target))
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        self.add_gate(Operation::CNOT(control, target))
    }

    pub fn siswap(&mut self, control: usize, target: usize) {
        self.add_gate(Operation::SISWAP(control, target))
    }

    pub fn iswap(&mut self, control: usize, target: usize) {
        self.add_gate(Operation::ISWAP(control, target))
    }

    pub fn swap(&mut self, control: usize, target: usize) {
        self.add_gate(Operation::SWAP(control, target))
    }

    pub fn s(&mut self, qubit: usize) {
        self.add_gate(Operation::S(qubit))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operation, Program};
    use crate::qubit_operations::which_qubits;

    #[test]
    fn test_qubit_number() {
        let mut program = Program::new();
        program.x(5);
        program.y(2);
        program.z(4);
        program.cnot(0, 6);
        let qubits = program.which_qubits();
        assert_eq!(qubits, vec![&0, &2, &4, &5, &6])
    }

    #[test]
    fn test_add_gates() {
        let mut program = Program::new();
        program.add_gates(vec![Operation::H(0), Operation::CZ(0, 1)]);

        let mut required_program = Program::new();
        required_program.h(0);
        required_program.cz(0, 1);
        assert_eq!(program, required_program)
    }

}
