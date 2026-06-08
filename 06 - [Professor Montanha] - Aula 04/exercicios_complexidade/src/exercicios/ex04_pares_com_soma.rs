pub fn pares_com_soma(lista: &[i32], alvo: i32) {
    let n = lista.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("  {} + {} = {}", lista[i], lista[j], alvo);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_pares_existem() {
        // Verifica que nao causa panic
        pares_com_soma(&[1, 2, 3, 4, 5], 6);
    }

    #[test]
    fn teste_lista_vazia() {
        pares_com_soma(&[], 5);
    }

    #[test]
    fn teste_um_elemento() {
        pares_com_soma(&[3], 6);
    }
}
