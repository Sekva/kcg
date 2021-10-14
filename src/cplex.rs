use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use crate::instancias::{Multas, Pesos, Valores};

pub fn resolver_por_cplex(
    b: f64,
    multas: &Multas,
    valores: &Valores,
    pesos: &Pesos,
) -> (f64, Vec<usize>) {
    let mut str_in: String = "".into();

    str_in.push_str(&format!("{} ", valores.len()));
    str_in.push_str(&format!("{} ", b));
    str_in.push_str(&format!("{}\n", multas.len()));

    for valor in valores {
        str_in.push_str(&format!("{}\n", valor));
    }

    for peso in pesos {
        str_in.push_str(&format!("{}\n", peso));
    }

    for (i, j, d) in multas {
        str_in.push_str(&format!("{} {} {}\n", i, j, d));
    }

    let mut file = File::create("tmp_inst").unwrap();
    file.write_all(str_in.as_bytes()).unwrap();
    let file = File::open("tmp_inst").unwrap();

    let str_r = Command::new("./lib_cplex/modelo.run")
        .stdin(file)
        .output()
        .expect("Ih");

    let out: Vec<String> = std::str::from_utf8(&str_r.stdout)
        .unwrap()
        .lines()
        .map(|l| l.trim().to_string())
        .collect();

    let tempo_execucao: f64 = out[0].parse::<f64>().unwrap() / 1000.0;

    let mut itens: Vec<usize> = out[1].split(" ").map(|ss| ss.parse().unwrap()).collect();
    itens.sort();

    (tempo_execucao, itens)
}
