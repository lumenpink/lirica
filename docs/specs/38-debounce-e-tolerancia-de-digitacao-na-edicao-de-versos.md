# Especificação Técnica — Ticket #38

## Contexto & Motivação
No laboratório métrico, a edição verso a verso (`renderAlignedRows` em [site/app.js](file:///home/lumen/src/lirica/site/app.js)) oferece inputs individuais para cada linha do poema com uma coluna gêmea à direita exibindo em tempo real as sílabas poéticas escandidas, badges de metro e rimas.

Originalmente, o ouvinte de eventos `input` de cada linha disparava `updateSingleRowLive()` de forma síncrona a cada tecla pressionada. Essa chamada remonta o poema completo, invoca o motor WebAssembly em Rust e recalcula todas as rimas e formas estróficas.
Em composições extensas (sonetos, décimas, galopes e folhetos de cordel com dezenas de versos), essa reescansão síncrona contínua gerava micro-congelamentos de tela (*UI jank*), atraso no cursor e sobrecarga desnecessária na thread principal do navegador durante a digitação rápida.

O objetivo do Ticket #38 é introduzir um temporizador de tolerância (*debounce* de 150ms) no ciclo de reescansão do editor, eliminando o engasgo e proporcionando uma experiência de escrita fluida sem abrir mão da precisão métrica.

---

## Decisões Técnicas & Arquitetura

### 1. Temporizador de Tolerância (`site/app.js`)
- Criar a função `agendarUpdateSingleRowLive()` com janela de 150ms:
  ```javascript
  let debounceUpdateRowTimer = null;

  function agendarUpdateSingleRowLive() {
    if (debounceUpdateRowTimer) {
      clearTimeout(debounceUpdateRowTimer);
    }
    debounceUpdateRowTimer = setTimeout(() => {
      updateSingleRowLive();
      debounceUpdateRowTimer = null;
    }, 150);
  }
  ```
- No ouvinte de `input` de cada input de verso (`renderAlignedRows`):
  - Substituir a chamada direta de `updateSingleRowLive()` por `agendarUpdateSingleRowLive()`.
- O salvamento assíncrono em `localStorage` permanece com seu próprio debounce de 400ms (`agendarSalvamentoRascunho()`), operando de forma harmônica e desacoplada da re-renderização visual.

### 2. Garantia de Sincronismo em Ações de Navegação
- No pressionamento de `Enter` (criação de nova linha):
  - Cancela o timer pendente e executa `updateSingleRowLive()` ou recria o workspace alinhado de forma imediata para posicionar o foco no novo verso sem atrasos.

---

## Alternativas Descartadas

- *Alternativa A: Escandir apenas o verso isolado sem reavaliar as rimas.*
  - Descartada: O esquema de rimas, os pares morfológicos e o diagnóstico da forma poética dependem da relação entre múltiplos versos. A escansão isolada deixaria a estrofe dessincronizada e produziria diagnósticos incoerentes.
- *Alternativa B: Debounce longo de 400ms a 600ms.*
  - Descartada: Criaria uma percepção incômoda de lentidão para o usuário, que esperaria quase meio segundo para ver as sílabas e badges reagirem após terminar uma palavra. 150ms é a janela ideal entre cadência humana de digitação e resposta instantânea do motor.
