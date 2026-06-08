pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    while i < esquerda.len() {
        resultado.push(esquerda[i]);
        i += 1;
    }

    while j < direita.len() {
        resultado.push(direita[j]);
        j += 1;
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_ordenacao_normal() {
        let lista = vec![38, 27, 43, 3, 9, 82, 10];
        assert_eq!(merge_sort(lista), vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn teste_ja_ordenada() {
        let lista = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(lista), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn teste_ordem_inversa() {
        let lista = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(lista), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn teste_lista_vazia() {
        let lista: Vec<i32> = vec![];
        assert_eq!(merge_sort(lista), vec![]);
    }

    #[test]
    fn teste_um_elemento() {
        let lista = vec![42];
        assert_eq!(merge_sort(lista), vec![42]);
    }
}
