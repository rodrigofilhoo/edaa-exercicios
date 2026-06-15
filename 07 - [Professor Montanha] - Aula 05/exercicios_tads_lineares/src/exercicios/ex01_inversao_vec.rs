pub fn inverter_vec(vec: &[i32]) -> Vec<i32> {
    let mut original = vec.to_vec();
    let mut invertido = Vec::new();

    while let Some(elemento) = original.pop() {
        invertido.push(elemento);
    }

    invertido
}
