use std::collections::VecDeque;

struct Processo {
    id: usize,
    tempo_restante: u32,
}

pub fn round_robin(tempos: &[u32], quantum: u32) {
    let n = tempos.len();
    let mut fila: VecDeque<Processo> = VecDeque::new();
    let mut conclusoes: Vec<u32> = vec![0; n];

    for i in 0..n {
        fila.push_back(Processo {
            id: i,
            tempo_restante: tempos[i],
        });
    }

    let mut tempo_atual: u32 = 0;

    while !fila.is_empty() {
        let mut processo = fila.pop_front().unwrap();

        if processo.tempo_restante <= quantum {
            tempo_atual += processo.tempo_restante;
            processo.tempo_restante = 0;
            conclusoes[processo.id] = tempo_atual;
            println!("  Tempo {}: Processo {} concluido", tempo_atual, processo.id + 1);
        } else {
            tempo_atual += quantum;
            processo.tempo_restante -= quantum;
            println!("  Tempo {}: Processo {} executou {} (restante: {})",
                tempo_atual, processo.id + 1, quantum, processo.tempo_restante);
            fila.push_back(processo);
        }
    }

    println!("  Tempo de conclusao:");
    for i in 0..n {
        println!("    Processo {}: {}", i + 1, conclusoes[i]);
    }
}
