use crate::operations::Operations;
use crate::Program;
use itertools::zip;

// Generate qasm file and input into python circuit generator https://www.media.mit.edu/quanta/qasm2circ/

pub fn draw_circuit(program: &Program) {
    let qubits = program.which_qubits();
    let mut circuit: Vec<String> = vec![String::from("qubit "); qubits.len()];

    for (n, string) in zip(qubits.to_owned(), circuit.iter_mut()) {
        string.push_str(format!("{} : ---", n).as_str())
    }

    for gate in program.gates.iter() {
        for (n, string) in zip(qubits.to_owned(), circuit.iter_mut()) {
            let m: String = plot_gate(gate, n) + "---";
            string.push_str(&m);
        }
    }

    for string in circuit.iter_mut() {
        string.push_str("||")
    }
    println!("_____________________________");
    println!("****** Quantum Circuit ******");
    println!("");
    for line in circuit.iter_mut() {
        println!("{:#?}", line);
    }
    println!("");
    println!("*****************************");
    println!("_____________________________");
}
//
fn plot_gate(gate: &Operations, qubit_index: &usize) -> String {
    match gate {
        Operations::Measure(qubit) => {
            let m: String = format!("{:-<3}", "M");
            return_string(qubit_index, *qubit, m)
        }

        Operations::X(qubit) => {
            let m: String = format!("{:-<1}", "X");
            return_string(qubit_index, *qubit, m)
        }
        Operations::Y(qubit) => {
            let m: String = format!("{:-<1}", "Y");
            return_string(qubit_index, *qubit, m)
        }
        Operations::Z(qubit) => {
            let m: String = format!("{:-<1}", "Z");
            return_string(qubit_index, *qubit, m)
        }

        Operations::RX(qubit, angle) => {
            let m: String = format!("RX({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operations::RY(qubit, angle) => {
            let m: String = format!("RY({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operations::RZ(qubit, angle) => {
            let m: String = format!("RZ({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operations::H(qubit) => {
            let m: String = format!("{:-<1}", "H");
            return_string(qubit_index, *qubit, m)
        }
        Operations::R(qubit, omega, theta, phi) => {
            let m: String = format!("R.{:.*},{:.*},{:.*}", 0, omega, 0, theta, 0, phi);
            return_string(qubit_index, *qubit, m)
        }
        Operations::ArbitrarySingle(qubit, _u) => {
            let m: String = format!("{:-<1}", "ArbU");
            return_string(qubit_index, *qubit, m)
        }
        Operations::CZ(control, target) => {
            let m: String = "CZ".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        Operations::CNOT(control, target) => {
            let m: String = "CNOT".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        Operations::SISWAP(control, target) => {
            let m: String = "SSwap".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        Operations::ArbitaryTwo(control, target, _u) => {
            let m: String = "ArbU".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        _ => {
            let default: String = "Other__".to_owned();
            default
        }
    }
}

fn return_string(indexed_qubit: &usize, gate_qubit: usize, message: String) -> String {
    if *indexed_qubit == gate_qubit {
        return message;
    }
    let default: String = format!("{:-<3}", "-");
    return default;
}

fn return_two_gate_string(
    indexed_qubit: &usize,
    control: usize,
    target: usize,
    message: String,
) -> String {
    if *indexed_qubit == control {
        let m: String = format!("{:->1}_C", message);
        return m;
    } else if *indexed_qubit == target {
        let m: String = format!("{:->1}_T", message);
        return m;
    }
    let default: String = format!("{:-<1}", "-");
    return default;
}

