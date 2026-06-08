pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_encontra_elemento() {
        let lista = vec![1, 3, 5, 7, 9, 11, 13, 15];
        assert_eq!(busca_binaria(&lista, 7), Some(3));
    }

    #[test]
    fn teste_elemento_nao_existe() {
        let lista = vec![1, 3, 5, 7, 9];
        assert_eq!(busca_binaria(&lista, 4), None);
    }

    #[test]
    fn teste_primeiro_elemento() {
        let lista = vec![2, 4, 6, 8];
        assert_eq!(busca_binaria(&lista, 2), Some(0));
    }

    #[test]
    fn teste_ultimo_elemento() {
        let lista = vec![2, 4, 6, 8];
        assert_eq!(busca_binaria(&lista, 8), Some(3));
    }

    #[test]
    fn teste_lista_vazia() {
        let vazia: &[i32] = &[];
        assert_eq!(busca_binaria(vazia, 1), None);
    }
}
