# Dockerfile para Railway
FROM rust:1.75-slim AS builder

WORKDIR /app

# Copiar arquivos de dependências primeiro (cache layer)
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm src/main.rs

# Copiar código fonte
COPY . .

# Build da aplicação
RUN cargo build --release

# Imagem final minimalista
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binário compilado
COPY --from=builder /app/target/release/task-processor /app/task-processor

# Tornar executável
RUN chmod +x /app/task-processor

# Expor porta (Railway define PORT automaticamente)
EXPOSE 3000

# Variável de ambiente para porta
ENV PORT=3000

# Comando de start
CMD ["./task-processor"]

