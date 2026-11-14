# Especificação — Ticket #40: Tipagem Individual de Rimas no Verso e Legenda Dinâmica no Card

## 1. Contexto e Motivação
A exportação visual em SVG/PNG (`gerar_card_svg`) adicionada no Ticket #35 e refinada no Ticket #39 exibia um laudo condensado no rodapé contendo:
- O esquema de rimas em texto corrido (ex.: `Rimas: abbaabbacdecde`).
- Um resumo agregado das qualidades morfológicas (ex.: `Pobres & Ricas`).

Essa representação gerava dois problemas críticos:
1. **Redundância:** A sequência de letras (`abab...`) já está estampada nos badges laterais de cada verso, duplicando informação desnecessariamente.
2. **Falso Nivelamento Metodológico:** Dizer no rodapé que o poema possui "Pobres & Ricas" não informa a proporção nem a localização exata das rimas. Um poema com 13 rimas ricas e 1 pobre ficava com o mesmo rótulo de um poema com 13 pobres e 1 rica.

O objetivo do Ticket #40 é:
1. Mover o diagnóstico de qualidade morfológica diretamente para o badge lateral de cada verso rimado (ex.: `A · Rica`, `B · Pobre`, `A · Provável`, `B · Indeterminada`), com indicador de clichê (`🔥`) quando aplicável.
2. Eliminar do rodapé do card a linha redundante de letras (`Rimas: abab...`) e a agregação genérica ("Pobres & Ricas").
3. Construir a legenda do rodapé de forma estritamente contextual, listando apenas os tipos de rima e metros que efetivamente aparecem nos versos exibidos no card.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Associação Verso a Verso vs. Resumo no Rodapé
- **Opção A (Descartada):** Manter o badge do verso apenas com a letra e colocar uma tabela de correspondência no rodapé.
  - *Problema:* Ocupa muito espaço vertical no rodapé e exige que o leitor cruze informações distantes.
- **Opção B (Escolhida):** Integrar o tipo de rima no próprio badge lateral do verso: `LETRA · TIPO [🔥]`.
  - *Racional:* O leitor bate o olho no verso e sabe imediatamente o metro, o ritmo, o esquema e a qualidade gramatical da rima naquele ponto exato.

### Decisão 2: Mapeamento de Qualidades para Rótulos Acessíveis
A partir de `ParRimaWeb.qualidade` em `analise.rimas_morfologicas`:
- `"Rica"` $\rightarrow$ `"Rica"`
- `"Pobre"` $\rightarrow$ `"Pobre"`
- `"ProvavelRica"` ou `"ProvavelPobre"` $\rightarrow$ `"Provável"`
- `"Indeterminada"` $\rightarrow$ `"Indeterminada"`
- `"Preciosa"` $\rightarrow$ `"Preciosa"`
- `"Rara"` $\rightarrow$ `"Rara"`
- Se `par.eh_cliche == true`: sufixo com `🔥` (ex.: `A · Pobre 🔥`).
- Se o verso não participa de par avaliado: exibe apenas a letra da rima (ex.: `A`).

### Decisão 3: Dimensionamento e Posicionamento dos Badges no SVG
- O badge de metro (`v.total_metricas v.icone`) permanece com largura fixa (54px).
- O badge de rima passa a ter largura dinâmica baseada no tamanho do texto formatado:
  `let rima_width = (badge_txt.chars().count() * 7 + 16).max(28);`
- Posição horizontal:
  `let rx = badge_right_x - rima_width;`
  Em seguida, `badge_right_x -= rima_width + 6;` para acomodar o badge de metro sem sobreposição.

### Decisão 4: Legenda Contextual Dinâmica Estrita
No rodapé do card:
- A legenda só inclui os itens que **efetivamente aparecem** nos versos selecionados:
  - Metros presentes (ex.: `⚔️ Decassílabo Heróico`).
  - Tipos de rima presentes:
    - Se houver `Rica`: `✨ Rima Rica`
    - Se houver `Pobre`: `✦ Rima Pobre`
    - Se houver `Preciosa`: `💎 Rima Preciosa`
    - Se houver `Rara`: `💠 Rima Rara`
    - Se houver `Provável`: `❓ Rima Provável`
    - Se houver `Indeterminada`: `❓ Rima Indeterminada`
    - Se houver `🔥`: `🔥 Clichê Histórico`
- Se houver laudo acústico global (`pureza_acustica` ou `tipo_disposicao`), ele pode ser mantido de forma complementar se relevante, mas sem a repetição das letras do esquema.
