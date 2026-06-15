use std::collections::VecDeque;

pub struct FilaTarefas {
    deque: VecDeque<String>,
}

impl FilaTarefas {
    pub fn nova() -> FilaTarefas {
        FilaTarefas {
            deque: VecDeque::new(),
        }
    }

    pub fn adicionar_urgente(&mut self, tarefa: &str) {
        self.deque.push_front(String::from(tarefa));
    }

    pub fn adicionar_normal(&mut self, tarefa: &str) {
        self.deque.push_back(String::from(tarefa));
    }

    pub fn processar(&mut self) -> Option<String> {
        self.deque.pop_front()
    }
}
