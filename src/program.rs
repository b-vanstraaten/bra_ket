use log::{debug, info, warn};
use nalgebra::ComplexField;
use std::mem::size_of_val;

use crate::gates::*;
use crate::types::*;

#[derive(Debug)]
pub struct State {
    pub number_of_qubits: Qubit,
    pub density_matrix: DensityMatrix,
}

impl State {
    pub fn new(number_of_qubits: Qubit) -> State {
        let mut density_matrix = create_density_matrix(number_of_qubits);
        State {
            number_of_qubits,
            density_matrix,
        }
    }
}

pub fn assert_approximately_equal(required_state: State, state: State) {
    if !approx_eq(&required_state, &state) {
        debug!("final_state: \n{}", state.density_matrix);
        assert_eq!(required_state.density_matrix, state.density_matrix);
    }
}

fn approx_eq(state: &State, other_state: &State) -> bool {
    let mut result = false;
    if state.number_of_qubits == other_state.number_of_qubits {
        if state.density_matrix.shape() == other_state.density_matrix.shape() {
            let difference = &state.density_matrix - &other_state.density_matrix;
            if difference.iter().all(|d| d.abs() < COMPARISON_PRECISION) {
                result = true;
            }
        }
    }
    result
}

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

fn create_density_matrix(number_of_qubits: usize) -> DensityMatrix {
    // calculating the hilbert dim
    let hilbert_dim = (1 << number_of_qubits) as usize;
    // printing the size of the density matrix to be created
    {
        let density_matrix_footprint = hilbert_dim.pow(2) * size_of_val(&C::new(0., 0.));
        let bytes_to_gigabyte: usize = 2 << 33;
        debug!(
            "Allocating density matrix of size: {:.4} Gb",
            (density_matrix_footprint as f32) / (bytes_to_gigabyte as f32)
        );
    }

    // creating the density matrix
    let mut rho = DensityMatrix::from_element(hilbert_dim, hilbert_dim, C::new(0., 0.));
    // setting the (0, 0) element to 1 to represent initialisation in the |000...> state
    rho[(0, 0)] = C::new(1., 0.);
    return rho;
}
