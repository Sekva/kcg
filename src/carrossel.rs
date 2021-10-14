use std::collections::VecDeque;

use crate::instancias::{Multas, Pesos, Valores};

fn diferentes(a: &[usize], b: &VecDeque<usize>) -> bool {
    if a.len() != b.len() {
        return true;
    }

    for ai in a {
        if !b.contains(&ai) {
            return true;
        }
    }

    return false;
}

fn guloso_add_unico(
    pesos: &Pesos,
    valores: &Valores,
    multas: &Multas,
    b_res: f64,
    x: &[usize],
    s: &VecDeque<usize>,
) -> Result<usize, ()> {
    let mut x_iter = Vec::new();

    for &i in x {
        if pesos[i] <= b_res && !s.contains(&i) {
            x_iter.push(i);
        }
    }

    if x_iter.is_empty() {
        return Err(());
    }

    let mut razoes: Vec<(usize, f64)> = Vec::new();
    for i in x_iter {
        let mut pp_i = valores[i];
        for (i_m, j_m, d) in multas {
            if i == *i_m && s.contains(&j_m) || i == *j_m && s.contains(&i_m) {
                pp_i = pp_i - d;
            }
        }

        razoes.push((i, pp_i / pesos[i]));
    }

    let razao_max: (usize, f64) = *razoes
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    if razao_max.1 < 0.0 {
        return Err(());
    }

    let i_opt = razao_max.0;
    return Ok(i_opt);
}

fn guloso_add_completar(
    pesos: &Pesos,
    valores: &Valores,
    multas: &Multas,
    mut b_res: f64,
    x: &[usize],
    s: &mut VecDeque<usize>,
) {
    loop {
        if !diferentes(x, &s) {
            break;
        }

        if let Ok(i) = guloso_add_unico(pesos, valores, multas, b_res, x, &s) {
            s.push_back(i);
            b_res = b_res - pesos[i];
        } else {
            break;
        }
    }
}

pub fn carrossel_guloso(
    pesos: &Pesos,
    valores: &Valores,
    multas: &Multas,
    b: f64,
) -> VecDeque<usize> {
    let alpha = 1;
    let beta = 0.4;

    let n_itens = valores.len();
    let x: Vec<usize> = (0..n_itens).collect();

    // Solução inicial
    let mut s = VecDeque::new();
    guloso_add_completar(pesos, valores, multas, b, &x, &mut s);
    // println!("Solução incial:    {:?}", s);

    // Remove os ultimos itens adicionados (dá pra melhorar)
    let tamanho_solucao = s.len();
    let max_itens = s.len() - (beta * s.len() as f64) as usize;
    for _ in 0..(s.len() - max_itens) {
        s.pop_back();
    }

    // Calcula o quanto ocupou de peso
    let mut b_res: f64 = b - s.iter().map(|&idx| pesos[idx]).sum::<f64>();

    // println!("Solução - alpha:   {:?}\n", s);

    let iteracoes = alpha * tamanho_solucao;
    for _ in 0..iteracoes {
        let option_mais_velho = s.pop_front();
        if option_mais_velho.is_none() {
            panic!("fila vazia...");
        }
        let mais_velho = option_mais_velho.unwrap();
        b_res = b_res + pesos[mais_velho];

        // println!("Solução parcial 1: {:?}", s);

        if let Ok(i) = guloso_add_unico(pesos, valores, multas, b_res, &x, &s) {
            s.push_back(i);
            b_res = b_res - pesos[i];
        } else {
            panic!("Nenhum idx unico");
        }

        // println!("Solução parcial 2: {:?}\n", s);
    }

    guloso_add_completar(pesos, valores, multas, b_res, &x, &mut s);
    // println!("Solução final:     {:?}", s);
    s
}

pub fn guloso_total(pesos: &Pesos, valores: &Valores, multas: &Multas, b: f64) -> VecDeque<usize> {
    let n_itens = valores.len();
    let x: Vec<usize> = (0..n_itens).collect();

    let mut s = VecDeque::new();
    guloso_add_completar(pesos, valores, multas, b, &x, &mut s);

    s
}
