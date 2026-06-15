mod exercicios;

use std::collections::VecDeque;

fn main() {
    println!("Exercicios - Tads Lineares (Vec, Pilha, Fila, Deque)");
    println!("{}", "=".repeat(55));

    // Grupo 1 - Vec e Operacoes Basicas
    println!("\nGrupo 1 - Vec e Operacoes Basicas");
    println!("{}", "-".repeat(55));

    println!("\nEx 01 - Inversao com Vec");
    let vetor = vec![1, 2, 3, 4, 5];
    println!("  Original: {:?}", vetor);
    let invertido = exercicios::ex01_inversao_vec::inverter_vec(&vetor);
    println!("  Invertido: {:?}", invertido);

    println!("\nEx 02 - Contador de ocorrencias");
    let frase: Vec<char> = "abracadabra".chars().collect();
    println!("  Frase: abracadabra");
    exercicios::ex02_contador_ocorrencias::contar_ocorrencias(&frase);

    println!("\nEx 03 - Remocao condicional");
    let mut vetor = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("  Antes: {:?}", vetor);
    exercicios::ex03_remocao_condicional::remover_pares(&mut vetor);
    println!("  Depois: {:?}", vetor);

    println!("\nEx 04 - Mescla ordenada");
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];
    println!("  Vec A: {:?}", a);
    println!("  Vec B: {:?}", b);
    let mesclado = exercicios::ex04_mescla_ordenada::mescla_ordenada(&a, &b);
    println!("  Mesclado: {:?}", mesclado);

    // Grupo 2 - Pilha (Stack)
    println!("\n\nGrupo 2 - Pilha (Stack)");
    println!("{}", "-".repeat(55));

    println!("\nEx 05 - Calculadora RPN");
    let expressao = "3 4 + 2 *";
    let resultado = exercicios::ex05_calculadora_rpn::calculadora_rpn(expressao);
    println!("  Expressao: {}", expressao);
    println!("  Resultado: {}", resultado);

    println!("\nEx 06 - Historico de navegacao");
    let mut nav = exercicios::ex06_historico_navegacao::Navegador::novo("google.com");
    println!("  Pagina atual: {}", nav.pagina_atual());
    nav.navegar("youtube.com");
    println!("  Navegar para youtube.com: {}", nav.pagina_atual());
    nav.navegar("github.com");
    println!("  Navegar para github.com: {}", nav.pagina_atual());
    nav.voltar();
    println!("  Voltar: {}", nav.pagina_atual());
    nav.voltar();
    println!("  Voltar: {}", nav.pagina_atual());
    nav.avancar();
    println!("  Avancar: {}", nav.pagina_atual());

    println!("\nEx 07 - Desfazer/Refazer");
    let mut editor = exercicios::ex07_desfazer_refazer::Editor::novo();
    editor.digitar("Ola");
    println!("  Digitar 'Ola': \"{}\"", editor.texto_atual());
    editor.digitar(" mundo");
    println!("  Digitar ' mundo': \"{}\"", editor.texto_atual());
    editor.desfazer();
    println!("  Desfazer: \"{}\"", editor.texto_atual());
    editor.refazer();
    println!("  Refazer: \"{}\"", editor.texto_atual());

    println!("\nEx 08 - Delimitadores balanceados");
    let expressoes = ["{[()]}", "([)]", "((("];
    for e in &expressoes {
        let result = exercicios::ex08_delimitadores_balanceados::delimitadores_balanceados(e);
        if result {
            println!("  \"{}\": Balanceado", e);
        } else {
            println!("  \"{}\": Nao balanceado", e);
        }
    }

    println!("\nEx 09 - Pilha com minimo");
    let mut pilha = exercicios::ex09_pilha_com_minimo::PilhaMin::nova();
    pilha.push(5);
    pilha.push(3);
    pilha.push(7);
    pilha.push(1);
    println!("  Empilhados: 5, 3, 7, 1");
    println!("  Minimo: {:?}", pilha.min());
    pilha.pop();
    println!("  Apos pop: minimo = {:?}", pilha.min());
    pilha.pop();
    println!("  Apos pop: minimo = {:?}", pilha.min());

    // Grupo 3 - Fila (Queue)
    println!("\n\nGrupo 3 - Fila (Queue)");
    println!("{}", "-".repeat(55));

    println!("\nEx 10 - Simulador de fila de banco");
    exercicios::ex10_fila_banco::simulador_fila_banco();

    println!("\nEx 11 - Impressora compartilhada");
    exercicios::ex11_impressora_compartilhada::impressora_compartilhada();

    println!("\nEx 12 - Buffer de mensagens");
    let mut buffer = exercicios::ex12_buffer_mensagens::FilaCircular::nova(3);
    buffer.inserir(String::from("Msg 1"));
    buffer.inserir(String::from("Msg 2"));
    buffer.inserir(String::from("Msg 3"));
    println!("  Inseridas: Msg 1, Msg 2, Msg 3 (capacidade 3)");
    buffer.inserir(String::from("Msg 4"));
    println!("  Inserida Msg 4 (sobrescreve Msg 1)");
    while let Some(msg) = buffer.remover() {
        println!("  Removida: {}", msg);
    }

    println!("\nEx 13 - Fila de prioridade manual");
    exercicios::ex13_fila_prioridade::demonstrar_fila_prioridade();

    // Grupo 4 - Deque
    println!("\n\nGrupo 4 - Deque");
    println!("{}", "-".repeat(55));

    println!("\nEx 14 - Palindromo com Deque");
    let textos = ["A man a plan a canal Panama", "Rust", "aba"];
    for t in &textos {
        let result = exercicios::ex14_palindromo_deque::eh_palindromo(t);
        if result {
            println!("  \"{}\": Palindromo", t);
        } else {
            println!("  \"{}\": Nao e palindromo", t);
        }
    }

    println!("\nEx 15 - Janela deslizante maxima");
    let vetor = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let maximos = exercicios::ex15_janela_deslizante::janela_deslizante_maxima(&vetor, k);
    println!("  Vetor: {:?}", vetor);
    println!("  Janela k={}: {:?}", k, maximos);

    println!("\nEx 16 - Fila de tarefas com prioridade de frente");
    let mut fila_tarefas = exercicios::ex16_fila_tarefas::FilaTarefas::nova();
    fila_tarefas.adicionar_normal("Tarefa normal 1");
    fila_tarefas.adicionar_normal("Tarefa normal 2");
    fila_tarefas.adicionar_urgente("Tarefa urgente 1");
    fila_tarefas.adicionar_normal("Tarefa normal 3");
    fila_tarefas.adicionar_urgente("Tarefa urgente 2");
    println!("  Ordem de processamento:");
    while let Some(tarefa) = fila_tarefas.processar() {
        println!("    {}", tarefa);
    }

    // Grupo 5 - Reflexao e Analise
    println!("\n\nGrupo 5 - Reflexao e Analise");
    println!("{}", "-".repeat(55));

    println!("\nEx 17 - Comparacao de desempenho");
    exercicios::ex17_comparacao_desempenho::comparar_desempenho();

    println!("\nEx 18 - Quando usar qual TAD?");
    exercicios::ex18_quando_usar_qual::quando_usar_qual();

    println!("\nEx 19 - Processar em lotes");
    let mut fila: VecDeque<i32> = VecDeque::new();
    for i in 1..=10 {
        fila.push_back(i);
    }
    println!("  Fila: {:?}", fila);
    exercicios::ex19_processar_em_lotes::processar_em_lotes(&mut fila, 3);

    println!("\nEx 20 - Round Robin");
    let tempos = vec![10, 5, 8, 3];
    let quantum = 3;
    println!("  Processos (tempo): {:?}", tempos);
    println!("  Quantum: {}", quantum);
    exercicios::ex20_round_robin::round_robin(&tempos, quantum);

    println!("\n{}", "=".repeat(55));
    println!("Exercicios concluidos.");
}
