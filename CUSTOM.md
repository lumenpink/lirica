# CUSTOM.md — Sobrescritas e Particularidades de Lírica

Este documento registra regras específicas e decisões arquiteturais de Lírica.

## 1. WebAssembly Soberano
- Aplicação web compilada em WebAssembly utilizando `wasm-pack` e `wasm-bindgen`.
- Consome o motor poético `joaju-lirica` estritamente a partir do registro esparso soberano (`sparse+https://git.2lp.in/api/packages/lumen/cargo/`), eliminando dependências locais de caminho.
- Execução puramente client-side, zero chamadas de rede externas e tempo de resposta instantâneo.
