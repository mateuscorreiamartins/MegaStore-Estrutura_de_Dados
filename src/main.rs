use megastore::{GrafoRecomendacao, Produto, SistemaRecomendacao};
use std::time::Instant;

fn main() {
    println!("===========================================================");
    println!("  ConectaStore — Sistema de Recomendacao MegaStore (Rust) ");
    println!("===========================================================\n");

    let mut sistema = GrafoRecomendacao::novo();

    // Cadastrando produtos
    println!("[15] Cadastrando produtos no catálogo...");
    sistema.adicionar_produto(Produto::novo(1, "Notebook Gamer Pro", "Eletrônicos", 4500.00));
    sistema.adicionar_produto(Produto::novo(2, "Mouse Sem Fio Ergoclick", "Acessórios", 150.00));
    sistema.adicionar_produto(Produto::novo(3, "Teclado Mecânico RGB", "Acessórios", 350.00));
    sistema.adicionar_produto(Produto::novo(4, "Monitor UltraWide 144Hz", "Eletrônicos", 1800.00));
    sistema.adicionar_produto(Produto::novo(5, "Headset 7.1 Surround", "Acessórios", 290.00));
    sistema.adicionar_produto(Produto::novo(6, "Cadeira Ergonômica Office", "Móveis", 1200.00));

    // Conectando produtos - simulando compras
    println!("[16] Construindo as arestas de afinidade no Grafo...");
    sistema.adicionar_conexao(1, 2); // Notebook <-> Mouse
    sistema.adicionar_conexao(1, 3); // Notebook <-> Teclado
    sistema.adicionar_conexao(2, 4); // Mouse <-> Monitor
    sistema.adicionar_conexao(3, 5); // Teclado <-> Headset
    sistema.adicionar_conexao(4, 6); // Monitor <-> Cadeira

    // Teste de Desempenho e Execução do BFS
    let id_busca = 1;
    let limite_k = 4;
    println!("\n[17] Gerando recomendações para 'Notebook Gamer Pro' (ID {}) [Limite K = {}]...", id_busca, limite_k);


}
