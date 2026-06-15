use std::collections::VecDeque;

pub fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    let mut num_lote = 1;

    while !fila.is_empty() {
        let mut lote: Vec<i32> = Vec::new();
        for _ in 0..tamanho_lote {
            if let Some(elemento) = fila.pop_front() {
                lote.push(elemento);
            } else {
                break;
            }
        }
        println!("  Lote {}: {:?}", num_lote, lote);
        num_lote += 1;
    }
}
