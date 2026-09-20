use megastore::{GrafoRecomendacao, Produto, SistemaRecomendacao};
use std::time::Instant;

#[test]
fn test_integracao_fluxo_completo() {
    let mut sistema = GrafoRecomendacao::novo();

    // Cadastrando 1.000 produtos
    for i in 1..=1000 {
        sistema.adicionar_produto(Produto::novo(
            i,
            &format!("Produto {}", i),
            "Geral",
            100.0,
        ));
    }

    // Criando conexões em cadeia: 1 <-> 2 <-> 3 <-> ... <-> 1000
    for i in 1..999 {
        sistema.adicionar_conexao(i, i + 1);
    }

    let inicio = Instant::now();
    let recs = SistemaRecomendacao::recomendar_bfs(&sistema, 1, 5);
    let tempo = inicio.elapsed();

    // Validações das recomendações usando .get()
    assert_eq!(recs.len(), 5);
    assert_eq!(recs.get(0).unwrap().id, 2); // 1º produto recomendado (ID 2)
    assert_eq!(recs.get(4).unwrap().id, 6); // 5º produto recomendado (ID 6)

    println!("Tempo para recomendação em grafo de 1.000 nós: {:?}", tempo);
}
