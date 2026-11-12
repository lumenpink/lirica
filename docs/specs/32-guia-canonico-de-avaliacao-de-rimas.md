# Especificação Técnica — Ticket #32

## Contexto & Motivação
O laboratório Lírica e o motor WebAssembly avaliam rimas em tempo real, calculando correspondências fonéticas, classes morfológicas e graus epistêmicos (Rica, Pobre, Preciosa, Rara, Provável e Indeterminada), além de detectar clichês históricos líricos.
Embora as seções de documentação cubram o motor geral (`regras.html`), as formas (`formas.html`) e os metros (`versos.html`), não existia uma página dedicada exclusivamente a desmistificar e documentar a avaliação de rimas para humanos.

Natasha precisa de uma referência canônica estável e didática que explique a fundamentação teórica lusófona (Antônio Feliciano de Castilho, Said Ali, Olavo Bilac, Segismundo Spina), o processo determinístico de decisão passo a passo, as salvaguardas conservadoras contra falsos positivos e a taxonomia de clichês poéticos.

## Decisões Técnicas & Arquitetura
1. **Nova Página Canônica `site/docs/rimas.html`**:
   - Desenvolvida com a identidade visual canônica de `site/docs/` (`docs-nav-bar`, `doc-section`, `badge-rima`, etc.).
   - Estruturada em 6 seções pedagógicas claras:
     1. **Fundamentos e Teoria Lusófona da Rima:** Distinção acústica entre rima consoante (perfeita) e toante (assonante), papel da tonicidade e tradição poética.
     2. **Categorias Morfológicas de Rima:**
        - *Rima Pobre:* Mesma classe gramatical (substantivo × substantivo, verbo × verbo).
        - *Rima Rica:* Classes gramaticais distintas (substantivo × adjetivo, verbo × substantivo).
        - *Rima Preciosa:* Assimetria de clíticos (*vê-la × estrela*), proparoxítonas ou combinações raras.
        - *Rima Rara:* Terminações de escassez fonológica extrema no léxico português.
     3. **A Árvore de Decisão do Algoritmo:** Passo a passo sequencial da avaliação (verificação de clíticos $\rightarrow$ escassez fonológica $\rightarrow$ disjunção de classes $\rightarrow$ identidade $\rightarrow$ hipóteses contextuais $\rightarrow$ indeterminação de salvaguarda).
     4. **Graus Epistêmicos & Postura Conservadora:** Explicação de por que a dúvida resulta em *Provável* ou *Indeterminado* (convergência nominal, particípios verbais) para nunca rotular incorretamente.
     5. **Detector de Clichês Históricos:** Explicação de pares hiper-saturados (*amor × dor*, *coração × paixão*) e sua função pedagógica sem desqualificar o metro do poema.
     6. **Glossário Rímico Rápido:** Termos essenciais para consulta ágil.

2. **Integração no Portal de Documentação (`site/docs/index.html`) e Navegação**:
   - Adicionar o card "🎯 Avaliação de Rimas" no portal `site/docs/index.html`.
   - Adicionar a aba `🎯 Avaliação de Rimas` na barra de navegação de todas as páginas de documentação (`site/docs/*.html`).
   - Adicionar link de referência no modal contextual e em `site/ajuda.html`.

## Alternativas Descartadas
- *Alternativa A: Inserir a explicação sobre rimas como um apêndice longo dentro de `regras.html`.*
  - Descartada: `regras.html` trata estritamente da escansão silábica (sinalefa, Castilho, hiato, diérese). Misturar morfologia e acústica de rimas ali causaria sobrecarga cognitiva e dificultaria a manutenção por Natasha.
- *Alternativa B: Deixar a explicação apenas dentro dos modais da aplicação.*
  - Descartada: O modal é contextual e resumido; leitores, pesquisadores e estudantes precisam de um ensaio canônico aprofundado com exemplos literários completos.
