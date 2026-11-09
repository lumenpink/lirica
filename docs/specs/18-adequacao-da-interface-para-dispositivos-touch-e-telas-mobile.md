# Especificação Técnica: Adequação da Interface para Dispositivos Touch e Telas Mobile

**Ticket:** #18  
**Branch:** `refactor/18-adequacao-da-interface-para-dispositivos-touch-e-telas-mobile`  
**Escopo:** `site/style.css`, `site/index.html`, `site/app.js` e testes de responsividade/touch

---

## 1. Contexto e Diagnóstico de Problemas

A interface do Lírica em telas móveis (smartphones com viewports de $360\text{px}$ a $430\text{px}$ e tablets até $768\text{px}$) apresenta gargalos críticos de ergonomia:

1. **Dashboard Superior Rígido em 2 Colunas:**
   - `.poetic-analysis-dashboard` utiliza `grid-template-columns: 1fr 1fr` fixo, espremendo os cartões de Forma Poética e Avaliação de Rimas em colunas minúsculas de ~160px no celular.
2. **Inputs com Auto-Zoom Indesejado no iOS:**
   - `.verse-input` usa fonte de `0.9375rem` (15px). Em navegadores baseados em WebKit/iOS Safari, qualquer campo com fonte menor que 16px dispara um zoom invasivo automático no viewport ao receber foco, quebrando o layout da aplicação.
3. **Alvos de Toque Inadequados (Touch Targets < 44px):**
   - O botão de exclusão de verso (`.btn-del-verse`) e o de quebra de estrofe (`.btn-del-stanza`) são glifos minúsculos sem padding de toque seguro, gerando toques acidentais ou perda de clique.
   - Os controles de opções (`.toggle-control`) possuem caixas de marcação pequenas e espaçamento insuficiente para o polegar.
4. **Header e Barra de Ações Quebrando em Múltiplas Linhas:**
   - `.header-brand` e `.header-actions` colidem em telas estreitas, desalinhando o título da aplicação e os botões de ação.
   - `.actions-bar` fragmenta-se em blocos soltos sem agrupamento visual coeso.
5. **Esteira de Sílabas e Rolagem Tátil:**
   - As esteiras de sílabas (`.syllables-strip`) e os badges de metro/rima precisam de rolagem horizontal inercial suave (`-webkit-overflow-scrolling: touch`) com padding terminal para não truncar a última sílaba do verso.

---

## 2. Requisitos e Diretrizes de Design Responsivo

### 2.1. Breakpoints Canônicos
* **Mobile Pequeno / Normal:** `@media (max-width: 640px)`
* **Tablet / Telas Médias:** `@media (max-width: 860px)`

### 2.2. Regras de Estilo e Ergonomia
1. **Dashboard em Coluna Única no Mobile:**
   - Em `max-width: 768px`, o dashboard `.poetic-analysis-dashboard` converte-se em coluna única (`grid-template-columns: 1fr`), garantindo leitura ampla e confortável de cada diagnóstico.
2. **Font-Size de 16px para Inputs no Mobile:**
   - `.verse-input` e `.select-input` devem ter `font-size: 1rem` (16px) em `@media (max-width: 768px)` para anular o auto-zoom do iOS Safari.
3. **Alvos de Toque $\ge 44\times 44\text{px}$ (WCAG / Apple HIG / Material):**
   - `.btn-del-verse` e `.btn-del-stanza` ganham área de clique mínima de $44\times 44\text{px}$, centralizada e com feedback tátil no toque (`:active`).
   - `.header-btn`, `.btn-ghost` e `.toggle-control` ganham altura mínima de $40\text{--}44\text{px}$ e espaçamento vertical confortável.
4. **Header Flexível e Compacto:**
   - Em telas pequenas, `.header-brand` alinha-se verticalmente com a marca no topo e as ações logo abaixo em barra rolável ou distribuída em grade compacta de ícones com rótulos.
5. **Scroll Horizontal Inercial na Esteira de Sílabas:**
   - `.verse-col-right` ganha `overflow-x: auto`, scroll suave, barra de rolagem sutil e margem direita de respiro para que versos longos (alexandrinos e decassílabos) sejam navegáveis com o polegar.

---

## 3. Critérios de Aceite

1. O layout renderiza fluidamente de $320\text{px}$ a $1920\text{px}$ sem overflow horizontal global (`overflow-x: hidden` no `body`/`html`).
2. O dashboard superior exibe cartões em coluna única empilhada em celulares e tablets compactos.
3. Nenhum botão ou controle interativo possui alvo tátil inferior a $40\text{--}44\text{px}$ em viewport mobile.
4. O editor verso a verso opera sem zoom invasivo em navegadores mobile.
5. A suite de testes em `tests/` e a validação do CSS asseguram conformidade estrita com os critérios declarados.

---

## 4. Laudo de Fechamento (Fase 9: Close)

- **Status:** Concluído e Aprovado
- **Breakpoints Implementados:** `@media (max-width: 768px)` e `@media (max-width: 480px)`
- **Alvos de Toque:** Mínimo de $44\times 44\text{px}$ para exclusão de versos e estrofes e $40\text{--}44\text{px}$ para botões e toggles
- **Inputs Mobile:** Font-size padronizado em $1\text{rem}$ ($16\text{px}$) anulando auto-zoom no Safari iOS
- **Dashboard e Editor:** Colapso responsivo para coluna única com esteira de sílabas rolável inercialmente
- **Testes Automatizados:** 19/19 testes aprovados (incluindo `test_mobile_touch_css_spec`)

