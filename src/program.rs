use std::mem::size_of_val;

use log::{debug, info, warn};
use nalgebra::ComplexField;

use crate::gates::*;
use crate::state::*;
use crate::types::*;

/// A struct to contain the quantum program. The density_matrix describes the quantum state
/// and the vector of gates describe the operations to be performed on the density matrix.
#[derive(Debug)]
pub struct Program {
    pub state: State,
    pub gates: Vec<Gate>,
}

#[allow(dead_code)]
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

    /// Adds a gate to the quantum program. It should be noted that the term gate is a slight
    /// simplification of notation. As the "gate" can be a measurement or even conditional logic.
    ///
    /// # Arguments
    ///
    /// * `gate`: gate to be added to the program.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn run(&mut self) {
        for gate in &self.gates {
            implement_gate(&mut self.state, gate)
        }
    }

    pub fn measure(&mut self, qubit: Qubit) {
        self.add_gate(Gate::Measure(qubit))
    }

    pub fn x(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::X(qubit, angle))
    }

    pub fn y(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::Y(qubit, angle))
    }

    pub fn z(&mut self, qubit: Qubit, angle: Angle) {
        self.add_gate(Gate::Z(qubit, angle))
    }
}
