#![allow(dead_code, non_upper_case_globals)]

use rand::rngs::adapter::ReseedingRng;

use crate::{
    calcular_peso_valor_total,
    cplex::resolver_por_cplex,
    instancias::{gerar_instancias, ler_instancias},
    Executor, Problema,
};

#[test]
fn libcplex() {
    let n_itens = 50;
    let (pesos, valores, multas, b) = &gerar_instancias(n_itens).unwrap();
    let b = *b;

    let problema = &Problema {
        pesos,
        valores,
        multas,
        b,
    };

    let sol_cplex = resolver_por_cplex(b, &multas, &valores, &pesos);
    println!(
        "CPLEX: {:?} = {:?}",
        sol_cplex,
        calcular_peso_valor_total(&sol_cplex.1, problema)
    );
}

#[test]
fn benchmark_2() {
    let mut n_guloso_melhor = 0;
    let mut n_carrossel_melhor = 0;
    let mut n_solucoes_iguais = 0;

    let mut tempos_cplex = Vec::new();
    let mut tempos_gulso = Vec::new();
    let mut tempos_carrossel = Vec::new();

    let iters = 100;

    let executores = (Executor::CPlex, Executor::Guloso, Executor::Carrossel);
    let (cplex, guloso, carrossel) = executores;

    for _ in 0..iters {
        let n_itens = 500;

        let (pesos, valores, multas, b) = &gerar_instancias(n_itens).unwrap();
        let b = *b;

        let problema = &Problema {
            pesos,
            valores,
            multas,
            b,
        };

        // Otimo
        let (tempo_cplex, sopt) = cplex.executar(b, &multas, &valores, &pesos);
        tempos_cplex.push(tempo_cplex);
        let (peso_total, valor_total, n_penalidades) = calcular_peso_valor_total(&sopt, problema);

        println!("\nOtimo");
        println!("peso necessario: {}", peso_total);
        println!("valor otimo:     {}", valor_total);
        println!("penalidades:     {}", n_penalidades);
        println!("em:              {}s", tempo_cplex);
        let _valor_otimo = valor_total;

        let valor_total_guloso;
        let valor_total_carrossel;

        {
            // Guloso
            let (tempo_guloso, sg) = guloso.executar(b, &multas, &valores, &pesos);
            tempos_gulso.push(tempo_guloso);
            let (peso_total, valor_total, n_penalidades) = calcular_peso_valor_total(&sg, problema);

            println!("\nGuloso");
            println!("peso total:      {}", peso_total);
            println!("valor total:     {}", valor_total);
            println!("penalidades:     {}", n_penalidades);
            println!("em:              {}s", tempo_guloso);

            valor_total_guloso = valor_total;
        }

        {
            // Carrossel
            println!("\nCarrossel");
            let (tempo_carrossel, sc) = carrossel.executar(b, &multas, &valores, &pesos);
            tempos_carrossel.push(tempo_carrossel);
            let (peso_total, valor_total_c, n_penalidades) =
                calcular_peso_valor_total(&sc, problema);
            println!("peso total:      {}", peso_total);
            println!("valor total:     {}", valor_total_c);
            println!("penalidades:     {}", n_penalidades);
            println!("em:              {}s", tempo_carrossel);
            valor_total_carrossel = valor_total;
        }

        // // Diferença
        // let mut achado = false;
        // for (ai, bi) in sc.iter().zip(sg.iter()) {
        //     if ai != bi {
        //         achado = true;
        //         break;
        //     }
        // }

        if valor_total_carrossel > valor_total_guloso {
            n_carrossel_melhor += 1;
        } else if valor_total_carrossel < valor_total_guloso {
            n_guloso_melhor += 1;
        } else {
            n_solucoes_iguais += 1;
        }

        println!();
        println!();
        println!("-------------------------------------------------------");
        println!();
        println!();
    }

    println!(
        "Vezes que o guloso foi melhor: {} ({})",
        n_guloso_melhor,
        n_guloso_melhor as f64 / iters as f64
    );

    println!(
        "Vezes que o carrossel foi melhor: {} ({})",
        n_carrossel_melhor,
        n_carrossel_melhor as f64 / iters as f64
    );

    println!("Soluções iguais: {}", n_solucoes_iguais);

    let tempo_medio_carrossel: f64 = tempos_carrossel.iter().sum::<f64>() / iters as f64;
    let tempo_medio_guloso: f64 = tempos_gulso.iter().sum::<f64>() / iters as f64;

    println!("Média de execução carrossel: {:?}s", tempo_medio_carrossel);
    println!("Média de execução guloso:    {:?}s", tempo_medio_guloso);
}

#[test]
fn g_benchmark() {
    let iters = 100;

    let executores = [Executor::Guloso, Executor::Carrossel];

    let mut melhores = vec![0; executores.len()];
    let mut tempos = Vec::new();

    for _i in 0..iters {
        let n_itens = 500;

        let (pesos, valores, multas, b) = &gerar_instancias(n_itens).unwrap();
        let b = *b;

        let problema = &Problema {
            pesos,
            valores,
            multas,
            b,
        };

        let mut valores_observados = vec![0.0; executores.len()];
        let mut tempos_observados = Vec::new();

        for (idx_executor, executor) in executores.iter().enumerate() {
            let (tempo, s) = executor.executar(b, &multas, &valores, &pesos);
            let (_peso, valor, _penalidades) = calcular_peso_valor_total(&s, problema);

            valores_observados[idx_executor] = valor;
            tempos_observados.push(tempo);
        }

        tempos.push(tempos_observados);

        let (idx, valor_max) = valores_observados
            .iter()
            .enumerate()
            .max_by(|(_, val1), (_, val2)| val1.partial_cmp(&val2).unwrap())
            .unwrap();

        let mut empate = 0;
        for (_idx, v) in valores_observados.iter().enumerate() {
            if v == valor_max {
                empate += 1;
            }
        }

        if empate == 1 {
            melhores[idx] += 1;
        }
    }

    for (idx_executor, executor) in executores.iter().enumerate() {
        println!("Executor: {:?}", executor);
        println!("Vezes melhor: {}", melhores[idx_executor]);

        let media_tempo_execucao: f64 = tempos
            .iter()
            .map(|tempos| tempos[idx_executor])
            .sum::<f64>()
            / iters as f64;

        println!("Média to tempo de execução: {}s", media_tempo_execucao);
        println!();
        println!();
        println!();
    }
}

#[test]
fn ils_vnd_t() {
    let executor = Executor::ILsVNd;

    let mut gaps = Vec::new();

    let mut tempos_cplex = Vec::new();
    let mut tempos_ilsvnd = Vec::new();

    let iters = 100;

    let mut otimos = 0;

    for _ in 0..iters {
        let n_itens = 500;

        let (pesos, valores, multas, b) = &gerar_instancias(n_itens).unwrap();
        let b = *b;

        let problema = &Problema {
            pesos,
            valores,
            multas,
            b,
        };

        let sol_cplex = resolver_por_cplex(b, &multas, &valores, &pesos);
        let res_cplex = calcular_peso_valor_total(&sol_cplex.1, problema);
        println!("CPLEX: {:?} = {:?}", sol_cplex, res_cplex);

        let (tempo, s) = executor.executar(b, &multas, &valores, &pesos);
        let (peso, valor, penalidades) = calcular_peso_valor_total(&s, problema);

        println!("\nILS+VND");
        println!("peso total:      {}", peso);
        println!("valor total:     {}", valor);
        println!("penalidades:     {}", penalidades);
        println!("em:              {}s", tempo);
        println!("itens:           {:?}", s);

        tempos_cplex.push(sol_cplex.0);
        tempos_ilsvnd.push(tempo);

        gaps.push((valor - res_cplex.1).abs() / res_cplex.1);

        if valor == res_cplex.1 {
            otimos += 1;
        }

        println!();
        println!();
        println!();
    }

    let tempo_medio_cplex: f64 = tempos_cplex.iter().sum::<f64>() / iters as f64;
    let tempo_medio_ilsvnd: f64 = tempos_ilsvnd.iter().sum::<f64>() / iters as f64;

    println!("Média de execução cplex:  {:?}s", tempo_medio_cplex);
    println!("Média de execução ilvvnd: {:?}s", tempo_medio_ilsvnd);
    println!(
        "Media gaps:               {:?}s",
        gaps.iter().sum::<f64>() / iters as f64
    );

    println!("Otimos: {}", otimos);
}

#[test]
fn tabela() {
    let executores = [
        Executor::CPlex,
        Executor::Guloso,
        Executor::Carrossel,
        Executor::ILsVNd,
    ];

    let densidades = [0.2, 0.4, 0.6, 0.8, 1.0];
    let mut tabela: Vec<Vec<String>> = Vec::new();
    let mut cabecalho: Vec<String> = Vec::new();
    cabecalho.push("Densidade".into());
    for executor in executores.iter() {
        cabecalho.push(format!("{:?}", executor));
        cabecalho.push("Gap".into());
        cabecalho.push("Penalidades".into());
        cabecalho.push("Tempo (s)".into());
    }
    tabela.push(cabecalho);

    for densidade in densidades {
        let (pesos, valores, multas, b) = &ler_instancias("./instancia.dat", densidade).unwrap();
        let b = *b;

        let problema = &Problema {
            pesos,
            valores,
            multas,
            b,
        };

        let mut valores_observados = vec![0.0; executores.len()];
        let mut tempos_observados = vec![0.0; executores.len()];
        let mut penalidades_observadas = vec![0; executores.len()];

        for (idx_executor, executor) in executores.iter().enumerate() {
            println!("Executando {:?}...", executor);
            let (tempo, s) = executor.executar(b, &multas, &valores, &pesos);
            let (_peso, valor, penalidades) = calcular_peso_valor_total(&s, problema);

            valores_observados[idx_executor] = valor;
            tempos_observados[idx_executor] = tempo;
            penalidades_observadas[idx_executor] = penalidades;
        }

        let mut linha: Vec<String> = Vec::new();
        linha.push(format!("{}", densidade).into());

        for (idx_executor, _executor) in executores.iter().enumerate() {
            linha.push("".into());
            let gap = (valores_observados[idx_executor] - valores_observados[0]).abs()
                / valores_observados[0];

            linha.push(format!("{}", gap).into());
            linha.push(format!("{}", penalidades_observadas[idx_executor]).into());
            linha.push(format!("{}", tempos_observados[idx_executor]).into());
        }

        tabela.push(linha);
    }

    for linha in tabela {
        println!("{:?}", linha);
    }
}
