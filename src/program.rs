use crate::gates::*;
use crate::helpers::*;
use crate::density_matrix::*;
use crate::types::*;

/// A struct to contain the quantum program. The density_matrix describes the quantum state
/// and the vector of gates describe the operations to be performed on the density matrix.
#[derive(Debug)]
pub struct Program {
    pub state: State,
    pub gates: Vec<Gate>,
}

impl Program {
    /// Creates a new program. It initialises the density matrix based on the number_of_qubits. The
    /// density matrix is initialised in the |00..><00..| state.
    ///
    /// # Arguments
    ///
    /// * `number_of_qubits`: the number of qubits involved in the experiment. The resulting density
    /// will be 2 ^ number_of_qubits by 2 ^ number_of_qubits.
    ///
    /// returns: a new quantum program
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(number_of_qubits: Qubit) -> Program {
        let state = State::new(number_of_qubits);
        let gates: Vec<Gate> = vec![];
        return Program { state, gates };
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.gates = vec![];
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn run(&mut self) {
        for gate in &self.gates {
            implement_gate(&mut self.state, gate)
        }
    }

    pub fn draw(&mut self) {
        draw_circuit(&self)
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
