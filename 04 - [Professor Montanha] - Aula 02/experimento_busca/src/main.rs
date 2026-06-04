use std::time::Instant;

fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
        }
    }

    (resultado, operacoes)
}

fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }

    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn executar_experimento(tamanho: usize, alvo: i32) {
    let vetor = gerar_vetor(tamanho);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Elemento procurado: {}", alvo);
    println!("{}", "-".repeat(60));

    let inicio = Instant::now();
    let (resultado1, ops1) = busca_sequencial_simples(&vetor, alvo);
    let tempo1 = inicio.elapsed();

    println!("\n Busca Sequencial Simples:");
    println!("  Resultado: {:?}", resultado1);
    println!("  Operações: {}", ops1);
    println!("  Tempo: {:?}", tempo1);

    let inicio = Instant::now();
    let (resultado2, ops2) = busca_sequencial_interrompida(&vetor, alvo);
    let tempo2 = inicio.elapsed();

    println!("\n Busca Sequencial Interrompida:");
    println!("  Resultado: {:?}", resultado2);
    println!("  Operações: {}", ops2);
    println!("  Tempo: {:?}", tempo2);

    println!("\n Análise Comparativa:");
    println!(
        "  Diferença de operações: {} operações",
        ops1.saturating_sub(ops2)
    );
    if tempo1 > tempo2 {
        println!("  O Algoritmo com interrupção foi mais rápido");
    } else if tempo2 > tempo1 {
        println!("  O Algoritmo simples foi mais rápido (provavelmente devido à variação)");
    } else {
        println!("  Os Tempos foram praticamente iguais");
    }
    println!("{}", "=".repeat(60));
}

fn experimento_buscas_multiplas(tamanho: usize, num_buscas: usize) {
    let vetor = gerar_vetor(tamanho);
    let inicio = Instant::now();

    for _ in 0..num_buscas {
        let _ = busca_sequencial_simples(&vetor, 5);
    }
    let tempo_simples = inicio.elapsed();

    let inicio = Instant::now();
    for _ in 0..num_buscas {
        let _ = busca_sequencial_interrompida(&vetor, 5);
    }
    let tempo_interrupcao = inicio.elapsed();

    println!("\n{} buscas em vetor de tamanho {}:", num_buscas, tamanho);
    println!("  Simples: {:?}", tempo_simples);
    println!("  Interrupção: {:?}", tempo_interrupcao);
}

fn busca_sequencial_simples_string(vetor: &[String], alvo: &str) -> (Option<usize>, usize) {
    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
        }
    }

    (resultado, operacoes)
}

fn busca_sequencial_interrompida_string(vetor: &[String], alvo: &str) -> (Option<usize>, usize) {
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }

    (None, operacoes)
}

fn executar_experimento_strings() {
    let nomes = vec![
        String::from("Alice"),
        String::from("Bruno"),
        String::from("Carlos"),
        String::from("Diana"),
        String::from("Eduardo"),
        String::from("Fernanda"),
        String::from("Gabriel"),
        String::from("Helena"),
        String::from("Igor"),
        String::from("Julia"),
    ];

    println!("\n{}", "=".repeat(60));
    println!("Ex 1: Busca com Strings");
    println!("Vetor: {:?}", nomes);
    println!("{}", "-".repeat(60));

    let alvo = "Gabriel";
    println!("Elemento procurado: \"{}\"", alvo);

    let inicio = Instant::now();
    let (resultado1, ops1) = busca_sequencial_simples_string(&nomes, alvo);
    let tempo1 = inicio.elapsed();

    println!("\n Busca Sequencial Simples (Strings):");
    println!("  Resultado: {:?}", resultado1);
    println!("  Operações: {}", ops1);
    println!("  Tempo: {:?}", tempo1);

    let inicio = Instant::now();
    let (resultado2, ops2) = busca_sequencial_interrompida_string(&nomes, alvo);
    let tempo2 = inicio.elapsed();

    println!("\n Busca Sequencial Interrompida (Strings):");
    println!("  Resultado: {:?}", resultado2);
    println!("  Operações: {}", ops2);
    println!("  Tempo: {:?}", tempo2);

    println!("\n Análise Comparativa:");
    println!(
        "  Diferença de operações: {} operações",
        ops1.saturating_sub(ops2)
    );
    if tempo1 > tempo2 {
        println!("  O Algoritmo com interrupção foi mais rápido");
    } else if tempo2 > tempo1 {
        println!("  O Algoritmo simples foi mais rápido (provavelmente devido à variação)");
    } else {
        println!("  Os Tempos foram praticamente iguais");
    }
    println!("{}", "=".repeat(60));
}

fn contar_ocorrencias(vetor: &[i32], alvo: i32) -> (usize, usize) {
    let mut contagem = 0;
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            contagem += 1;
        }
    }

    (contagem, operacoes)
}

fn executar_experimento_contagem() {
    let vetor = vec![3, 7, 2, 7, 5, 7, 1, 9, 7, 4, 7, 8, 6, 7, 10];

    println!("\n{}", "=".repeat(60));
    println!("Ex 2: Contar Ocorrências");
    println!("Vetor: {:?}", vetor);
    println!("{}", "-".repeat(60));

    let alvo = 7;
    println!("Elemento procurado: {}", alvo);

    let (contagem, operacoes) = contar_ocorrencias(&vetor, alvo);

    println!("\n Resultado:");
    println!("  O elemento {} aparece {} vezes no vetor", alvo, contagem);
    println!("  Operações realizadas: {}", operacoes);
    println!("{}", "=".repeat(60));
}

fn buscar_todas_posicoes(vetor: &[i32], alvo: i32) -> (Vec<usize>, usize) {
    let mut posicoes = Vec::new();
    let mut operacoes = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            posicoes.push(i);
        }
    }

    (posicoes, operacoes)
}

fn executar_experimento_todas_posicoes() {
    let vetor = vec![3, 7, 2, 7, 5, 7, 1, 9, 7, 4, 7, 8, 6, 7, 10];

    println!("\n{}", "=".repeat(60));
    println!("Ex 4: Buscar Todas as Posições");
    println!("Vetor: {:?}", vetor);
    println!("{}", "-".repeat(60));

    let alvo = 7;
    println!("Elemento procurado: {}", alvo);

    let (posicoes, operacoes) = buscar_todas_posicoes(&vetor, alvo);

    println!("\n Resultado:");
    println!(
        "  O elemento {} foi encontrado nas posições: {:?}",
        alvo, posicoes
    );
    println!("  Total de ocorrências: {}", posicoes.len());
    println!("  Operações realizadas: {}", operacoes);
    println!("{}", "=".repeat(60));
}


fn main() {
    println!("\n    Experimento: Comparação de Algoritmos de Busca\n");

    println!("\n    Cenário 1: Elemento no início (melhor caso para interrupção)");
    executar_experimento(1_000, 5);
    executar_experimento(100_000, 5);
    executar_experimento(1_000_000, 5);
    println!("\n\n    Cenário 2: Elemento no meio do vetor");
    executar_experimento(1_000, 500);
    executar_experimento(100_000, 50_000);
    executar_experimento(1_000_000, 500_000);

    println!("\n\n    Cenário 3: Elemento no final (pior caso)");
    executar_experimento(1_000, 1_000);
    executar_experimento(100_000, 100_000);
    executar_experimento(1_000_000, 1_000_000);

    println!("\n\n    Cenário 4: Elemento não existe no vetor");
    executar_experimento(1_000, -1);
    executar_experimento(100_000, -1);
    executar_experimento(1_000_000, -1);
    println!("\n\n    Experimento Adicional: Buscas Múltiplas");
    experimento_buscas_multiplas(100_000, 1_000);

    println!("\n\n    Exercícios Propostos");

    println!("\n    Exercício 1: Busca com Strings");
    executar_experimento_strings();

    println!("\n    Exercício 2: Contar Ocorrências");
    executar_experimento_contagem();

    println!("\n    Exercício 4: Buscar Todas as Posições");
    executar_experimento_todas_posicoes();

    println!("\n\n Experimento concluído!\n");
}
