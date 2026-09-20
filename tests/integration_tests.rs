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



}