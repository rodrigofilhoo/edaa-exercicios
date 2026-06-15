pub struct Editor {
    texto: String,
    pilha_desfazer: Vec<String>,
    pilha_refazer: Vec<String>,
}

impl Editor {
    pub fn novo() -> Editor {
        Editor {
            texto: String::new(),
            pilha_desfazer: Vec::new(),
            pilha_refazer: Vec::new(),
        }
    }

    // Complexidade: O(n) onde n = tamanho do texto (clone)
    pub fn digitar(&mut self, texto: &str) {
        self.pilha_desfazer.push(self.texto.clone());
        self.texto.push_str(texto);
        self.pilha_refazer.clear();
    }

    // Complexidade: O(1)
    pub fn desfazer(&mut self) {
        if let Some(estado) = self.pilha_desfazer.pop() {
            self.pilha_refazer.push(self.texto.clone());
            self.texto = estado;
        }
    }

    // Complexidade: O(1)
    pub fn refazer(&mut self) {
        if let Some(estado) = self.pilha_refazer.pop() {
            self.pilha_desfazer.push(self.texto.clone());
            self.texto = estado;
        }
    }

    pub fn texto_atual(&self) -> &str {
        &self.texto
    }
}
