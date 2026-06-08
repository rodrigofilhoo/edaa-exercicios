mod exercicios;

fn main() {
    println!("Analise de Complexidade Assintotica (Big-O)");
    println!("{}", "=".repeat(50));

    println!("\nEx 01 - Verificar primeiro - O(1)");
    let lista = vec![10, 20, 30, 40, 50];
    let resultado = exercicios::ex01_verificar_primeiro::verificar_primeiro(&lista);
    println!("  Lista: {:?}", lista);
    println!("  Primeiro elemento: {:?}", resultado);

    println!("\nEx 02 - Somar lista - O(n)");
    let lista = vec![1, 2, 3, 4, 5];
    let soma = exercicios::ex02_somar_lista::somar_lista(&lista);
    println!("  Lista: {:?}", lista);
    println!("  Soma: {}", soma);

    println!("\nEx 03 - Busca binaria - O(log n)");
    let lista = vec![1, 3, 5, 7, 9, 11, 13, 15];
    let resultado = exercicios::ex03_busca_binaria::busca_binaria(&lista, 7);
    println!("  Lista: {:?}", lista);
    println!("  Buscar 7: {:?}", resultado);

    println!("\nEx 04 - Pares com soma - O(n^2)");
    let lista = vec![1, 2, 3, 4, 5];
    println!("  Lista: {:?}", lista);
    println!("  Pares com soma 6:");
    exercicios::ex04_pares_com_soma::pares_com_soma(&lista, 6);

    println!("\nEx 05 - Imprimir pares e pares - O(n^2)");
    let lista = vec![1, 2, 3];
    println!("  Lista: {:?}", lista);
    exercicios::ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&lista);

    println!("\nEx 06 - Potencias de dois - O(log n)");
    println!("  Potencias de dois ate 32:");
    exercicios::ex06_potencias_de_dois::potencias_de_dois(32);

    println!("\nEx 07 - Fibonacci recursivo - O(2^n)");
    let n = 10;
    let resultado = exercicios::ex07_fibonacci_recursivo::fibonacci_recursivo(n);
    println!("  Fibonacci({}) = {}", n, resultado);

    println!("\nEx 08 - Ordenacao bolha - O(n^2)");
    let mut lista = vec![5, 3, 8, 1, 2];
    println!("  Antes: {:?}", lista);
    exercicios::ex08_ordenacao_bolha::ordenacao_bolha(&mut lista);
    println!("  Depois: {:?}", lista);

    println!("\nEx 09 - Produto de matrizes - O(n^3)");
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let c = exercicios::ex09_produto_de_matrizes::produto_de_matrizes(&a, &b);
    println!("  A = {:?}", a);
    println!("  B = {:?}", b);
    println!("  A x B = {:?}", c);

    println!("\nEx 10 - Merge sort - O(n log n)");
    let lista = vec![38, 27, 43, 3, 9, 82, 10];
    println!("  Antes: {:?}", lista);
    let ordenada = exercicios::ex10_merge_sort::merge_sort(lista);
    println!("  Depois: {:?}", ordenada);

    println!("\n{}", "=".repeat(50));
    println!("Experimento concluido.");
}
