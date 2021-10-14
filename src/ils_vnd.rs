use rand::prelude::{IteratorRandom, SliceRandom};

use crate::{
    calcular_peso_total, calcular_valor_total,
    carrossel::guloso_total,
    instancias::{Multas, Pesos, Valores},
    utils::dq_vec,
    Problema,
};

pub fn initialize(problema: &Problema) -> Vec<usize> {
    let mut r: Vec<usize> = Vec::new();

    let mut x: Vec<usize> = (0..problema.valores.len()).collect();
    x.shuffle(&mut rand::thread_rng());

    let mut b_res = problema.b;
    while let Some(v) = x.pop() {
        if b_res >= problema.pesos[v] {
            b_res = b_res - problema.pesos[v];
            r.push(v);
        }
    }

    r
}

fn perturb(k: usize, s: &[usize], problema: &Problema) -> Vec<usize> {
    let x: Vec<usize> = (0..problema.valores.len())
        .filter(|e| !s.contains(&e))
        .choose_multiple(&mut rand::thread_rng(), k);
    let peso_necessario = calcular_peso_total(&x, problema);

    let mut s = Vec::from(s);
    let mut b_res = problema.b - calcular_peso_total(&s, problema);

    while let Some(removido) = s.pop() {
        b_res = b_res + problema.pesos[removido];

        if b_res >= peso_necessario {
            break;
        }
    }

    s.extend(x.iter());

    let mut b_res = problema.b - calcular_peso_total(&s, problema);
    while b_res < 0.0 {
        let removido = s.pop().unwrap();
        b_res = b_res + problema.pesos[removido];
    }

    s
}

pub fn add_free_vertices(s: &mut Vec<usize>, problema: &Problema) {
    let mut x: Vec<usize> = (0..problema.valores.len())
        .filter(|e| !s.contains(&e))
        .collect();
    let mut b_res = problema.b - calcular_peso_total(&s, problema);

    while let Some(add) = x.pop() {
        if b_res >= problema.pesos[add] {
            s.push(add);
            b_res = b_res - problema.pesos[add];
        }
    }
}

pub fn first_improvement(k: usize, s: &[usize], problema: &Problema) -> Vec<usize> {
    let mut v_menos_s: Vec<usize> = (0..problema.valores.len())
        .filter(|e| !s.contains(&e))
        .collect();
    v_menos_s.shuffle(&mut rand::thread_rng());

    if k == 1 {
        for adicionar in v_menos_s {
            let mut s_tmp = Vec::from(s);

            s_tmp.push(adicionar);

            let mut b_res = problema.b - calcular_peso_total(&s_tmp, problema);
            while b_res < 0.0 {
                let removido = s_tmp.pop().unwrap();
                if removido == adicionar {
                    s_tmp.push(removido);
                    s_tmp.reverse();
                } else {
                    b_res = b_res + problema.pesos[removido];
                }
            }

            if calcular_valor_total(s, problema) < calcular_valor_total(&s_tmp, problema) {
                return s_tmp;
            }
        }
    } else if k == 2 {
        for removido in s {
            let mut s_tmp: Vec<usize> = s.iter().filter(|el| *el != removido).cloned().collect();

            let mut b_res = problema.b - calcular_peso_total(&s_tmp, problema);
            let mut adicionados = 0;

            while let Some(add) = v_menos_s.pop() {
                if adicionados == 2 {
                    break;
                }

                if b_res >= problema.pesos[add] {
                    s_tmp.push(add);
                    adicionados += 1;
                    b_res = b_res - problema.pesos[add];
                }
            }

            if calcular_valor_total(s, problema) < calcular_valor_total(&s_tmp, problema) {
                return s_tmp;
            }
        }
    }

    Vec::from(s)
}

fn local_search(s: &[usize], problema: &Problema) -> Vec<usize> {
    let mut k = 1;
    let mut s = Vec::from(s);
    let mut iter = 0;
    let max_iter = 200;

    while k <= 2 && iter < max_iter {
        iter += 1;

        let sp = first_improvement(k, &s, problema);
        if calcular_valor_total(&sp, problema) <= calcular_valor_total(&s, problema) {
            k += 1;
        } else {
            k = 1;
            s = sp;
            add_free_vertices(&mut s, problema);
        }
    }

    s
}

pub fn ils_vnd(pesos: &Pesos, valores: &Valores, multas: &Multas, b: f64) -> Vec<usize> {
    let max_iter = 5;
    let c1 = 1;
    let c2 = 3;
    let c3 = 4;
    let c4 = 2;

    let problema = &Problema {
        pesos,
        valores,
        multas,
        b,
    };

    let guloso = false;
    let mut s: Vec<usize>;
    if guloso {
        s = dq_vec::<usize>(&guloso_total(
            problema.pesos,
            problema.valores,
            problema.multas,
            problema.b,
        ));
    } else {
        s = initialize(problema);
    }

    s = local_search(&s, problema);
    let mut s_opt: Vec<usize> = s.clone();

    let mut otimo_valor_local = calcular_valor_total(&s, problema);

    let mut i = 1;

    for _i in 0..max_iter {
        let mut sp: Vec<usize> = perturb(c1, &s, problema);
        sp = local_search(&sp, problema);

        // Aceitação
        if calcular_valor_total(&s, problema) < calcular_valor_total(&sp, problema) {
            s = sp;
            i = 1;

            let valor_s = calcular_valor_total(&s, problema);

            if otimo_valor_local < valor_s {
                otimo_valor_local = valor_s;
                i = i - s.len() / c2;
            }

            if calcular_valor_total(&s_opt, problema) < valor_s {
                s_opt = s.clone(); // Valor movido, segurança
                i = i - s.len() * c3;
            }
        } else if i <= s.len() / c2 {
            i += 1;
        } else {
            otimo_valor_local = calcular_valor_total(&s, problema);
            s = perturb(c4, &s, &problema);
        }
    }

    s_opt
}
