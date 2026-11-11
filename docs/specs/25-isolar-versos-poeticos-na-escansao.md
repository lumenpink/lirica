# Especificação: Isolar Versos Poéticos na Escansão (Ticket #25)

## 1. Contexto e Diagnóstico do Defeito
No Ticket #24, foram adicionados campos para Título e Autor sincronizados ao editor em modo livre no formato Markdown:
```markdown
# Título
## Autor

Corpo do poema...
```
Entretanto, as funções `renderWorkspace()`, `renderAlignedRows()` e `copiarAnalise()` em `site/app.js` passaram a chamar `escandir()` enviando `getFullText()`. Como o motor de escansão poética trata linhas vazias duplas como delimitadores de estrofe, as linhas de título e autor foram interpretadas como a Estrofe 0 do poema, contendo dois versos que não rimam entre si.

Consequências diretas:
1. O índice da primeira estrofe real do poema virou 1 em vez de 0 na saída analítica.
2. A renderização visual das estrofes em `site/app.js` buscava a análise morfológica e métrica comparando `morfStanza.estrofe_indice === stanzaIdx`. Para a primeira estrofe do poema (`stanzaIdx = 0`), os dados retornados pertenciam ao falso bloco de título/autor (com 0 pares rímicos).
3. Isso fez desaparecer completamente os badges de resumo rímico e morfológico no topo das estrofes reais.
4. Versos falsos e métricas artificiais foram atribuídos ao título e autor.

## 2. Decisão Arquitetural (Árvore de Decisão para a Natasha)
- **Alternativa A:** Fazer o motor `joaju-lirica` no backend ignorar sintaxe Markdown.
  *Descarte:* Mistura preocupações de formatação de UI com regras canônicas de métrica poética pura.
- **Alternativa B:** Filtrar apenas no frontend em `site/app.js`.
  *Descarte parcial:* Deixa a biblioteca WASM exposta a erros caso invocada externamente com texto contendo metadados Markdown.
- **Solução Adotada (Defensiva Dupla):**
  1. **Frontend (`site/app.js`):** A escansão métrica e a montagem das linhas alinhadas recebem estritamente `verses.join('\n')`. Metadados de cabeçalho (`# Título` e `## Autor`) são mantidos exclusivamente para persistência em `localStorage` e exportação/cópia de Markdown integral.
  2. **Core WASM (`src/lib.rs`):** A função de escansão e auditoria limpa e ignora linhas de cabeçalho Markdown (`# ` e `## `) antes de fatiar o texto em estrofes e versos poéticos, prevenindo poluição na API.
  3. **Restauração do Topo das Estrofes:** Com o índice de estrofe sincronizado na base 0 para os versos reais, os badges de rimas no cabeçalho de cada estrofe voltam a renderizar com acurácia total.

## 3. Critérios de Aceitação
- Chamar a escansão com texto contendo cabeçalhos `# Título` e `## Autor` não adiciona versos espúrios ao resultado.
- A contagem de versos e estrofes corresponde exatamente ao corpo lírico do poema.
- Em poemas de estrofe única (como a décima espinela de *Ninguém viva descurado*), os pares de rima são indexados na estrofe 0 e renderizados corretamente no topo da estrofe.
