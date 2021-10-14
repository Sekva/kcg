use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use rand::prelude::IteratorRandom;

pub type Pesos = Vec<f64>;
pub type Valores = Vec<f64>;
pub type Multas = Vec<(usize, usize, f64)>;

pub fn gerar_instancias(n_itens: u64) -> Result<(Pesos, Valores, Multas, f64), ()> {
    let mut pesos = Vec::new();
    let mut valores = Vec::new();
    let mut multas = Vec::new();

    let b = (n_itens * 3) as f64;
    let n_multas = n_itens * 6;

    let pesos_possiveis = 3..=20;
    let valores_possiveis = 5..=25;
    let multas_possiveis = 2..=15;

    for _ in 0..n_itens {
        let pi = pesos_possiveis
            .clone()
            .choose(&mut rand::thread_rng())
            .unwrap() as f64;
        let wi = valores_possiveis
            .clone()
            .choose(&mut rand::thread_rng())
            .unwrap() as f64;
        pesos.push(wi);
        valores.push(pi);
    }

    while multas.len() != n_multas as usize {
        let u: usize = (0..n_itens).choose(&mut rand::thread_rng()).unwrap() as usize;
        let v: usize = (0..n_itens).choose(&mut rand::thread_rng()).unwrap() as usize;

        if u == v {
            continue;
        }

        let mut achado = false;
        for (u2, v2, _) in &multas {
            achado |= u2 == &u && v2 == &v || u2 == &v && v2 == &u;
        }
        if achado {
            continue;
        }

        let d = multas_possiveis
            .clone()
            .choose(&mut rand::thread_rng())
            .unwrap() as f64;

        if u < v {
            multas.push((u, v, d));
        } else {
            multas.push((v, u, d));
        }
    }

    Ok((pesos, valores, multas, b))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn ler_instancias(arq: &str, densidade: f64) -> Result<(Pesos, Valores, Multas, f64), ()> {
    let mut pesos = Vec::new();
    let mut valores = Vec::new();
    let mut multas = Vec::new();

    if let Ok(lines) = read_lines(arq) {
        for line in lines {
            if let Ok(l) = line {
                let ss: Vec<&str> = l.split(" ").collect();

                if ss[0] == "p" {
                    let n_itens = ss[2].parse().unwrap();
                    pesos = vec![0.0; n_itens];
                    valores = vec![0.0; n_itens];
                }

                if ss[0] == "n" {
                    let idx: usize = ss[1].parse::<usize>().unwrap() - 1;
                    let valor: usize = ss[2].parse().unwrap();
                    pesos[idx] = 2.0 * valor as f64;
                    valores[idx] = valor as f64;
                }

                if ss[0] == "not_e" {
                    let i: usize = ss[1].parse::<usize>().unwrap() - 1;
                    let j: usize = ss[2].parse::<usize>().unwrap() - 1;
                    let d: f64 = (1.0 / 25.0) * (valores[i] + valores[j]) as f64;
                    multas.push((i, j, d));
                }
            }
        }
    }

    let b = pesos.iter().sum::<f64>() / 4.0;

    let n_multas = (multas.len() as f64 * densidade) as usize;
    let mut cont = 10;
    while multas.len() != n_multas {
        multas.remove(cont);
        cont = (cont + 10) % multas.len();
    }

    Ok((pesos, valores, multas, b))
}
