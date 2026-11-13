# Especificação Técnica — Ticket #37

## Contexto & Motivação
A cesura é a pausa interna e divisória rítmica fundamental que fraciona versos longos (decassílabos e alexandrinos) em dois ou mais hemistíquios equilibrados. Ela governa a cadência da fala, o contraponto dramático e a respiração poética natural.

No Lírica, o motor WebAssembly já reconhece o metro e o padrão rítmico dos versos (Heroico, Sáfico, Alexandrino, etc.), mas não tornava visível a articulação interna dos hemistíquios. Além disso, uma implementação ingênua que simplesmente cravasse uma barra após um número fixo de sílabas métricas (como a 6ª) provocaria cortes intraverbais esdrúxulos em palavras paroxítonas (ex.: degolando *"arde"* em *"que_ar || de"* ou *"contentamento"* em *"con-ten-ta-men || to"*).

Natasha precisa de um sistema determinístico que calcule as cesuras reais no motor Rust, respeitando a integridade lexical dos vocábulos e os padrões canônicos da métrica lusófona, apresentando-as visualmente de forma clássica e responsiva.

## Decisões Técnicas & Arquitetura

### 1. Cálculo Determinístico no Backend Rust (`VersoFormatado.cesuras`)
O backend WebAssembly expõe em `VersoFormatado` o campo:
```rust
pub cesuras: Vec<usize>,
```
onde cada inteiro representa o índice 1-based da sílaba métrica imediatamente anterior à pausa cesural.

#### Regras de Identificação:
1. **Escopo Estrito de Versos Longos:**
   - Apenas decassílabos (10 sílabas métricas) e dodecassílabos / alexandrinos (12 sílabas métricas) admitem cesura medial. Versos curtos (redondilhas de 5 ou 7 sílabas) não possuem cesura.
2. **Inviolabilidade da Fronteira de Palavra (*Word Boundary*):**
   - A cesura só pode ser marcada após uma sílaba se o vocábulo contido nela **terminar ali**. Se a palavra continuar na sílaba seguinte, a cesura naquele ponto é cancelada para evitar cortes intraverbais.
3. **Decassílabo Heroico (6 + 4):**
   - Pausa ancorada na 6ª sílaba métrica tônica.
   - Se a palavra terminar na 6ª sílaba métrica: marca cesura no índice 6.
4. **Decassílabo Sáfico / Lírico (4 + 6 ou 4 + 4 + 2):**
   - Pausa ancorada na 4ª sílaba métrica tônica.
   - *Cesura Masculina / Oxítona:* se a palavra terminar na 4ª sílaba: marca cesura no índice 4.
   - *Cesura Feminina / Lírica Clássica:* se a palavra for paroxítona com tônica na 4ª e átona de fechamento na 5ª (como *"Amor é fogo"*): marca cesura no índice 5, dividindo o verso em dois hemistíquios perfeitos de 5 + 5 sílabas sem mutilar vocábulos.
5. **Alexandrino Clássico (6 + 6):**
   - Pausa no índice 6 (ou 7 em caso de átona feminina absorvida no 1º hemistíquio).
6. **Prioridade e Ancoragem por Pontuação Sintática:**
   - Pontuações internas (vírgula, ponto-e-vírgula, dois-pontos, travessão) adjacentes a uma tônica rítmica confirmam e ancoram a cesura.

### 2. Apresentação Visual na UI (`site/app.js` e `site/style.css`)
- **Tira de Sílabas (`.syllables-strip`):**
  - A sílaba métrica que fecha o hemistíquio recebe a classe `.com-cesura`.
  - Exibição de um separador sutil com a clássica barra dupla `‖` (U+2016) ou espaçamento estético de compasso (`.caesura-divider`), com cor atenuada (`var(--text-muted)` ou `var(--accent)` translúcido).
  - O sublinhado ciano de elisão (`border-bottom`) e o destaque dourado/laranja de tônica são 100% preservados sem conflito.
- **Card Contextual do Verso (`#modal-legenda`):**
  - O modal de metro informa a presença e a posição da cesura, exibindo os dois hemistíquios divididos por `‖`.
- **Garantia Mobile:**
  - O glifo e o espaçamento não geram quebra de linha em telas estreitas (320px–400px), respeitando a rolagem horizontal nativa do strip de sílabas.

## Alternativas Descartadas
- *Alternativa A: Usar sublinhado para indicar a cesura.*
  - Descartada: O sublinhado já é utilizado para indicar elisões e sinalefas (`.syl-pill.elidida`). Sobrepor outro sublinhado geraria colisão no CSS e confusão cognitiva imediata.
- *Alternativa B: Cortar palavras ao meio pela contagem fixa de sílabas.*
  - Descartada: Quebraria vocábulos como *"ar-de"* em *"que_ar ‖ de"*, produzindo aberrações métricas inaceitáveis na tradição literária lusófona.
- *Alternativa C: Calcular a cesura no JavaScript.*
  - Descartada: O JavaScript não possui o autômato silábico nem o rastreio de limites de vocábulos e elisões consolidadas. O cálculo no Rust garante integridade, testes unitários automatizados e facilidade de manutenção para Natasha.
