use std::collections::VecDeque;

struct Cliente {
    nome: String,
    chegada: u32,
    atendimento: u32,
}

pub fn simulador_fila_banco() {
    let mut fila: VecDeque<Cliente> = VecDeque::new();

    fila.push_back(Cliente { nome: String::from("Cliente 1"), chegada: 0, atendimento: 3 });
    fila.push_back(Cliente { nome: String::from("Cliente 2"), chegada: 1, atendimento: 5 });
    fila.push_back(Cliente { nome: String::from("Cliente 3"), chegada: 3, atendimento: 2 });
    fila.push_back(Cliente { nome: String::from("Cliente 4"), chegada: 5, atendimento: 4 });
    fila.push_back(Cliente { nome: String::from("Cliente 5"), chegada: 6, atendimento: 3 });
    fila.push_back(Cliente { nome: String::from("Cliente 6"), chegada: 9, atendimento: 2 });
    fila.push_back(Cliente { nome: String::from("Cliente 7"), chegada: 10, atendimento: 6 });
    fila.push_back(Cliente { nome: String::from("Cliente 8"), chegada: 12, atendimento: 1 });

    let mut tempo_atual: u32 = 0;
    let mut espera_total: u32 = 0;
    let mut atendidos: u32 = 0;

    while let Some(cliente) = fila.pop_front() {
        if tempo_atual < cliente.chegada {
            tempo_atual = cliente.chegada;
        }
        let espera = tempo_atual - cliente.chegada;
        espera_total += espera;
        println!("  {}: chegou={}, inicio={}, espera={}",
            cliente.nome, cliente.chegada, tempo_atual, espera);
        tempo_atual += cliente.atendimento;
        atendidos += 1;
    }

    let media = espera_total as f64 / atendidos as f64;
    println!("  Tempo medio de espera: {:.1}", media);
}
