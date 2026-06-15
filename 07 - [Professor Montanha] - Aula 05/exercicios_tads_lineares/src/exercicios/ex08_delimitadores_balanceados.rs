pub fn delimitadores_balanceados(expressao: &str) -> bool {
    let mut pilha: Vec<char> = Vec::new();

    for c in expressao.chars() {
        if c == '(' || c == '[' || c == '{' {
            pilha.push(c);
        } else if c == ')' {
            if pilha.pop() != Some('(') {
                return false;
            }
        } else if c == ']' {
            if pilha.pop() != Some('[') {
                return false;
            }
        } else if c == '}' {
            if pilha.pop() != Some('{') {
                return false;
            }
        }
    }

    pilha.is_empty()
}
