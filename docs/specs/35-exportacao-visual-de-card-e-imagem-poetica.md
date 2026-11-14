# Especificação Técnica — Ticket #35

## Contexto & Motivação
O Lírica permite a análise aprofundada de poemas e formas estróficas em tempo real, mas a partilha visual do trabalho poético estava limitada à cópia de texto bruto ou visualização na tela.

O objetivo do Ticket #35 é fornecer uma ferramenta de exportação estética e compartilhamento para poetas, professores, estudantes e entusiastas:
1. **Card Poético Vetorial (SVG nativo):** Geração determinística e elegante via Rust/WASM, incorporando tipografia clássica, notação escandida com sílabas poéticas, tônicas, cesuras (`‖`), esquema de rimas e diagnóstico estrófico.
2. **Download e Cópia em Alta Resolução (PNG):** Renderização direta do SVG em `<canvas>` offscreen ($2\times$ pixel density para telas retina) com botões para baixar PNG ou copiar a imagem diretamente para a área de transferência.
3. **Exportação de Documento Typst Completo (`.typ`):** Geração de arquivo Typst pronto para compilação editorial acadêmica e editorial.
4. **Três Temas Visuais:**
   - *Manuscrito (Pergaminho):* Fundo creme/marfim suave, texto sépia, destaques em vinho clássico e dourado envelhecido.
   - *Noturno (Lírica Dark):* Paleta escura nativa da aplicação, texto claro com acentos esmeralda/dourado.
   - *Minimalista:* Preto e branco tipográfico editorial, ideal para publicação impressa ou monocromática.

---

## Decisões Técnicas & Arquitetura

### 1. Motor Gerador de SVG em Rust (`src/lib.rs`)
Para que o processo seja 100% reprodutível, auditável e testável pela Natasha através de `cargo test`, o SVG é construído diretamente em Rust:
- Função exportada via WebAssembly:
  `gerar_card_svg(texto: &str, titulo: &str, autor: &str, tema: &str, estrofe_idx: i32, incluir_metricas: bool) -> String`
- Elementos do SVG:
  - Fundo com borda sutil e padding generoso.
  - Cabeçalho: Título e Autor com tipografia serifada.
  - Estrofes e Versos: Versos escandidos com sílabas poéticas, tônicas em negrito/destaque, cesuras canônicas `‖` e badges alinhados de metro e rima à direita.
  - Rodapé: Forma poética identificada (ex.: *Soneto Clássico*, *Pajada Gaúcha*) e crédito discreto `Lírica • lirica.2lp.in`.
- Todo o CSS e tipografia são embutidos na tag `<style>` dentro do SVG para garantir que a renderização offscreen em `<canvas>` não sofra perda de folhas de estilo externas.

### 2. Conversão e Exportação no Frontend (`site/app.js`)
- Modal de Exportação `#modal-export`:
  - Visualização prévia interativa do SVG ao vivo.
  - Seletores reativos: Tema (Manuscrito / Noturno / Minimalista), Escopo (Poema Completo ou Estrofe específica), e alternador de Métrica silábica.
  - Ações:
    - **Baixar PNG:** Desenha o SVG em um `<canvas>` offscreen dimensionado com escala $2\times$ e dispara download de `card-lirica.png`.
    - **Baixar SVG:** Gera um Blob `image/svg+xml` e dispara download de `card-lirica.svg`.
    - **Copiar Imagem:** Converte o canvas em `blob` PNG e grava na área de transferência com `navigator.clipboard.write([new ClipboardItem({'image/png': blob})])`.
    - **Exportar Typst:** Gera arquivo `.typ` formatado para download.

### 3. Integração na Interface (`site/index.html` e `site/style.css`)
- Adição de botão de atalho de exportação na barra de ações superiores (`#btn-exportar-card`).
- Diálogo modal acessível com suporte a fechar por `Escape`, clique no backdrop ou botão de fechar.

---

## Alternativas Descartadas

1. **Uso de bibliotecas externas (html2canvas, dom-to-image, rasterizeHTML):**
   - *Descartada:* Essas bibliotecas injetam centenas de kilobytes de dívida técnica, dependem de renderização DOM frágil, quebram em navegadores móveis e não lidam corretamente com tipografia e sombras vetoriais.
2. **Geração do SVG puramente em JavaScript:**
   - *Descartada:* Impediria testes unitários e testes de mutação automatizados no pipeline nativo em Rust (`tests/wasm_api.rs`), deixando a Natasha desprotegida contra regressões na renderização do card.
