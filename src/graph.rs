use crate::product::Produto;
use std::collections::HashMap;

// Estrutura do Grafo de Recomendação baseado em Lista de Adjacência e HashMap
pub struct GrafoRecomendacao {
    // Cadastro rápido de produtos O(1): ID -> Produto
    pub produtos: HashMap<u32, Produto>,
    // Lista de Adjacência: ID -> Vetor de IDs dos produtos conectados
    pub adjacencias: HashMap<u32, Vec<u32>>,
}

impl GrafoRecomendacao {
    pub fn novo() -> Self {
        GrafoRecomendacao {
            produtos: HashMap::new(),
            adjacencias: HashMap::new(),
        }
    }

    // Cadastra ou atualiza um produto no sistema
    pub fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        self.produtos.insert(id, produto);
        self.adjacencias.entry(id).or_insert_with(Vec::new);
    }

    // Adiciona uma conexão bidirecional (aresta) entre dois produtos
    pub fn adicionar_conexao(&mut self, id1: u32, id2: u32) -> bool {
        if self.produtos.contains_key(&id1) && self.produtos.contains_key(&id2) {
            self.adjacencias.entry(id1).or_default().push(id2);
            self.adjacencias.entry(id2).or_default().push(id1);
            true
        } else {
            false
        }
    }

    // Consulta um produto pelo identificador.
    pub fn obter_produto(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    // Retorna o número total de vértices (produtos)
    pub fn quantidade_produtos(&self) -> usize {
        self.produtos.len()
    }
}