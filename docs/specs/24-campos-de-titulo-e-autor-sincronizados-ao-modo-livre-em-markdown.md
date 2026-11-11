# Especificação Técnica: Campos de Título e Autor Sincronizados ao Modo Livre em Markdown

**Ticket:** #24  
**Branch:** `feat/24-titulo-autor-markdown-modo-livre`  
**Escopo:** `site/index.html`, `site/style.css`, `site/app.js`, `tests/wasm_api.rs`

---

## 1. Contexto e Motivação

O Lírica operava até então apenas sobre uma sequência isolada de versos, sem identificação formal do título da obra ou da autoria.
Isso causava limitações:
1. Ao carregar exemplos da rica biblioteca (ex.: "Canção do Exílio", Gonçalves Dias), o usuário não via título e autor explicitamente no topo do espaço de trabalho.
2. No Modo Texto Livre (`raw-textarea`), não existia um formato simples e intuitivo para o usuário colar um poema completo contendo título e autor.
3. Não havia sincronização entre o editor livre e os metadados poéticos.

---

## 2. Decisões Técnicas e Arquitetura de Sincronização

Para a Natasha entender as escolhas:

1. **Por que Markdown clássico (`# Título` e `## Autor`) no Modo Livre?**
   - É a sintaxe mais universal, simples e amigável da web contemporânea.
   - Qualquer pessoa que copia um poema do Obsidian, GitHub, blogs ou editores de texto modernos já utiliza `#` para título e `##` para subtítulo/autor.
2. **Ignorar Cabeçalhos na Análise Métrica:**
   - O motor de escansão não pode escandir o título ou o autor como se fossem versos métricos da composição.
   - Linhas iniciando com `# ` e `## ` são extraídas como metadados e filtradas da esteira métrica de `verses`.
3. **Sincronização Bidirecional Transparente:**
   - **Visual -> Livre (`btnToggleRaw`):** Ao abrir o textarea livre, o título e autor atuais são inseridos automaticamente no topo formatados com `# ` e `## `.
   - **Livre -> Visual (`btnApplyRaw`):** Ao clicar em "Aplicar aos Versos", o parser faz regex/split:
     - Linha iniciada por `# ` vira o valor de `#input-titulo`.
     - Linha iniciada por `## ` vira o valor de `#input-autor`.
     - Linhas restantes (sem `#` e sem `%%`) tornam-se os versos poéticos.
4. **Persistência Unificada no LocalStorage (Retrocompatibilidade Total):**
   - A chave `'lirica_draft_poem'` armazena o texto em Markdown contendo `# ` e `## ` no topo (caso preenchidos).
   - Na restauração no bootstrap:
     - Se o texto salvo contiver `# ` ou `## `, eles preenchem os inputs e são removidos de `verses`.
     - Se o texto salvo não contiver cabeçalhos (rascunhos legados), todos os dados vão para `verses` sem quebra.
5. **Ergonomia Visual:**
   - Barra `.poem-meta-bar` sobre o workspace com inputs flexíveis, sem bordas pesadas, perfeitamente integrados à tipografia editorial do Lírica.

---

## 3. Critérios de Aceite

1. `site/index.html` deve conter os elementos `#input-titulo` e `#input-autor`.
2. `site/app.js` deve realizar a extração de `# ` e `## ` no `btnApplyRaw` e a geração desses prefixos no `btnToggleRaw`.
3. `carregarPoema()` deve preencher os inputs `#input-titulo` e `#input-autor` com os metadados do exemplo carregado.
4. `limparLaboratorio()` deve resetar os inputs `#input-titulo` e `#input-autor`.
5. Testes em `tests/wasm_api.rs` cobrindo a presença dos elementos no DOM, o parser Markdown e a persistência de metadados.
