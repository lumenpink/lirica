# Especificação — Ticket #42: Glifos SVG Embutidos de Rima, Unificação de Rima Indeterminada e Eliminação de Emojis

## 1. Contexto e Motivação
O Ticket #41 substituiu com sucesso os rótulos textuais longos dos versos por ícones simbólicos e purificou o rodapé do card poético. No entanto, o uso de emojis do sistema operacional (`✨`, `⚡`, `💎`, `💠`, `❓`, `🔥`) revelou limitações graves:
1. **Falha Crítica de Contraste:** Fontes de emoji de sistema têm paletas de cores fixas (geralmente amarelo suave e ciano claro) que não respondem à propriedade CSS/SVG `fill`. No tema pergaminho (`#eeddc2`) e no tema minimalista (`#ffffff`), o contraste despenca para ~1.3:1, violando as diretrizes WCAG e comprometendo a legibilidade na impressão e no canvas PNG.
2. **Inconsistência Visual Entre Plataformas:** Cada sistema operacional (iOS/macOS, Android, Linux, Windows) renderiza emojis com estilos, traços e proporções divergentes, quebrando o padrão editorial refinado do Lírica.
3. **Ambiguidade de "Rima Provável" no Card Exportado:** Em um documento ou imagem exportada estática, termos como "Rima Provável" (originados do heurístico conservador de substantivo/adjetivo) sem o painel interativo de morfologia geram incerteza no leitor. A unificação honesta e transparente em **"Rima Indeterminada (pelo motor automático)"** expressa com precisão os limites de um analisador estático.
4. **Presença Residual de Emojis na UI Web:** O mesmo vício visual se reproduzia na interface web (`site/`), onde cards de auditoria, badges de versos e cabeçalhos usavam emojis em vez de ícones vetoriais coesos.

O objetivo do Ticket #42 é eliminar categoricamente os emojis das rimas e da interface, substituindo-os por **glifos vetoriais SVG de alto contraste** embutidos no motor Rust e reutilizados na interface web.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Glifos Vetoriais Embutidos vs. Emojis de Sistema
- **Opção A (Descartada):** Continuar com emojis Unicode coloridos do sistema.
  - *Problema:* Cores inalteráveis, contraste inacessível (~1.3:1), perda de controle tipográfico e renderização imprevisível em canvas/PNG.
- **Opção B (Escolhida):** Desenhar caminhos SVG (`<path>`, `<polygon>`, `<circle>`) embutidos em grade uniforme (16×16), usando `fill="currentColor"` na web e `fill="{theme.rhyme_text}"` no card exportado.
  - *Racional:* Contraste WCAG AAA garantido em todos os três temas do card (`manuscrito`, `noturno`, `minimalista`), fidelidade vetorial absoluta na impressão e no canvas, além de total controle editorial.

### Decisão 2: Catálogo Canônico de Glifos Vetoriais de Rima
- **`ico-rima-rica`**: Estrela geométrica sólida de 5 pontas.
- **`ico-rima-pobre`**: Estrela clássica de 4 pontas / centelha editorial.
- **`ico-rima-toante`**: Raio angular estilizado de alto contraste.
- **`ico-rima-preciosa`**: Gema/diamante lapidado facetado.
- **`ico-rima-rara`**: Anel editorial (círculos concêntricos).
- **`ico-rima-indeterminada`**: Losango vazado com ponto central (unificando casos prováveis e indeterminados).
- **`ico-cliche`**: Chama/tocha lírica vetorial de alto contraste.

### Decisão 3: Unificação Semântica da Rima Indeterminada
- No card estático e na interface, quando o motor automático não puder provar inequivocamente a classe morfológica (casos `ProvavelRica`, `ProvavelPobre` ou `Indeterminada`), ambos convergem para a mesma representação visual (`#ico-rima-indeterminada`).
- O texto explicativo na legenda é formalizado como:
  **`Rima Indeterminada (pelo motor automático)`**.
- Isso elimina dubiedades para o leitor final e documenta com transparência a barreira sintática do analisador.

### Decisão 4: Purificação da Interface Web (`site/`)
- Inclusão de um sprite SVG oculto em `site/index.html` com os símbolos canônicos reutilizáveis via `<svg class="ico-svg"><use href="#..."></use></svg>`.
- Atualização em `site/app.js` para renderizar os glifos vetoriais nos pares de rima, badges de estrofe, modal de auditoria e alertas de clichê.
- Remoção de emojis anacrônicos ou monárquicos em `formIcon` (`👑`, `✨`, `🔨`, etc.), adotando identificadores tipográficos e neutros.

---

## 3. Matriz de Compatibilidade e Verificação
- **Exportação SVG (`gerar_card_svg`):** Deve conter os `<defs>` com os IDs dos ícones e os elementos `<use>` correspondentes posicionados nos badges e na legenda.
- **Renderização PNG via Canvas (`baixarCardPNG`):** A especificação SVG permite `<use href="#id">` dentro do mesmo documento SVG, sendo perfeitamente suportado por WebKit, Blink e Gecko durante a rasterização em Blob URL.
- **Testes Automatizados:** `tests/wasm_api.rs` deve verificar a presença dos elementos vetoriais SVG, a ausência de emojis nos badges e a legenda `Rima Indeterminada (pelo motor automático)`.
