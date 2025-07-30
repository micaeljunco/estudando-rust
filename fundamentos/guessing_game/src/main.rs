// Importando o módulo de entrada/saída da biblioteca padrão
use std::io; // 'io' faz parte de 'std' (standard library)
use rand::Rng;

// A biblioteca padrão do Rust contém muitos itens já trazidos por padrão
// via o "prelude", mas nem tudo está automaticamente disponível. Por isso
// às vezes você precisa fazer 'use' explícito.

// Documentação oficial da biblioteca padrão:
// https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    println!("Adivinhe o número!");
    println!("Digite seu palpite: ");

    let numero_secreto = rand::thread_rng()
        .gen_range(1, 101);

    // Em Rust, variáveis são imutáveis por padrão.
    // Aqui usamos 'mut' para permitir que o valor da variável mude.
    // 'String::new()' cria uma nova string vazia.
    // OBS: '::' é o operador de caminho. Aqui acessa uma função associada do tipo String.
    let mut chute = String::new();

    // 'io::stdin()' retorna uma instância de entrada padrão (input do usuário).
    // '.read_line(&mut chute)' lê a linha digitada e armazena na variável.
    // O '&mut chute' passa uma referência mutável (evita cópias desnecessárias).
    // '.expect()' trata erros, encerrando o programa se algo der errado.
    io::stdin()
        .read_line(&mut chute)
        .expect("[ERRO]: Não foi possível ler a entrada.");

    // Imprime o valor digitado pelo usuário.
    // Obs: o uso de {chute} é interpolação com macro println!
    println!("Seu chute: {chute}");

}
