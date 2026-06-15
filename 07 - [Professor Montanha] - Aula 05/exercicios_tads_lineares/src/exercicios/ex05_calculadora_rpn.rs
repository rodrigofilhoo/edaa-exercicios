pub fn calculadora_rpn(expressao: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();

    for token in expressao.split_whitespace() {
        if token == "+" || token == "-" || token == "*" || token == "/" {
            let b = pilha.pop().unwrap();
            let a = pilha.pop().unwrap();
            let resultado = if token == "+" {
                a + b
            } else if token == "-" {
                a - b
            } else if token == "*" {
                a * b
            } else {
                a / b
            };
            pilha.push(resultado);
        } else {
            let numero: f64 = token.parse().unwrap();
            pilha.push(numero);
        }
    }

    pilha.pop().unwrap()
}
