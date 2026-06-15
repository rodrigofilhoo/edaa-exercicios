pub fn remover_pares(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 == 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
