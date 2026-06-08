pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;
    for &elemento in lista {
        total += elemento;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_soma_normal() {
        assert_eq!(somar_lista(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn teste_lista_vazia() {
        let vazia: &[i32] = &[];
        assert_eq!(somar_lista(vazia), 0);
    }

    #[test]
    fn teste_lista_um_elemento() {
        assert_eq!(somar_lista(&[7]), 7);
    }

    #[test]
    fn teste_soma_negativos() {
        assert_eq!(somar_lista(&[-1, -2, -3]), -6);
    }
}
