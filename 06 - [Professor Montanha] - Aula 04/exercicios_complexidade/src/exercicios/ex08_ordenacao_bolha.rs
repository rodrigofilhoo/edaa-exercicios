pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_ordenacao_normal() {
        let mut lista = vec![5, 3, 8, 1, 2];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn teste_ja_ordenada() {
        let mut lista = vec![1, 2, 3, 4, 5];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn teste_ordem_inversa() {
        let mut lista = vec![5, 4, 3, 2, 1];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn teste_lista_vazia() {
        let mut lista: Vec<i32> = vec![];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![]);
    }

    #[test]
    fn teste_um_elemento() {
        let mut lista = vec![42];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![42]);
    }
}
