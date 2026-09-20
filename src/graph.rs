use crate::product::Produto;
use std::collections::HashMap;

// Estrutura do Grafo de Recomendação baseado em Lista de Adjacência e HashMap
pub struct GrafoRecomendacao {
    // Cadastro rápido de produtos O(1): ID -> Produto
    pub produtos: HashMap<u32, Produto>,
    // Lista de Adjacência: ID -> Vetor de IDs dos produtos conectados
    pub adjacencias: HashMap<u32, Vec<u32>>,
}


