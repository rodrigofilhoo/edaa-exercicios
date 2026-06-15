struct Item {
    nome: String,
    prioridade: u32,
    ordem: usize,
}

pub fn demonstrar_fila_prioridade() {
    let mut fila: Vec<Item> = Vec::new();

    fila.push(Item { nome: String::from("Tarefa A"), prioridade: 1, ordem: 0 });
    fila.push(Item { nome: String::from("Tarefa B"), prioridade: 3, ordem: 1 });
    fila.push(Item { nome: String::from("Tarefa C"), prioridade: 2, ordem: 2 });
    fila.push(Item { nome: String::from("Tarefa D"), prioridade: 3, ordem: 3 });
    fila.push(Item { nome: String::from("Tarefa E"), prioridade: 1, ordem: 4 });

    while !fila.is_empty() {
        let mut idx = 0;
        for i in 1..fila.len() {
            if fila[i].prioridade > fila[idx].prioridade {
                idx = i;
            } else if fila[i].prioridade == fila[idx].prioridade && fila[i].ordem < fila[idx].ordem {
                idx = i;
            }
        }
        let item = fila.remove(idx);
        println!("  Processando: {} (prioridade {})", item.nome, item.prioridade);
    }
}
