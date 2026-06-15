use std::collections::VecDeque;

pub fn eh_palindromo(texto: &str) -> bool {
    let mut deque: VecDeque<char> = VecDeque::new();

    for c in texto.chars() {
        if c != ' ' {
            deque.push_back(c.to_ascii_lowercase());
        }
    }

    while deque.len() > 1 {
        let frente = deque.pop_front().unwrap();
        let tras = deque.pop_back().unwrap();
        if frente != tras {
            return false;
        }
    }

    true
}
