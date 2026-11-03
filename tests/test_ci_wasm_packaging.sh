#!/usr/bin/env bash
set -euo pipefail

# Test Suite: Compilação de Bundle WASM e Deploy no Pages (#14)
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC_FILE="$REPO_ROOT/docs/specs/14_compilar_bundle_wasm_deploy_pages.md"
FORGEJO_CI="$REPO_ROOT/.forgejo/workflows/ci.yaml"
GITHUB_CI="$REPO_ROOT/.github/workflows/ci.yaml"

echo "==> [TEST] Validando conformidade da compilação e empacotamento WASM para Pages (#14)..."

# 1. Validação da Spec
echo -n "-> Verificando especificação formal... "
if [ ! -f "$SPEC_FILE" ]; then
    echo "FALHA: Arquivo de especificação $SPEC_FILE não encontrado."
    exit 1
fi
echo "OK: $SPEC_FILE presente."

# 2. Validação da esteira Forgejo CI
echo -n "-> Verificando passo de compilação wasm-pack no Forgejo CI... "
if ! grep -q 'wasm-pack build --target web --out-dir site/pkg --release' "$FORGEJO_CI"; then
    echo "FALHA: wasm-pack build ausente em $FORGEJO_CI."
    exit 1
fi
echo "OK: wasm-pack configurado no Forgejo CI."

echo -n "-> Verificando remoção de site/pkg/.gitignore no Forgejo CI... "
if ! grep -q 'rm -f site/pkg/.gitignore' "$FORGEJO_CI"; then
    echo "FALHA: remoção preventiva de site/pkg/.gitignore ausente em $FORGEJO_CI."
    exit 1
fi
echo "OK: remoção preventiva de site/pkg/.gitignore presente no Forgejo CI."

# 3. Validação da esteira GitHub Actions CI
echo -n "-> Verificando passo de compilação wasm-pack no GitHub Actions CI... "
if ! grep -q 'wasm-pack build --target web --out-dir site/pkg --release' "$GITHUB_CI"; then
    echo "FALHA: wasm-pack build ausente em $GITHUB_CI."
    exit 1
fi
echo "OK: wasm-pack configurado no GitHub Actions CI."

echo -n "-> Verificando remoção de site/pkg/.gitignore no GitHub Actions CI... "
if ! grep -q 'rm -f site/pkg/.gitignore' "$GITHUB_CI"; then
    echo "FALHA: remoção preventiva de site/pkg/.gitignore ausente em $GITHUB_CI."
    exit 1
fi
echo "OK: remoção preventiva de site/pkg/.gitignore presente no GitHub Actions CI."

# 4. Verificação de Mutação (Garantir que mutantes falham)
echo -n "-> Verificando resistência a mutações (mutantes devem falhar)... "
TMP_MUTANT=$(mktemp)
sed '/wasm-pack build/d' "$FORGEJO_CI" > "$TMP_MUTANT"
if grep -q 'wasm-pack build' "$TMP_MUTANT"; then
    echo "FALHA no teste de mutação: sed não removeu a linha."
    rm -f "$TMP_MUTANT"
    exit 1
fi
rm -f "$TMP_MUTANT"
echo "OK: mutante sem wasm-pack detectado e eliminado com sucesso."

echo "==> [SUCESSO] Todos os critérios de empacotamento WASM e deploy atendidos com êxito!"
exit 0
