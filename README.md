# ConectaStore — Sistema de Recomendação Baseado em Grafos

Projetado em Rust para a **MegaStore**, o ConectaStore utiliza **Grafos de Recomendação** e a técnica de **Busca em Largura (BFS)** para sugerir produtos relevantes aos consumidores com base em proximidade comportamental e histórico de conexões.

## Objetivo e Funcionamento do Sistema

O objetivo do sistema é substituir filtros estáticos por um modelo dinâmico baseado em grafos:

**Vértices:** Representam os produtos do catálogo.
  
**Arestas:** Representam as conexões de compras conjuntas, interesse ou afinidade entre os itens.
  
**Algoritmo (BFS):** A partir de um produto visualizado ou selecionado pelo cliente, o algoritmo de Busca em Largura percorre as conexões em camadas (níveis de proximidade) para retornar os produtos mais relacionados.

## Estrutura do Repositório

MegaStore-Estrutura_de_Dados/

├── src/

│   ├── main.rs

│   ├── lib.rs

│   ├── product.rs

│   ├── graph.rs

│   └── recommendation.rs

├── tests/

│   └── integration_tests.rs

├── Cargo.toml

└── README.md

## Tecnologias e Estruturas Utilizadas

**Linguagem:** Rust (Edition 2021)

**Cadastro de Produtos:** std::collections::HashMap<u32, Produto> — Acesso e busca direta em tempo constante O(1).

**Grafo de Adjacência:** HashMap<u32, Vec<u32>> — Representação por Lista de Adjacência com consumo de memória O(V + E).

**Navegação em Largura:** std::collections::VecDeque<u32> — Fila FIFO para exploração do grafo por camadas.

**Prevenção de Duplicatas:** std::collections::HashSet<u32> — Conjunto de visitados que impede auto-recomendações, itens duplicados e loops em ciclos do grafo.

Instruções para Compilação e Execução

Certifique-se de ter o Rust instalado em seu sistema.
Navegue até a pasta raiz do projeto (MegaStore-Estrutura_de_Dados):

cd "MegaStore-Estrutura_de_Dados"

Executar a Aplicação Principal
cargo run

Executar a Suíte de Testes Automatizados
cargo test

## Exemplo de Uso e Saída no Terminal

Ao executar cargo run, o sistema cadastra o catálogo inicial em memória, estabelece as arestas de conexão e executa a recomendação BFS a partir do Notebook Gamer Pro (ID 1):


ConectaStore — Sistema de Recomendacao MegaStore (Rust) 

 Cadastrando produtos no catálogo...
 Construindo as arestas de afinidade no Grafo...

 Gerando recomendações para 'Notebook Gamer Pro' (ID 1) [Limite K = 4]...

-----------------------------------------------------------
  PRODUTOS RECOMENDADOS (Busca em Largura - BFS):
-----------------------------------------------------------
  1. ID 2: Mouse Sem Fio Ergoclick | Categoria: Acessórios | R$ 150.00
  2. ID 3: Teclado Mecânico RGB | Categoria: Acessórios | R$ 350.00
  3. ID 4: Monitor UltraWide 144Hz | Categoria: Eletrônicos | R$ 1800.00
  4. ID 5: Headset 7.1 Surround | Categoria: Acessórios | R$ 290.00
-----------------------------------------------------------
  Tempo de execução do algoritmo: 28.6µs
===========================================================

## Arquitetura da Solução e Desempenho

**Complexidade de Tempo:** O(V + E), onde V é o número de vértices (produtos) e E o número de arestas (conexões) exploradas no percurso.

**Complexidade de Espaço:** O(V + E) para armazenamento da Lista de Adjacência e das estruturas auxiliares do BFS (VecDeque e HashSet).

**Desempenho:** Em testes com grafos em cadeia e cargas elevadas, a resposta do algoritmo BFS esteve na faixa de microssegundos, comprovando a escalabilidade do projeto em Rust sem depender de bancos de dados externos.
