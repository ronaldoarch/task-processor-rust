# 🚀 Guia de Deploy no Railway

## Pré-requisitos

- Conta no [Railway](https://railway.app)
- Repositório GitHub conectado

## Passo a Passo

### 1. Criar Projeto no Railway

1. Acesse [railway.app](https://railway.app)
2. Faça login com sua conta GitHub
3. Clique em **"New Project"**
4. Selecione **"Deploy from GitHub repo"**
5. Escolha o repositório `ronaldoarch/task-processor-rust`

### 2. Configuração Automática

O Railway detectará automaticamente:
- ✅ Linguagem: Rust
- ✅ Build command: `cargo build --release`
- ✅ Start command: `cargo run --release`
- ✅ Porta: Configurada via variável `PORT` (padrão: 3000)

### 3. Variáveis de Ambiente (Opcional)

O projeto funciona sem variáveis de ambiente, mas você pode configurar:

- `PORT`: Porta do servidor (padrão: 3000)
- `RUST_LOG`: Nível de log (ex: `debug`, `info`, `warn`)

### 4. Deploy

O Railway fará deploy automaticamente:
- ✅ A cada push para a branch `main`
- ✅ Build otimizado em modo release
- ✅ Healthcheck automático em `/api/health`

### 5. Acessar a Aplicação

Após o deploy:
1. Railway gerará uma URL pública (ex: `https://task-processor-rust-production.up.railway.app`)
2. Acesse a URL para ver a API funcionando
3. WebSocket estará disponível em `wss://sua-url/ws`

## Testando o Deploy

```bash
# Health check
curl https://sua-url.railway.app/api/health

# Criar tarefa
curl -X POST https://sua-url.railway.app/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name": "Teste Railway", "duration_ms": 2000, "priority": "high"}'

# Listar tarefas
curl https://sua-url.railway.app/api/tasks
```

## Troubleshooting

### Build falha
- Verifique se todas as dependências estão no `Cargo.toml`
- Railway usa Rust estável, certifique-se de compatibilidade

### Porta não configurada
- Railway define automaticamente a variável `PORT`
- O código já está configurado para usar essa variável

### Timeout no deploy
- Builds em Rust podem demorar alguns minutos
- Railway tem timeout de 15 minutos, suficiente para este projeto

## Monitoramento

Railway fornece:
- 📊 Logs em tempo real
- 📈 Métricas de uso (CPU, memória)
- 🔄 Deploy automático
- 🔔 Notificações de status

## Custom Domain (Opcional)

1. Vá em **Settings** > **Networking**
2. Adicione seu domínio personalizado
3. Configure DNS conforme instruções do Railway

---

🎉 **Pronto!** Seu projeto estará no ar em poucos minutos!

