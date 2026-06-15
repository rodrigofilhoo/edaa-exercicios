pub struct PilhaMin {
    pilha: Vec<i32>,
    minimos: Vec<i32>,
}

impl PilhaMin {
    pub fn nova() -> PilhaMin {
        PilhaMin {
            pilha: Vec::new(),
            minimos: Vec::new(),
        }
    }

    // Complexidade: O(1)
    pub fn push(&mut self, valor: i32) {
        self.pilha.push(valor);
        if self.minimos.is_empty() || valor <= *self.minimos.last().unwrap() {
            self.minimos.push(valor);
        }
    }

    // Complexidade: O(1)
    pub fn pop(&mut self) -> Option<i32> {
        if let Some(valor) = self.pilha.pop() {
            if Some(&valor) == self.minimos.last() {
                self.minimos.pop();
            }
            Some(valor)
        } else {
            None
        }
    }

    // Complexidade: O(1)
    pub fn min(&self) -> Option<i32> {
        self.minimos.last().copied()
    }
}
