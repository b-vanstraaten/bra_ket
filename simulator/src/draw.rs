use crate::gates::Operation;
use crate::Program;
use itertools::zip;

pub fn draw_circuit(program: &Program) {
    let qubits = program.which_qubits();
    let mut circuit: Vec<String> = vec![String::from("Qubit "); qubits.len()];

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
fn plot_gate(gate: &Operation, qubit_index: &usize) -> String {
    match gate {
        Operation::Measure(qubit) => {
            let m: String = format!("{:-<7}", "M");
            return_string(qubit_index, *qubit, m)
        }

        Operation::X(qubit) => {
            let m: String = format!("{:-<7}", "X");
            return_string(qubit_index, *qubit, m)
        }
        Operation::Y(qubit) => {
            let m: String = format!("{:-<7}", "Y");
            return_string(qubit_index, *qubit, m)
        }
        Operation::Z(qubit) => {
            let m: String = format!("{:-<7}", "Z");
            return_string(qubit_index, *qubit, m)
        }

        Operation::RX(qubit, angle) => {
            let m: String = format!("RX({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operation::RY(qubit, angle) => {
            let m: String = format!("RY({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operation::RZ(qubit, angle) => {
            let m: String = format!("RZ({:.*})", 1, angle);
            return_string(qubit_index, *qubit, m)
        }
        Operation::H(qubit) => {
            let m: String = format!("{:-<7}", "H");
            return_string(qubit_index, *qubit, m)
        }
        Operation::R(qubit, omega, theta, phi) => {
            let m: String = format!("R.{:.*},{:.*},{:.*}", 0, omega, 0, theta, 0, phi);
            return_string(qubit_index, *qubit, m)
        }
        Operation::ArbitarySingle(qubit, u) => {
            let m: String = format!("{:-<7}", "ArbU");
            return_string(qubit_index, *qubit, m)
        }
        Operation::CNOT(control, target) => {
            let m: String = "CNOT".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        Operation::SISWAP(control, target) => {
            let m: String = "SSwap".to_owned();
            return_two_gate_string(qubit_index, *control, *target, m)
        }
        Operation::ArbitaryTwo(control, target, u) => {
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
    let default: String = format!("{:-<7}", "-");
    return default;
}

fn return_two_gate_string(
    indexed_qubit: &usize,
    control: usize,
    target: usize,
    message: String,
) -> String {
    if *indexed_qubit == control {
        let m: String = format!("{:->5}_C", message);
        return m;
    } else if *indexed_qubit == target {
        let m: String = format!("{:->5}_T", message);
        return m;
    }
    let default: String = format!("{:-<7}", "-");
    return default;
}

pub const AARDVARK: &str = r#"

****************************************************************
---------------------Quantum--Aardvark--------------------------
     (`.  : /               __..----..__
      `.`.| |:          _,-':::''' '  `:`-._
        `.:/||       _,':::::'         `::::`-.
          \\`|    _,':::::::'     `:.     `':::`.
           ;` `-''  `::::::.                  `::\
        ,-'      .::'  `:::::.         `::..    `:\
      ,' /_) -.            `::.           `:.     |
    ,'.:     `    `:.        `:.     .::.          \
__,-'   ___,..-''-.  `:.        `.   /::::.         |
|):'_,--'           `.    `::..       |::::::.      ::\
`-'                 |`--.:_::::|_____|::::::::.__  ::|
                   |   _/|::::|      |::::::|::/|  :|
                   /:./  |:::/        |__:::):/  |  :\
                 ,'::'  /:::|        ,'::::/_/    `. ``-..--'-_
                ''''   (//|/|      ,';':,-'         `-.___---''
---------------------------------------------------------------
****************************************************************

"#;
