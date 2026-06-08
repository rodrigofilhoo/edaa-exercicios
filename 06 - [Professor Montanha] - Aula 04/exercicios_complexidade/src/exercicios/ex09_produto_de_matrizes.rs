pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_matrizes_2x2() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6], vec![7, 8]];
        let resultado = produto_de_matrizes(&a, &b);
        assert_eq!(resultado, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn teste_matriz_identidade() {
        let a = vec![vec![1, 0], vec![0, 1]];
        let b = vec![vec![5, 6], vec![7, 8]];
        let resultado = produto_de_matrizes(&a, &b);
        assert_eq!(resultado, vec![vec![5, 6], vec![7, 8]]);
    }

    #[test]
    fn teste_matrizes_3x3() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        let resultado = produto_de_matrizes(&a, &b);
        assert_eq!(
            resultado,
            vec![vec![30, 24, 18], vec![84, 69, 54], vec![138, 114, 90]]
        );
    }
}
