pub struct FilaCircular {
    buffer: Vec<Option<String>>,
    capacidade: usize,
    inicio: usize,
    fim: usize,
    tamanho: usize,
}

impl FilaCircular {
    pub fn nova(capacidade: usize) -> FilaCircular {
        let mut buffer = Vec::new();
        for _ in 0..capacidade {
            buffer.push(None);
        }
        FilaCircular {
            buffer,
            capacidade,
            inicio: 0,
            fim: 0,
            tamanho: 0,
        }
    }

    pub fn inserir(&mut self, mensagem: String) {
        if self.tamanho == self.capacidade {
            self.inicio = (self.inicio + 1) % self.capacidade;
        } else {
            self.tamanho += 1;
        }
        self.buffer[self.fim] = Some(mensagem);
        self.fim = (self.fim + 1) % self.capacidade;
    }

    pub fn remover(&mut self) -> Option<String> {
        if self.tamanho == 0 {
            return None;
        }
        let mensagem = self.buffer[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;
        mensagem
    }
}
