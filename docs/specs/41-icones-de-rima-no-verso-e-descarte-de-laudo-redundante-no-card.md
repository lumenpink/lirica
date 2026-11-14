# Especificação — Ticket #41: Ícones de Rima no Verso e Descarte de Laudo Redundante no Card

## 1. Contexto e Motivação
A evolução do card poético vetorial (`gerar_card_svg`) no Ticket #40 inseriu diagnósticos individuais por extenso nos badges de cada verso (ex.: `A · Rica`, `B · Pobre`) e manteve um bloco descritivo com laudo acústico no final do card.

Essa abordagem gerou ruído visual excessivo:
1. **Badges de Rima Inflados:** Rótulos textuais ocupavam muito espaço horizontal na linha do verso, desequilibrando a composição tipográfica em relação ao badge métrico (`10 ⚔️`).
2. **Poluição no Rodapé:** A exibição de termos técnicos e acústicos genéricos ("Disposição Mista / Livre", "Consoante (Perfeita)") gerava confusão para leigos e contradizia a proposta estética limpa do card.

O objetivo do Ticket #41 é:
1. Simplificar o quadro/badge de rimas do verso para conter estritamente a **letra da rima e o ícone correspondente** (ex.: `A ✨`, `B ✦`, `A ⚡`, `A 💎`, `A ❓`, `A 🔥`).
2. Presumir consonância por padrão (tradição luso-brasileira), destacando com `⚡` apenas as rimas toantes (assonantes).
3. Eliminar categoricamente todo e qualquer bloco textual de laudo acústico ou disposição de estrofes no final do card.
4. Integrar a decodificação dos ícones métricos e rímicos em uma **única linha de Legenda**, incluindo exclusivamente os conceitos que aparecem no card atual.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Ícones Simbólicos vs. Rótulos Textuais no Badge
- **Opção A (Descartada):** Textos extensos como `A · Pobre`, `A · Provável`, `A · Indeterminada`.
  - *Problema:* Ocupam de 70px a 110px de largura, roubando espaço do texto lírico.
- **Opção B (Escolhida):** Badge compacto contendo apenas a letra e o glifo simbólico (ex.: `A ✨`, `B ✦`, `A ⚡`).
  - *Racional:* Largura fixa ou quase fixa (~38px a 46px), perfeita harmonia visual com o badge métrico de 54px.

### Decisão 2: Tabela Canônica de Ícones de Rima
- `✨` Rima Rica (classes gramaticais distintas)
- `✦` Rima Pobre (mesma classe gramatical)
- `⚡` Rima Toante (identidade acústica restrita às vogais)
- `💎` Rima Preciosa (esdrúxula/proparoxítona ou morfologia erudita)
- `💠` Rima Rara (terminação de frequência estatística reduzida)
- `❓` Rima Provável / Indeterminada (zona cinzenta de lookback sintático)
- `🔥` Clichê Histórico (par exaustivamente repetido na tradição)

### Decisão 3: Presunção de Consonância e Tratamento da Rima Toante
- Na tradição poética lusófona, toda rima é considerada consoante (perfeita) a menos que se trate de assonância (toante).
- Se a terminação ou o laudo acústico apontar rima toante (`Toante`), o verso recebe o ícone `⚡` e a legenda traz `⚡ Rima Toante`.
- Caso contrário, a rima é consoante e seu ícone reflete a qualidade morfológica (`✨`, `✦`, etc.).
- Se houver clichê histórico no par, o glifo `🔥` é adicionado ao badge (ex.: `A ✦ 🔥`).

### Decisão 4: Purificação do Rodapé do Card
- Removido todo e qualquer parágrafo ou laudo descritivo sobre rimas ao final do card.
- Mantida apenas a linha de Legenda:
  `Legenda: ⚔️ Decassílabo Heróico  •  ✨ Rima Rica  •  ✦ Rima Pobre`
  *(adicionando `⚡ Rima Toante` ou `🔥 Clichê Histórico` apenas se o poema tiver esses elementos).*
