# Especificação Técnica — Ticket #44: Atualização do joaju-lirica para 0.6.0-dev.68 com Sinalefa Terciária

**Ticket:** #44  
**Branch:** `feat/44-joaju-lirica-0-6-0-dev-68-com-sinalefa-terciaria`  
**Escopo:** `lirica` (`Cargo.toml`, `tests/wasm_api.rs`, WebAssembly)  
**Status:** Em Especificação (Fase 3: Spec)

---

## 1. Contexto e Motivação

No Ticket #21 do motor `joaju-lirica`, implementou-se a resolução fonotática para sinalefa terciária por crase homófona através de clítico átono intermediário. Versos como *"O horizonte da serra a alvorecer"*, que antes quebravam em 11 sílabas por interrupção prematura da elisão, agora são corretamente unificados em 10 sílabas poéticas (`ra_a_al`), com tônicas em [3, 6, 10] (Decassílabo Martelo).

O objetivo do Ticket #44 é integrar o `joaju-lirica` `0.6.0-dev.68` ao crate `lirica`, garantindo que:
1. O laboratório web interativo e os cards exportados reflitam a escansão canônica perfeita.
2. Não haja regressão nos 37 testes existentes da API WebAssembly e contratos do frontend.
3. Adicione-se um teste explícito de contrato para o verso analisado.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Fonte da Verdade da Métrica
- **Opção A (Descartada):** Ajustes ad-hoc em `src/lib.rs` (manipulação de strings/regex antes de repassar à UI).
  - *Problema:* Viola o princípio de fonte única da verdade e acumula dívida técnica para a Natasha.
- **Opção B (Escolhida):** Atualização formal da dependência `joaju-lirica` via registro `gambi` para a versão `0.6.0-dev.68`.
  - *Racional:* O `lirica` atua exclusivamente como ponte WebAssembly e renderizador visual de alta fidelidade; toda a inteligência fonológica permanece estritamente encapsulada no `joaju-lirica`.

---

## 3. Matriz de Testes e Validação

1. **Teste de Escansão WebAssembly para Verso Canônico com Crase Tripla:**
   - `"O horizonte da serra a alvorecer"` deve retornar 10 sílabas métricas, classificação `"Decassílabo Martelo"` e sílaba `ra_a_al`.
2. **Integralidade dos 37 Testes Existentes:**
   - Camões, Vinicius, Gonçalves Dias, Bilac, Cruz e Sousa, Cordel, Espinela, Pajada Gaúcha e Galope à Beira-Mar devem manter 100% de assertividade.
3. **Quality Gate:**
   - `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --test wasm_api`.
