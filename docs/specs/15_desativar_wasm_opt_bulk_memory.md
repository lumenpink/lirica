# Especificação Técnica: Desativação do wasm-opt para Compatibilidade com Bulk Memory

- **Ticket**: #15
- **Escopo**: Configuração de `wasm-opt = false` no perfil de release do `wasm-pack` no arquivo `Cargo.toml`.

## 1. Contexto e Motivação
A esteira de integração contínua executa `wasm-pack build --target web --out-dir site/pkg --release`. A partir das versões recentes do compilador Rust, a geração de código WebAssembly inclui instruções nativas de bulk memory (`memory.copy`, `memory.fill`).

O utilitário `wasm-opt` executado internamente pelo `wasm-pack` rejeita essas operações caso a flag `--enable-bulk-memory` não esteja habilitada em sua validação, disparando erro de compilação e abortando a esteira antes da publicação. A desativação de `wasm-opt` no metadado do pacote soluciona o conflito de forma homogênea, preservando a integridade do código gerado pelo `rustc` com `opt-level = "z"` e `lto = "fat"`.

## 2. Requisitos Técnicos
1. `Cargo.toml`:
   - Declarar a seção `[package.metadata.wasm-pack.profile.release]`.
   - Definir `wasm-opt = false`.
2. `tests/test_ci_wasm_packaging.sh`:
   - Incluir verificação de que `Cargo.toml` contém `wasm-opt = false` para prevenir regressões na esteira de compilação.
