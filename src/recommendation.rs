use crate::graph::GrafoRecomendacao;
use crate::product::Produto;
use std::collections::{HashSet, VecDeque};

pub struct SistemaRecomendacao;

impl SistemaRecomendacao {

    // Executa o algoritmo de Busca em Largura (BFS) para sugerir produtos por proximidade.
    // id_origem: ID do produto inicial.
    // limite: Quantidade máxima K de recomendações retornadas.
    // Garante a prevenção de recomendações duplicadas utilizando HashSet.
    
    pub fn recomendar_bfs(grafo: &GrafoRecomendacao, id_origem: u32, limite: usize) -> Vec<Produto> {
        let mut recomendacoes = Vec::new();
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();

        // Evita auto-recomendação marcando o produto de origem como visitado
        visitados.insert(id_origem);
        fila.push_back(id_origem);

        while let Some(id_atual) = fila.pop_front() {
            if recomendacoes.len() >= limite {
                break;
            }

            if let Some(vizinhos) = grafo.adjacencias.get(&id_atual) {
                for &vizinho in vizinhos {
                    // retorna true apenas se o elemento for inédito
                    if visitados.insert(vizinho) {
                        fila.push_back(vizinho);

                        if let Some(prod) = grafo.obter_produto(vizinho) {
                            recomendacoes.push(prod.clone());
                            if recomendacoes.len() >= limite {
                                break;
                            }
                        }
                    }
                }
            }
        }

        recomendacoes
    }
}
