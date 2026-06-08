pub fn imprimir_pares_e_pares(lista: &[i32]) {
    for &x in lista {
        println!("  {}", x);
    }

    for &x in lista {
        for &y in lista {
            println!("  ({}, {})", x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_executa_sem_panic() {
        imprimir_pares_e_pares(&[1, 2, 3]);
    }

    #[test]
    fn teste_lista_vazia() {
        imprimir_pares_e_pares(&[]);
    }
}
