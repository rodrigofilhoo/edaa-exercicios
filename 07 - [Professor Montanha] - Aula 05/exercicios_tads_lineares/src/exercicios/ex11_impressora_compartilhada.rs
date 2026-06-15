use std::collections::VecDeque;

struct Trabalho {
    nome: String,
    paginas: u32,
}

pub fn impressora_compartilhada() {
    let mut fila: VecDeque<Trabalho> = VecDeque::new();

    fila.push_back(Trabalho { nome: String::from("Relatorio.pdf"), paginas: 10 });
    fila.push_back(Trabalho { nome: String::from("Foto.jpg"), paginas: 1 });
    fila.push_back(Trabalho { nome: String::from("Tese.docx"), paginas: 50 });
    fila.push_back(Trabalho { nome: String::from("Slide.pptx"), paginas: 15 });

    let mut total_paginas: u32 = 0;

    while let Some(trabalho) = fila.pop_front() {
        println!("  Imprimindo: {} ({} paginas)", trabalho.nome, trabalho.paginas);
        total_paginas += trabalho.paginas;
    }

    println!("  Total de paginas impressas: {}", total_paginas);
}
