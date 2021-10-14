#[macro_use]
extern crate prettytable;

pub mod carrossel;
pub mod cplex;
pub mod ils_vnd;
pub mod ils_vnd_mwis;
pub mod instancias;
pub mod utils;

use carrossel::{carrossel_guloso, guloso_total};
use cplex::resolver_por_cplex;
use ils_vnd::ils_vnd;
use instancias::{Multas, Pesos, Valores};
use utils::dq_vec;

pub struct Problema<'a> {
    pesos: &'a Pesos,
    valores: &'a Valores,
    multas: &'a Multas,
    b: f64,
}

pub fn calcular_peso_total(s: &[usize], problema: &Problema) -> f64 {
    let mut peso_total = 0.0;

    for &item in s {
        peso_total += problema.pesos[item];
    }

    peso_total
}

pub fn calcular_valor_total(s: &[usize], problema: &Problema) -> f64 {
    let mut valor_total = 0.0;

    for &item in s {
        valor_total += problema.valores[item];
    }

    for (i, j, d) in problema.multas.clone() {
        if s.contains(&i) && s.contains(&j) {
            valor_total -= d;
        }
    }

    valor_total
}

pub fn calcular_peso_valor_total(s: &[usize], problema: &Problema) -> (f64, f64, usize) {
    let mut peso_total = 0.0;
    let mut valor_total = 0.0;
    let mut n_penalidades = 0;

    for &item in s {
        peso_total += problema.pesos[item];
        valor_total += problema.valores[item];
    }

    for (i, j, d) in problema.multas.clone() {
        if s.contains(&i) && s.contains(&j) {
            n_penalidades += 1;
            valor_total -= d;
        }
    }

    (peso_total, valor_total, n_penalidades)
}

#[derive(Debug)]
pub enum Executor {
    CPlex,
    Guloso,
    Carrossel,
    ILsVNd,
}

impl Executor {
    fn executar(
        &self,
        b: f64,
        multas: &Multas,
        valores: &Valores,
        pesos: &Pesos,
    ) -> (f64, Vec<usize>) {
        match self {
            Executor::CPlex => resolver_por_cplex(b, multas, valores, pesos),
            Executor::Guloso => {
                let ti = std::time::Instant::now();
                let s = guloso_total(&pesos, &valores, &multas, b);
                let dr = ti.elapsed();
                let mut sg = dq_vec(&s);
                sg.sort();
                (dr.as_secs_f64(), sg)
            }
            Executor::Carrossel => {
                let ti = std::time::Instant::now();
                let s = carrossel_guloso(&pesos, &valores, &multas, b);
                let dr = ti.elapsed();
                let mut sc = dq_vec(&s);
                sc.sort();
                (dr.as_secs_f64(), sc)
            }
            Executor::ILsVNd => {
                let ti = std::time::Instant::now();
                let mut s = ils_vnd(&pesos, &valores, &multas, b);
                let dr = ti.elapsed();
                s.sort();
                (dr.as_secs_f64(), s)
            }
        }
    }
}

#[cfg(test)]
mod testes;
