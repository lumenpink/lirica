# Especificação Técnica — Ticket #39

## Contexto & Motivação
A primeira versão da exportação de card poético (Ticket #35) estabeleceu a renderização vetorial SVG nativa em Rust e a exportação em PNG de alta resolução. No entanto, o uso prático revelou quatro lacunas fundamentais de usabilidade, fidelidade filológica e design de informação:

1. **Bug de Rolagem no Preview:** O container de pré-visualização no modal (`.export-preview-wrapper`) utilizava `align-items: center` com `max-height: 520px` e `overflow-y: auto`. Esse padrão de flexbox impede a rolagem para cima e para baixo em conteúdos que ultrapassam a altura visível, cortando cabeçalho e rodapé em poemas de mais de 4 versos.
2. **Desconexão com Critérios Métricos da Tela:** A função `gerar_card_svg` executava o motor com parâmetros fixos (`false, false, false, true`), ignorando as seleções ativas do usuário (`Regra de Castilho`, `Cesura na Pontuação`, `Diérese Crescente`). Isso gerava discrepâncias entre o que o usuário via no laboratório e o que era renderizado no card.
3. **Falta de Transparência Metodológica:** Um card compartilhado com o público não informava qual escola métrica foi empregada, gerando dúvidas para leitores e avaliadores.
4. **Legenda Ausente para Ícones dos Versos:** Cada verso exibe badges com ícones métricos (ex.: `10 ⚔️`, `11 🕊️`, `7 🎶`), mas leitores que recebem a imagem fora da plataforma não possuem a chave interpretativa desses glifos.
5. **Omissão do Diagnóstico e Laudo de Rimas:** O card exibia apenas as letras das rimas (`A`, `B`), omitindo o laudo qualitativo da estrofe (esquema, classificação morfológica *Rica/Pobre/Rara*, acústica e detecção de clichês).

---

## Decisões Técnicas & Arquitetura

### 1. Assinatura Expandida em Rust (`src/lib.rs`)
A função exportada via WASM passa a receber os parâmetros de configuração métrica:
```rust
#[wasm_bindgen]
pub fn gerar_card_svg(
    texto: &str,
    titulo: &str,
    autor: &str,
    tema: &str,
    estrofe_idx: i32,
    incluir_metricas: bool,
    pontuacao: bool,
    dierese: bool,
    castilho: bool,
    rima: bool,
) -> String
```
- A escansão é realizada com `escandir(texto, "visual", pontuacao, dierese, castilho, rima)`.

### 2. Mini-legenda Contextual de Ícones
- O motor identifica o conjunto único de pares `(icone, classificacao)` presentes estritamente nos versos exibidos no card.
- Uma linha tipográfica discreta é renderizada logo abaixo dos versos:
  `Ícones: ⚔️ Decassílabo Heróico • 🕊️ Decassílabo Sáfico • 🎶 Redondilha Maior`
- Garante total inteligibilidade do card de forma autônoma sem poluir o visual.

### 3. Laudo de Rimas Incorporado
- Para a estrofe ou poema exibido, o card incorpora:
  - Esquema estrófico (ex.: `Rimas: ABBA` ou `ABCBDB`).
  - Diagnóstico qualitativo do motor (ex.: `Rimas Ricas (verbo × substantivo)` ou `Rimas Consoantes / Perfeitas`).
  - Indicação de eventual clichê poético quando detectado.

### 4. Rodapé Metodológico com Critérios Ativos
- Adição de uma linha ou tag sutil no rodapé registrando as convenções ativas:
  - `Critérios: Castilho (átonas descartadas) • Cesura na Pontuação • Diérese Crescente` (ou equivalente sucinto).

### 5. Correção do Container de Preview (`site/style.css` e `site/app.js`)
- Em `.export-preview-wrapper`:
  - Substituição de `align-items: center` por `align-items: flex-start` com centralização horizontal via `margin: 0 auto`.
  - Permite rolagem fluida e completa de qualquer card (desde quartetos breves até sonetos completos de 14 versos ou décimas).
- Em `site/app.js`:
  - Passagem dos valores dos checkboxes `chkPontuacao.checked`, `chkDierese.checked`, `chkCastilho.checked`, `chkRima.checked` na chamada de `renderizarCardSVGAtual()`.

---

## Alternativas Descartadas

- *Alternativa A: Exibir uma tabela com todos os 12 ícones do sistema no rodapé.*
  - Descartada: causaria sobrecarga visual imensa em telas pequenas e cards de estrofe única. Exibir apenas os ícones utilizados preserva a elegância minimalista.
- *Alternativa B: Deixar os critérios métricos apenas na tela e não no card exportado.*
  - Descartada: quem visualiza uma imagem ou SVG em redes sociais ou artigos acadêmicos precisa saber se o decassílabo considerou ou não a última sílaba átona sob a regra de Castilho. A transparência metodológica é indispensável.
