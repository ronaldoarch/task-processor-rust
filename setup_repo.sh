#!/bin/bash

# Script para configurar o repositório no GitHub
# Execute este script após criar o repositório no GitHub

echo "🚀 Configurando repositório GitHub..."

# Nome do repositório (ajuste se necessário)
REPO_NAME="task-processor-rust"
GITHUB_USER=$(git config user.name | tr '[:upper:]' '[:lower:]' | tr ' ' '-')

echo "📝 Nome do repositório: $REPO_NAME"
echo "👤 Usuário GitHub: $GITHUB_USER"

# Verificar se já existe um remote
if git remote get-url origin > /dev/null 2>&1; then
    echo "⚠️  Remote 'origin' já existe. Removendo..."
    git remote remove origin
fi

# Adicionar remote (ajuste a URL conforme necessário)
echo "🔗 Adicione o remote manualmente com:"
echo "   git remote add origin https://github.com/$GITHUB_USER/$REPO_NAME.git"
echo ""
echo "Ou se preferir SSH:"
echo "   git remote add origin git@github.com:$GITHUB_USER/$REPO_NAME.git"
echo ""
echo "Depois execute:"
echo "   git branch -M main"
echo "   git push -u origin main"

