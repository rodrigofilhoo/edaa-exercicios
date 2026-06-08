pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_lista_com_elementos() {
        assert_eq!(verificar_primeiro(&[10, 20, 30]), Some(10));
    }

    #[test]
    fn teste_lista_vazia() {
        let vazia: &[i32] = &[];
        assert_eq!(verificar_primeiro(vazia), None);
    }

    #[test]
    fn teste_lista_um_elemento() {
        assert_eq!(verificar_primeiro(&[42]), Some(42));
    }
}
