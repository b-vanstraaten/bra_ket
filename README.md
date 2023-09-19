# Rust Quantum Simulator
![GitHub Workflow Status](https://github.com/b-vanstraaten/bra_ket/workflows/Rust/badge.svg)

![Rust logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

This is a Rust-based quantum simulator that provides an identical interface for state vector and density matrix simulations. It is fully parallelized using the Rayon library. This project was developed as a learning exercise in Rust programming and qubit simulation.

## Features

- Simulate quantum circuits using state vector or density matrix representations.
- Support for common quantum gates and operations.
- Parallelized simulation for improved performance on multi-core processors.
- User-friendly interface for defining and running quantum circuits.

## Installation

To use this quantum simulator, you need to have Rust and Cargo (Rust's package manager) installed on your system. If you don't have Rust installed, you can download it from the official website: [Rust Installation](https://www.rust-lang.org/learn/get-started).
then add it as dependancy in your `Cargo.toml`:

```toml
[dependencies]
bra_ket = "0.1.3"
```
Make sure to replace `"0.1.2"` with the actual version number from the release you want to use.

Then, in your Rust code:
```rust
use bra_ket::*;
fn main() {

    // creating the quantum program 
    let mut program = Program::new();
    program.h(0);
    program.cnot(0, 1);
    program.cnot(0, 2);

    // drawing the program 
    program.draw();

    // creating a three qubit state vector / density matrix to hold the quantum state
    let mut state_vector = StateVector::new(3);
    let mut density_matrix = DensityMatrix::new(3);

    // running the program to evolve the state vector from its 
    //initial state of |000> to (1 / SQRT_2) *(|000> + |111>)
    program.run(&mut state_vector);
    program.run(&mut density_matrix);

    // printing the final state vector / density matrix
    println!("state vector \n{}", state_vector);
    println!("density matrix \n{}", density_matrix);
}
```
This code produces the GHZ state (1 / SQRT_2) *(|000> + |111>) in both state vector and density matrix representations.

## Documentation

For detailed documentation and examples, please refer to the [official documentation](https://docs.rs/bra_ket/0.1.3/bra_ket/).

# Examples

This section provides a brief overview of the example files included in the `examples` folder of this quantum simulator project. These examples demonstrate how to use the quantum simulator to simulate various quantum algorithms and circuits. You can find the complete source code for each example in the `examples` folder.

## 1. `ghz_state.rs`

- **Description:** This example demonstrates the creation of a Greenberger-Horne-Zeilinger (GHZ) state using a quantum circuit. The GHZ state is an entangled quantum state that plays a crucial role in quantum information theory.

- **Usage:** To run this example, execute the following command from the project root directory:

  ```bash
  cargo run --example ghz_state
  ```


## 2. `qft.rs`

- **Description:** The Quantum Fourier Transform (QFT) is a quantum algorithm that efficiently computes the discrete Fourier transform of a quantum state. This example demonstrates the implementation of the QFT using the quantum simulator.

- **Usage:** To run this example, execute the following command:

  ```bash
  cargo run --example qft
  ```

## 3. `shors.rs`

- **Description:** Shor's algorithm is a quantum algorithm that can efficiently factor large numbers. This example showcases the implementation of Shor's algorithm using the quantum simulator.

- **Usage:** To run this example, execute the following command:

  ```bash
  cargo run --example shors
  ```

## 4. `vge.rs`

- **Description:** The Variational Quantum Eigensolver (VQE) is a quantum algorithm used for finding the ground state energy of a quantum system. This example demonstrates how to use the quantum simulator to perform a VQE calculation for H2, it computes the ground state energy and plots the energy landscape.

- **Usage:** To run this example, execute the following command:

  ```bash
  cargo run --example vqe
  ```

These example files provide a starting point for experimenting with quantum algorithms and circuits using the Rust Quantum Simulator. You can modify and extend them to explore different quantum computing concepts and applications.



## Contributing

This project was developed as a learning exercise, but contributions are welcome! If you want to contribute, please:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure they pass the existing tests.
4. Add new tests if necessary.
5. Create a pull request to submit your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/b-vanstraaten/bra_ket/blob/main/License.txt) file for details.

## Acknowledgments

- Thanks to the Rust community for creating an excellent language and ecosystem.
- The inspiration for this project comes from the field of quantum computing and quantum circuit simulation.

Happy Quantum Computing! 🌌🔬🧬
