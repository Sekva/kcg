use rand::prelude::SliceRandom;

use crate::{calcular_peso_total, calcular_valor_total, utils::iguais, Problema};

pub fn add_free_vertices_mwis(s: &mut Vec<usize>, problema: &Problema) {
    // Adiciona tudo que dá
    let mut x: Vec<usize> = (0..problema.valores.len())
        .filter(|e| !s.contains(&e))
        .collect();
    x.shuffle(&mut rand::thread_rng());

    let mut b_res = problema.b - calcular_peso_total(&s, problema);
    while let Some(v) = x.pop() {
        let mut achado = false;
        for (i, j, _d) in problema.multas {
            if v == *i && s.contains(&j) || v == *j && s.contains(&i) {
                achado = true;
                break;
            }
        }

        if !achado && b_res >= problema.pesos[v] {
            b_res = b_res - problema.pesos[v];
            s.push(v);
        }
    }
}

pub fn first_improvement_mwis(k: usize, s: &[usize], problema: &Problema) -> Vec<usize> {
    let s = Vec::from(s);

    let mut v_menos_s: Vec<usize> = (0..problema.valores.len())
        .filter(|e| !s.contains(&e))
        .collect();
    v_menos_s.shuffle(&mut rand::thread_rng());

    if k == 1 {
        while let Some(ins) = v_menos_s.pop() {
            let mut s_tmp = s.clone();

            let mut adjacentes: Vec<usize> = Vec::new();
            for (i, j, _d) in problema.multas {
                if ins == *i && s_tmp.contains(&j) {
                    adjacentes.push(*j);
                }

                if ins == *j && s_tmp.contains(&i) {
                    adjacentes.push(*i);
                }
            }

            // Remove todos os adjacentes (omega) à ins que tão na solução
            s_tmp = s_tmp
                .iter()
                .filter(|p_adj| !adjacentes.contains(&p_adj))
                .cloned()
                .collect();

            // Adiciona 1
            s_tmp.push(ins);

            // Remove itens até entrar no capacidade
            s_tmp.reverse();
            let mut b_res = problema.b - calcular_peso_total(&s_tmp, problema);
            while b_res < 0.0 {
                s_tmp.pop();
                b_res = problema.b - calcular_peso_total(&s_tmp, problema); //TODO: não precisa recalcular, só soma com o idxeda do pop
            }

            if calcular_valor_total(&s, problema) < calcular_valor_total(&s_tmp, problema) {
                return s_tmp;
            }
        }

        // Foi toda a vizinhança e não foi ninguem...
        return s;
    } else if k == 2 {
        for idx in 0..s.len() {
            let mut s_tmp = s.clone();
            let mut add_1: Option<usize> = None;
            let mut add_2: Option<usize> = None;

            // TODO: esocolher add_1 e add_2 de v_menos_s

            while let Some(add) = v_menos_s.pop() {
                let mut achado = false;
                for (i, j, _d) in problema.multas {
                    if (add == *i && s_tmp.contains(&j)) || (add == *j && s_tmp.contains(&i)) {
                        achado = true;
                        break;
                    }
                }

                if !achado {
                    add_1 = Some(add);
                }
            }

            while let Some(add) = v_menos_s.pop() {
                let mut achado = false;
                for (i, j, _d) in problema.multas {
                    if (add == *i && s_tmp.contains(&j)) || (add == *j && s_tmp.contains(&i)) {
                        achado = true;
                        break;
                    }
                }

                if !achado {
                    add_2 = Some(add);
                }
            }

            // Remove 1
            s_tmp.remove(idx);

            // Add 2 (ou tenta)
            if add_1.is_some() {
                s_tmp.push(add_1.unwrap());
            }

            if add_2.is_some() {
                s_tmp.push(add_2.unwrap());
            }

            // Remove itens até entrar no capacidade
            s_tmp.reverse();
            let mut b_res = problema.b - calcular_peso_total(&s_tmp, problema);
            while b_res < 0.0 {
                s_tmp.pop();
                b_res = problema.b - calcular_peso_total(&s_tmp, problema); //TODO: não precisa recalcular, só soma com o idxeda do pop
            }

            if calcular_valor_total(&s, problema) < calcular_valor_total(&s_tmp, problema) {
                return s_tmp;
            }
        }

        // Foi toda a vizinhança e não foi ninguem...
        return s;
    }

    unreachable!()
}

pub fn initialize_mwis(problema: &Problema) -> Vec<usize> {
    let mut r: Vec<usize> = Vec::new();

    let mut x: Vec<usize> = (0..problema.valores.len()).collect();
    x.shuffle(&mut rand::thread_rng());

    let mut b_res = problema.b;

    while let Some(v) = x.pop() {
        let mut achado = false;
        for (i, j, _d) in problema.multas {
            if v == *i && r.contains(&j) || v == *j && r.contains(&i) {
                achado = true;
                break;
            }
        }

        if !achado && b_res >= problema.pesos[v] {
            b_res = b_res - problema.pesos[v];
            r.push(v);
        }
    }

    r
}

pub fn perturb_mwis(k: usize, s: &[usize], problema: &Problema) -> Vec<usize> {
    let mut r = Vec::from(s);

    // Garantir que são diferentes, mas e loop infinito? MIS unico?
    while iguais(s, &r) {
        let mut x: Vec<usize> = (0..problema.valores.len())
            .filter(|e| !s.contains(&e))
            .collect();
        x.shuffle(&mut rand::thread_rng());

        let mut n_adicionados = 0;

        for v in x {
            if n_adicionados == k {
                break;
            }

            let mut adjacentes: Vec<usize> = Vec::new();
            for (i, j, _d) in problema.multas {
                if v == *i && r.contains(&j) {
                    adjacentes.push(*j);
                }

                if v == *j && r.contains(&i) {
                    adjacentes.push(*i);
                }
            }

            // Remove todos os adjacentes
            r = r
                .iter()
                .filter(|p_adj| !adjacentes.contains(&p_adj))
                .cloned()
                .collect();

            // Adiciona o novo
            r.push(v);
            n_adicionados += 1;
        }

        let mut b_res = problema.b - calcular_peso_total(&r, problema);

        // Remove itens até entrar no capacidade

        while b_res < 0.0 {
            r.pop();
            b_res = problema.b - calcular_peso_total(&r, problema);
        }

        add_free_vertices_mwis(&mut r, problema);
    }
    r
}
