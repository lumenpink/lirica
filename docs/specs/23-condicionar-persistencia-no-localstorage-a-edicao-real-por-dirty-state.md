# Especificação Técnica: Condicionar Persistência no LocalStorage a Edição Real por Dirty State

**Ticket:** #23  
**Branch:** `fix/23-dirty-state-persistencia-localstorage`  
**Escopo:** `site/app.js`, `tests/wasm_api.rs`

---

## 1. Contexto e Diagnóstico

No ticket #20 e #21, implementou-se o salvamento de rascunhos poéticos no `localStorage`.
Entretanto, a função `carregarPoema(texto)` invocava `salvarRascunhoImediato(texto)` sempre que um cartão de exemplo era clicado na modal de exemplos.
Além disso, qualquer operação passiva de visualização gravava a chave `'lirica_draft_poem'`.

Esse comportamento gerava um efeito colateral adverso na experiência da pessoa usuária:
- Ao navegar e consultar exemplos na modal, o exemplo consultado era gravado como rascunho persistido no `localStorage`.
- Consequentemente, nas recargas subsequentes (`F5`), a chave `'lirica_draft_poem'` continha dados, o que **bloqueava o sorteio aleatório** de novos poemas no bootstrap.
- O aplicativo ficava artificialmente "preso" ao último exemplo inspecionado, exigindo que o usuário clicasse no botão "Limpar" para que a biblioteca voltasse a sortear.

---

## 2. Decisão Técnica: Máquina de Estados por *Dirty State*

Para a Natasha entender a mecânica de estados:

1. **Estado Inicial (Clean State / Pristine):**
   - O sorteio aleatório no bootstrap e o clique em exemplos na modal (`carregarPoema`) operam **estritamente em memória**.
   - Nenhuma chamada a `localStorage.setItem` é disparada ao apenas carregar ou visualizar um exemplo.
   - O estado da sessão permanece marcado como `isDirty = false`.
2. **Transição para Dirty State (Edição Ativa):**
   - No instante em que a pessoa usuária realiza uma alteração manual:
     - Digitação em qualquer verso (`input`);
     - Adição de novo verso ou nova estrofe (`btnAddVerso`, `btnAddEstrofe`);
     - Exclusão de verso ou estrofe (`btnDel`, Backspace em linha vazia);
     - Aplicação de texto customizado no modo livre (`btnApplyRaw`);
   - O estado passa para `isDirty = true` e o rascunho é persistido no `localStorage` (com debounce ou flush imediato).
3. **Comportamento no Reload (`F5`):**
   - Se o usuário apenas navegou nos exemplos sem editar nada (`isDirty === false`), não há rascunho salvo no `localStorage`. Ao recarregar a página, um novo exemplo é sorteado aleatoriamente da biblioteca.
   - Se o usuário editou qualquer verso (`isDirty === true`), o rascunho é preservado fielmente no `localStorage` e restaurado no reload.
4. **Comportamento no Reset / Limpar:**
   - Ao acionar `limparLaboratorio()`, o `localStorage` é limpo (`removeItem`), `isDirty` retorna para `false` e a esteira é esvaziada.

---

## 3. Critérios de Aceite

1. `carregarPoema()` NÃO deve invocar `salvarRascunhoImediato()`.
2. `sortearExemploAleatorio()` e a inicialização de exemplos no bootstrap NÃO devem disparar gravação no storage.
3. Eventos de digitação (`input`), adição de versos e `btnApplyRaw` devem marcar a sessão como dirty e gravar no `localStorage`.
4. Teste de contrato em `tests/wasm_api.rs` comprovando a ausência de chamadas a `salvarRascunhoImediato` dentro de `carregarPoema` e a presença do controle por dirty state.
