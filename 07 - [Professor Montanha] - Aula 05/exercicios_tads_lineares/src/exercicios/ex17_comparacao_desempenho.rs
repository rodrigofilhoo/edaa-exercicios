use std::collections::VecDeque;
use std::time::Instant;

pub fn comparar_desempenho() {
    let n: i32 = 10_000;

    let inicio = Instant::now();
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    while !vec.is_empty() {
        vec.remove(0);
    }
    let tempo_vec = inicio.elapsed();

    let inicio = Instant::now();
    let mut deque: VecDeque<i32> = VecDeque::new();
    for i in 0..n {
        deque.push_back(i);
    }
    while !deque.is_empty() {
        deque.pop_front();
    }
    let tempo_deque = inicio.elapsed();

    let inicio = Instant::now();
    let cap = n as usize;
    let mut buffer: Vec<i32> = vec![0; cap];
    let mut head: usize = 0;
    let mut tail: usize = 0;
    let mut size: usize = 0;
    for i in 0..n {
        buffer[tail] = i;
        tail = (tail + 1) % cap;
        size += 1;
    }
    while size > 0 {
        let _ = buffer[head];
        head = (head + 1) % cap;
        size -= 1;
    }
    let tempo_circular = inicio.elapsed();

    println!("  10.000 enqueue/dequeue:");
    println!("  Vec ingenua:    {:?}", tempo_vec);
    println!("  VecDeque:       {:?}", tempo_deque);
    println!("  Fila circular:  {:?}", tempo_circular);
}
