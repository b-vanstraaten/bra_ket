use crate::draw::*;
use crate::operations::*;
use crate::state_traits::{StateTraits};
use crate::types::*;
use std::ops::Add;

/// A quantum program, encoding the sequence of qubit operations to be performed.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub gates: Vec<Operations>,
}

/// Add two programs, so that one runs after the other.
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

    pub fn run<T: StateTraits>(
        &self,
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

    pub fn add_gate(&mut self, gate: Operations) {
        self.gates.push(gate);
    }

    pub fn add_gates(&mut self, mut gates: Vec<Operations>) {
        self.gates.append(&mut gates)
    }

    pub fn measure(&mut self, qubit: usize) {
        self.add_gate(Operations::Measure(qubit))
    }

    pub fn measure_all(&mut self) {
        self.add_gate(Operations::MeasureAll)
    }

    pub fn reinitialise_all(&mut self) {
        self.add_gate(Operations::ReinitialiseAll)
    }

    pub fn x(&mut self, qubit: usize) {
        self.add_gate(Operations::X(qubit))
    }

    pub fn rx(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operations::RX(qubit, angle))
    }

    pub fn y(&mut self, qubit: usize) {
        self.add_gate(Operations::Y(qubit))
    }

    pub fn ry(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operations::RY(qubit, angle))
    }

    pub fn z(&mut self, qubit: usize) {
        self.add_gate(Operations::Z(qubit))
    }

    pub fn rz(&mut self, qubit: usize, angle: Angle) {
        self.add_gate(Operations::RZ(qubit, angle))
    }

    pub fn r(&mut self, qubit: usize, phi: Angle, theta: Angle, omega: Angle) {
        self.add_gate(Operations::R(qubit, phi, theta, omega))
    }

    pub fn h(&mut self, qubit: usize) {
        self.add_gate(Operations::H(qubit))
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        self.add_gate(Operations::CZ(control, target))
    }

    pub fn crz(&mut self, control: usize, target: usize, _angle: Angle) {
        self.add_gate(Operations::CZ(control, target))
    }

    pub fn cnot(&mut self, control: usize, target: usize) {
        self.add_gate(Operations::CNOT(control, target))
    }

    pub fn siswap(&mut self, control: usize, target: usize) {
        self.add_gate(Operations::SISWAP(control, target))
    }

    pub fn iswap(&mut self, control: usize, target: usize) {
        self.add_gate(Operations::ISWAP(control, target))
    }

    pub fn swap(&mut self, control: usize, target: usize) {
        self.add_gate(Operations::SWAP(control, target))
    }

    pub fn s(&mut self, qubit: usize) {
        self.add_gate(Operations::S(qubit))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operations, Program};

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
        program.add_gates(vec![Operations::H(0), Operations::CZ(0, 1)]);

        let mut required_program = Program::new();
        required_program.h(0);
        required_program.cz(0, 1);
        assert_eq!(program, required_program)
    }

}
