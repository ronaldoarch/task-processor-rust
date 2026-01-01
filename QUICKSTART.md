# 🚀 Guia Rápido de Início

## Instalação do Rust

Se você ainda não tem Rust instalado:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ou visite: https://www.rust-lang.org/tools/install
```

Após instalar, reinicie o terminal ou execute:
```bash
source $HOME/.cargo/env
```

## Executando o Projeto

```bash
# 1. Navegue até o diretório do projeto
cd rust

# 2. Compile e execute
cargo run

# Ou em modo release (mais rápido)
cargo run --release
```

O servidor estará disponível em `http://localhost:3000`

## Testando a API

### Com cURL:

```bash
# Health check
curl http://localhost:3000/api/health

# Criar tarefa
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name": "Minha Tarefa", "duration_ms": 3000, "priority": "high"}'

# Listar tarefas
curl http://localhost:3000/api/tasks

# Estatísticas
curl http://localhost:3000/api/stats
```

### Com o Cliente de Exemplo:

Em outro terminal:
```bash
cargo run --example client_example
```

### Com o Dashboard Web:

1. Abra o arquivo `examples/websocket_client.html` no seu navegador
2. Clique em "Conectar WebSocket"
3. Crie algumas tarefas e veja as atualizações em tempo real!

## Executando Testes

```bash
# Testes unitários
cargo test

# Testes com output detalhado
cargo test -- --nocapture

# Testes de integração (requer servidor rodando em outro terminal)
cargo test --test integration_test
```

## Dicas

- O servidor processa tarefas automaticamente em background
- Tarefas com prioridade "high" são processadas primeiro
- WebSockets fornecem atualizações em tempo real
- Estatísticas são atualizadas automaticamente

## Troubleshooting

**Erro: "address already in use"**
- Altere a porta no `main.rs` (linha 63) ou pare o processo que está usando a porta 3000

**Erro: "cargo: command not found"**
- Instale Rust usando o comando acima ou visite https://rustup.rs

**WebSocket não conecta**
- Certifique-se de que o servidor está rodando
- Verifique se está usando `ws://` e não `http://` para WebSocket

