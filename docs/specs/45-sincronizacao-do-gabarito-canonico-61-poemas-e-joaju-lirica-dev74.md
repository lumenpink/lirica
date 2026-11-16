# Especificação Técnica: Sincronização do Gabarito Canônico (61 Peças) e Joaju-Lírica dev74

**Ticket:** #45  
**Branch:** `feat/45-sincronizacao-do-gabarito-canonico-61-poemas-e-joaju-lirica-dev74`  
**Escopo:** `lirica` (`Cargo.toml`, `site/gabarito.json`, `site/docs/gabarito.html`, `site/index.html`, `tests/`)  
**Status:** Concluído e Validado (Fases 1 a 8 - Quality Gate Aprovado)

---

## 1. Contexto e Motivação

Com a conclusão do Ticket #22 no motor `joaju-lirica`, o Gabarito Canônico foi expandido de 55 para 61 poemas auditados (573 versos e 106 mutações ativas), incorporando o *Romance do Pavão Misterioso* (cordel in-octavo), *Pajada Gaúcha*, *Galope à Beira-Mar*, poemas de alta tensão prosódica (Augusto dos Anjos e Cruz e Sousa) e a matriz de sentinelas fonotáticos.

O cliente web e interface WASM (`lirica`) precisa refletir essa ampliação tanto no motor computacional subjacente quanto na interface de consulta pública (`site/docs/gabarito.html` e `site/gabarito.json`).

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Sincronização Exata de `site/gabarito.json` vs. Endpoint Dinâmico
- **Opção A (Descartada):** Criar uma rota dinâmica ou exportar via WASM em tempo de execução.
  - *Problema:* Aumenta desnecessariamente o tamanho do binário WASM (`.wasm`) para carregar strings estáticas de 61 poemas e dezenas de mutações quando a página do gabarito é primariamente um catálogo consultivo estático servido pelo Cloudflare Pages.
- **Opção B (Escolhida):** Sincronizar o JSON estático `site/gabarito.json` diretamente do artefato gerador do `joaju-lirica` (`tests/corpus/gabarito_canonico.json`).
  - *Vantagem:* Mantém o binário WASM enxuto, permite navegação instantânea e busca local no cliente via `fetch('../gabarito.json')`, sem custo extra de memória ou parsing no WASM.

### Decisão 2: Atualização de Dependência do Motor
- Atualizar `Cargo.toml` para `joaju-lirica = "0.6.0-dev.74"`.
- Testar a compatibilidade da API WASM com a suíte de testes de integração (`tests/wasm_contract_tests.rs`).

### Decisão 3: Atualização Textual do Site e Metadados
- Atualizar `site/docs/gabarito.html` e `site/index.html` para unificar os números de referência (61 poemas, 573 versos e 106 mutações declarativas), removendo resquícios obsoletos ("51 poemas" e "55 obras").

---

## 3. Matriz de Validação
- `tests/wasm_contract_tests.rs`: Garantir que os contratos WASM e escansão de casos complexos (incluindo crases triplas e novos esquemas) continuem passando.
- `tests/gabarito_site_sync_tests.rs`: Novo teste para assegurar que `site/gabarito.json` tem exatamente 61 poemas, que nenhum campo obrigatório está nulo e que o total de mutações bate com o esperado (prevenindo que Natasha encontre dados defasados no futuro).
