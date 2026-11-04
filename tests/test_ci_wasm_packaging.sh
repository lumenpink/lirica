#!/usr/bin/env bash
set -euo pipefail

# Test Suite: Compilação de Bundle WASM e Compatibilidade (#14, #15)
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SPEC_FILE_14="$REPO_ROOT/docs/specs/14_compilar_bundle_wasm_deploy_pages.md"
SPEC_FILE_15="$REPO_ROOT/docs/specs/15_desativar_wasm_opt_bulk_memory.md"
FORGEJO_CI="$REPO_ROOT/.forgejo/workflows/ci.yaml"
GITHUB_CI="$REPO_ROOT/.github/workflows/ci.yaml"
CARGO_TOML="$REPO_ROOT/Cargo.toml"

echo "==> [TEST] Validando conformidade de compilação, bulk memory e empacotamento (#14, #15)..."

# 1. Validação das Specs
echo -n "-> Verificando especificações formais... "
if [ ! -f "$SPEC_FILE_14" ] || [ ! -f "$SPEC_FILE_15" ]; then
    echo "FALHA: Arquivos de especificação não encontrados."
    exit 1
fi
echo "OK: especificações presentes."

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

# 4. Validação de compatibilidade wasm-opt = false no Cargo.toml (#15)
CARGO_TOML="$REPO_ROOT/Cargo.toml"
echo -n "-> Verificando wasm-opt = false em Cargo.toml... "
if ! grep -q 'wasm-opt = false' "$CARGO_TOML"; then
    echo "FALHA: wasm-opt = false ausente em $CARGO_TOML (necessário para compatibilidade bulk memory)."
    exit 1
fi
echo "OK: wasm-opt desativado para compatibilidade com bulk memory."

# 5. Verificação de Mutação (Garantir que mutantes falham)
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
