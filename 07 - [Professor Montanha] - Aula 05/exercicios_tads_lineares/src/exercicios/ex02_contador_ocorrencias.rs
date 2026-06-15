pub fn contar_ocorrencias(vec: &[char]) {
    let mut letras: Vec<char> = Vec::new();
    let mut contagens: Vec<usize> = Vec::new();

    for x in vec {
        let mut encontrado = false;
        for i in 0..letras.len() {
            if letras[i] == *x {
                contagens[i] += 1;
                encontrado = true;
                break;
            }
        }
        if !encontrado {
            letras.push(*x);
            contagens.push(1);
        }
    }

    for i in 0..letras.len() {
        println!("  '{}': {}", letras[i], contagens[i]);
    }
}
