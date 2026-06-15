pub fn mescla_ordenada(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultado.push(a[i]);
            i += 1;
        } else {
            resultado.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        resultado.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        resultado.push(b[j]);
        j += 1;
    }

    resultado
}
