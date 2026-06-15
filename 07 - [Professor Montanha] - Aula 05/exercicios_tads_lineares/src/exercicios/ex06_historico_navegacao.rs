pub struct Navegador {
    pagina_atual: String,
    historico_back: Vec<String>,
    historico_forward: Vec<String>,
}

impl Navegador {
    pub fn novo(pagina_inicial: &str) -> Navegador {
        Navegador {
            pagina_atual: String::from(pagina_inicial),
            historico_back: Vec::new(),
            historico_forward: Vec::new(),
        }
    }

    // Complexidade: O(1)
    pub fn navegar(&mut self, pagina: &str) {
        self.historico_back.push(self.pagina_atual.clone());
        self.pagina_atual = String::from(pagina);
        self.historico_forward.clear();
    }

    // Complexidade: O(1)
    pub fn voltar(&mut self) {
        if let Some(pagina) = self.historico_back.pop() {
            self.historico_forward.push(self.pagina_atual.clone());
            self.pagina_atual = pagina;
        }
    }

    // Complexidade: O(1)
    pub fn avancar(&mut self) {
        if let Some(pagina) = self.historico_forward.pop() {
            self.historico_back.push(self.pagina_atual.clone());
            self.pagina_atual = pagina;
        }
    }

    pub fn pagina_atual(&self) -> &str {
        &self.pagina_atual
    }
}
