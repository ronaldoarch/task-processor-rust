#!/bin/bash

# Script para enviar o código para o GitHub
# Execute este script APÓS criar o repositório no GitHub

echo "🚀 Enviando código para GitHub..."
echo ""

# Verificar se o remote existe
if ! git remote get-url origin > /dev/null 2>&1; then
    echo "❌ Remote 'origin' não encontrado. Adicionando..."
    git remote add origin https://github.com/ronaldoarch/task-processor-rust.git
else
    echo "✅ Remote 'origin' já configurado"
    git remote set-url origin https://github.com/ronaldoarch/task-processor-rust.git
fi

echo ""
echo "📤 Enviando código para GitHub..."
echo ""

# Garantir que estamos na branch main
git branch -M main

# Fazer push
if git push -u origin main; then
    echo ""
    echo "✅ Sucesso! Código enviado para:"
    echo "   https://github.com/ronaldoarch/task-processor-rust"
    echo ""
    echo "🎉 Repositório criado e configurado com sucesso!"
else
    echo ""
    echo "❌ Erro ao enviar. Possíveis causas:"
    echo "   1. Repositório ainda não foi criado no GitHub"
    echo "   2. Problemas de autenticação"
    echo ""
    echo "📝 Para criar o repositório:"
    echo "   1. Acesse: https://github.com/new"
    echo "   2. Nome: task-processor-rust"
    echo "   3. NÃO marque README/.gitignore/license"
    echo "   4. Clique em 'Create repository'"
    echo "   5. Execute este script novamente"
fi

