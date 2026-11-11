# Especificação Técnica: Completar Corpus dos Poemas de Exemplo na Biblioteca WebAssembly

**Ticket:** #22  
**Branch:** `feat/22-completar-corpus-exemplos-wasm`  
**Escopo:** `src/lib.rs`, `tests/wasm_api.rs`

---

## 1. Contexto e Motivação

A função `carregar_exemplos()` em `src/lib.rs` fornece o acervo clássico de demonstração poética embutido no binário WebAssembly do Lírica.
Entretanto, quatro dos sete poemas foram cadastrados originalmente de forma incompleta, contendo apenas a primeira estrofe (quadra isolada de 4 versos):
1. **Canção do Exílio (Gonçalves Dias):** Possuía 4 versos, omitindo as 4 estrofes seguintes (total de 5 estrofes e 24 versos).
2. **Sinfonias do Ocaso (Cruz e Sousa):** Possuía uma quadra truncada, quando a obra original publicada em *Broquéis* (1893) é um célebre soneto simbolista de 14 versos decassílabos.
3. **Autopsicografia (Fernando Pessoa):** Possuía apenas a primeira estrofe (4 versos), faltando as estrofes 2 e 3 (total de 12 versos).
4. **Ouvir Estrelas (Olavo Bilac):** Possuía apenas a primeira quadra (4 versos), quando a composição em *Via Láctea* (XIII) é um soneto parnasiano clássico de 14 versos decassílabos.

---

## 2. Decisões Técnicas e Corpus Canônico

Para a Natasha entender as escolhas textuais:

1. **Fidelidade às Edições Críticas:**
   - Adotar o texto integral das edições consagradas em domínio público para cada uma das quatro obras poéticas.
   - Manter as quebras de estrofe canônicas com linhas em branco (`\n\n`) para correta segmentação pelo motor métrico.
2. **Impacto na Escansão e Análise de Rimas:**
   - Com poemas completos, a análise de estrofes (`AnaliseWeb`) poderá identificar a estrutura global da obra (ex.: sonetos petrarquianos/parnasianos com 14 versos e esquemas complexos de rimas ricas/pobres, oitavas e quartetos).
3. **Hermeticidade e Determinismo:**
   - Todo o corpus reside estaticamente no código Rust da biblioteca (`src/lib.rs`), sem requisições de rede ou dependências externas em tempo de execução.

---

## 3. Critérios de Aceite

1. `carregar_exemplos()` retorna JSON válido com os 7 poemas estruturados.
2. Cada um dos 4 poemas atualizados atinge a contagem integral de versos:
   - `goncalves_dias`: $\ge 20$ versos não-vazios em 5 estrofes.
   - `cruz_e_sousa`: 14 versos decassílabos em 4 estrofes (4+4+3+3).
   - `pessoa`: 12 versos heptassílabos em 3 estrofes (4+4+4).
   - `bilac`: 14 versos decassílabos em 4 estrofes (4+4+3+3).
3. Testes automatizados em `tests/wasm_api.rs` cobrindo a contagem e integridade do corpus.
