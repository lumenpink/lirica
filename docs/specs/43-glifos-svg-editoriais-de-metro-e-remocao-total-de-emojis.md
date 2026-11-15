# Especificação — Ticket #43: Glifos SVG Editoriais de Metro e Remoção Total de Emojis

## 1. Contexto e Motivação
No Ticket #42, eliminamos com sucesso os emojis coloridos do sistema nas avaliações de rima e cabeçalhos. Contudo, as marcações métricas dos versos (`v.icone`) continuaram exibindo emojis do sistema operacional (`⚔️`, `🏛️`, `🔨`, `📜`, `🌿`, `🍃`, `👑`, `⚜️`, `🎵`, `🌊`, `🕊️`, `🪶`), herdados da função `joaju_lirica::icone_metro`.

Essa persistência acarreta problemas editoriais sérios:
1. **Incompatibilidade Estética e Editorial:** Imagens cartunescas coloridas de espadas, coroas, templos e passarinhos contrastam de forma infantil com uma obra literária ou estudo métrico formal.
2. **Violação de Diretrizes de Vocabulário:** O uso da coroa (`👑`) no alexandrino e da flor-de-lis (`⚜️`) no dodecassílabo evoca simbolismos monárquicos e imperialistas que violam frontalmente as diretrizes do projeto.
3. **Instabilidade de Renderização:** Plataformas diferentes renderizam os emojis com cores, espaçamentos e linhas de base inconsistentes, prejudicando o alinhamento no card poético SVG e na interface alinhada.

O objetivo do Ticket #43 é erradicar **definitivamente todos os emojis** das marcações métricas dos versos no card exportado (`gerar_card_svg`), na interface web (`site/`) e na documentação (`site/docs/`), adotando **glifos vetoriais SVG de alto contraste**.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Glifos Vetoriais SVG Editoriais para Métrica
- **Opção A (Descartada):** Notação puramente tipográfica/numérica (ex.: `10h`, `12a`).
  - *Problema:* Reduziria excessivamente a identidade visual expressiva do Lírica.
- **Opção B (Escolhida):** Catálogo de glifos vetoriais SVG (grade 16×16, herança `currentColor` / `theme.badge_text`), associando síntese gráfica elegante à cadência métrica de cada verso.
  - *Racional:* Preserva a expressividade estética ("lumenzinho") garantindo simultaneamente rigor editorial, acessibilidade WCAG AAA e uniformidade vetorial absoluta.

### Decisão 2: Expurgando Símbolos Monárquicos e Alegóricos
- **Alexandrino (12s):** A coroa (`👑`) é categoricamente descartada. Em seu lugar, adota-se o portal de cesura simétrica `6 ‖ 6`, refletindo a estrutura arquitetônica hemistíquia do grande verso clássico.
- **Dodecassílabo (12s geral):** A flor-de-lis (`⚜️`) é descartada em favor de um hexágono geométrico facetado de doze vértices.
- **Decassílabo Heroico (6, 10):** Adaga lírica clássica em traço fino (ritmo marcial camoniano).
- **Decassílabo Sáfico (4, 8, 10):** Lira grega estilizada (cadência melódica de Safo).
- **Decassílabo Martelo (3, 6, 10):** Martelo galopeano geométrico (tradição nordestina).
- **Decassílabo Geral (10s):** Losango sólido clássico (`◆`).
- **Redondilhas (7s e 5s):** Círculo de compasso peninsular (`●`) e arco lírico (`▲`).
- **Octossílabo (8s):** Quadrado facetado (`■`).
- **Eneassílabo (9s):** Onda rítmica ternária (`≈`).
- **Hendecassílabo (11s):** Pluma ascendente geométrica (`◈`).
- **Forma Livre:** Círculo vazado neutro (`○`).

### Decisão 3: Integração no Motor de Cards (`src/lib.rs`)
- Mapeamento determinístico de `class_name` para o respectivo ID de glifo (`#ico-metro-...`).
- Inclusão dos caminhos dos 12 glifos métricos no `<defs>` do SVG gerado por `gerar_card_svg`.
- O badge métrico do verso desenha a contagem silábica seguida do glifo via `<use href="#ico-metro-..." fill="{theme.badge_text}" />`.
- A legenda dinâmica do rodapé decodifica os metros presentes desenhando o glifo vetorial seguido do nome da classificação métrica.

### Decisão 4: Purificação da Interface Web e Documentação
- O sprite SVG em `site/index.html` passa a conter tanto os glifos de rima quanto os 12 glifos métricos.
- Em `site/app.js`, o badge métrico renderiza `<svg class="ico-svg"><use href="#ico-metro-..."></use></svg>`.
- As páginas de documentação (`versos.html`, `ajuda.html`, etc.) substituem as tabelas de emojis por glifos vetoriais e tipografia editorial.

---

## 3. Matriz de Testes e Validação
- **Ausência Total de Emojis Métricos:** Testes automatizados em `tests/wasm_api.rs` garantindo que o SVG exportado não contém `⚔️`, `🏛️`, `🔨`, `📜`, `🌿`, `🍃`, `👑`, `⚜️`, `🎵`, `🌊`, `🕊️`, `🪶`.
- **Presença dos Glifos Vetoriais de Metro:** Verificação de `<defs>` e `<use href="#ico-metro-...">` nos badges métricos e na legenda dinâmica.
- **Compatibilidade nos 3 Temas:** Teste de renderização nos temas `manuscrito`, `noturno` e `minimalista`.
