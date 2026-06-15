use std::collections::VecDeque;

pub fn janela_deslizante_maxima(vec: &[i32], k: usize) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..vec.len() {
        while !deque.is_empty() && *deque.front().unwrap() + k <= i {
            deque.pop_front();
        }

        while !deque.is_empty() && vec[*deque.back().unwrap()] <= vec[i] {
            deque.pop_back();
        }

        deque.push_back(i);

        if i >= k - 1 {
            resultado.push(vec[*deque.front().unwrap()]);
        }
    }

    resultado
}
