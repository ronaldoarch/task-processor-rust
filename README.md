# 🦀 Task Processor - Sistema de Processamento Assíncrono em Rust

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Um sistema de processamento de tarefas assíncrono de alto desempenho construído com Rust, demonstrando conceitos avançados como concorrência, WebSockets, e arquitetura de microserviços.

## 🎯 Sobre o Projeto

Este projeto foi desenvolvido como demonstração de habilidades avançadas em Rust, incluindo:
- Programação assíncrona com Tokio
- Design de APIs RESTful
- WebSockets para comunicação em tempo real
- Processamento paralelo e concorrência
- Arquitetura de sistemas escaláveis

## ✨ Características

- **API REST Completa**: Endpoints para criar, listar, buscar e cancelar tarefas
- **WebSockets em Tempo Real**: Atualizações instantâneas sobre o status das tarefas
- **Processamento Paralelo**: Execução simultânea de múltiplas tarefas com sistema de prioridades
- **Estatísticas em Tempo Real**: Métricas detalhadas sobre o processamento
- **Arquitetura Assíncrona**: Construído com Tokio para máxima performance
- **Type Safety**: Aproveitamento completo do sistema de tipos do Rust
- **Error Handling Robusto**: Tratamento de erros usando `anyhow` e `thiserror`

## 🚀 Tecnologias Utilizadas

- **Tokio**: Runtime assíncrono de alta performance
- **Axum**: Framework web moderno e ergonômico
- **Serde**: Serialização/deserialização eficiente
- **WebSockets**: Comunicação bidirecional em tempo real
- **Channels**: Comunicação entre threads assíncronas
- **UUID**: Identificadores únicos para tarefas

## 📋 Pré-requisitos

- Rust 1.70+ ([instalação](https://www.rust-lang.org/tools/install))
- Cargo (incluído com Rust)

## 🛠️ Instalação e Execução

```bash
# Clonar o repositório
git clone <seu-repositorio>
cd rust

# Compilar o projeto
cargo build --release

# Executar o servidor
cargo run

# Ou executar em modo release para melhor performance
cargo run --release
```

O servidor estará disponível em `http://localhost:3000`

## 📚 API Endpoints

### Health Check
```bash
GET /api/health
```

### Criar Tarefa
```bash
POST /api/tasks
Content-Type: application/json

{
    "name": "Processar dados",
    "duration_ms": 5000,
    "priority": "high"  # "low", "medium", "high"
}
```

### Listar Todas as Tarefas
```bash
GET /api/tasks
```

### Obter Tarefa Específica
```bash
GET /api/tasks/{task_id}
```

### Cancelar Tarefa
```bash
POST /api/tasks/{task_id}/cancel
```

### Estatísticas do Sistema
```bash
GET /api/stats
```

### WebSocket para Atualizações em Tempo Real
```bash
ws://localhost:3000/ws
```

## 💡 Exemplos de Uso

### Criar uma tarefa com cURL

```bash
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Processar arquivo CSV",
    "duration_ms": 3000,
    "priority": "high"
  }'
```

### Conectar via WebSocket (JavaScript)

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Atualização de tarefa:', data);
};
```

### Usar com Python

```python
import requests
import json

# Criar tarefa
response = requests.post(
    'http://localhost:3000/api/tasks',
    json={
        'name': 'Processar dados',
        'duration_ms': 5000,
        'priority': 'high'
    }
)

task = response.json()
print(f"Tarefa criada: {task['id']}")

# Obter estatísticas
stats = requests.get('http://localhost:3000/api/stats').json()
print(f"Total de tarefas: {stats['total_tasks']}")
```

## 🧪 Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com output detalhado
cargo test -- --nocapture

# Executar testes de integração (requer servidor rodando)
cargo test --test integration_test
```

## 🏗️ Arquitetura

O projeto demonstra vários conceitos importantes de Rust:

1. **Ownership e Borrowing**: Uso eficiente de referências e ownership
2. **Async/Await**: Programação assíncrona com Tokio
3. **Channels**: Comunicação entre threads usando `broadcast` channels
4. **Arc e RwLock**: Compartilhamento seguro de estado entre threads
5. **Error Handling**: Uso de `Result` e tipos de erro customizados
6. **Pattern Matching**: Uso extensivo de `match` e `if let`
7. **Traits**: Implementação de `Default` e uso de traits do stdlib

## 📊 Métricas e Performance

- **Throughput**: Processa centenas de tarefas por segundo
- **Latência**: Resposta em milissegundos
- **Concorrência**: Suporta milhares de conexões simultâneas
- **Memória**: Uso eficiente de memória com zero-cost abstractions

## 🎯 Destaques Técnicos

Este projeto demonstra:

- ✅ Concorrência e paralelismo em Rust
- ✅ Programação assíncrona avançada
- ✅ Design de APIs RESTful
- ✅ WebSockets para comunicação em tempo real
- ✅ Tratamento robusto de erros
- ✅ Testes unitários e de integração
- ✅ Documentação completa
- ✅ Código idiomático em Rust

## 📝 Estrutura do Projeto

```
rust/
├── src/
│   ├── main.rs          # Ponto de entrada e configuração do servidor
│   ├── models.rs         # Modelos de dados e estruturas
│   ├── processor.rs      # Lógica de processamento de tarefas
│   ├── api/
│   │   ├── mod.rs
│   │   └── handlers.rs   # Handlers da API REST
│   └── websocket.rs      # Handler WebSocket
├── tests/
│   └── integration_test.rs  # Testes de integração
├── Cargo.toml           # Dependências e configuração
└── README.md            # Documentação
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

## 📄 Licença

Este projeto está sob a licença MIT.

## 👨‍💻 Autor

Criado como projeto de portfólio para demonstrar habilidades em Rust e desenvolvimento de sistemas assíncronos.

---

**Nota**: Este projeto foi desenvolvido para demonstrar conhecimento avançado em Rust, incluindo programação assíncrona, concorrência, e design de APIs. É um excelente exemplo para portfólios técnicos!

