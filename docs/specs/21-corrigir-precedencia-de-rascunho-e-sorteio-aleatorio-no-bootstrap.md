# Especificação Técnica: Precedência de Rascunho e Sorteio Aleatório no Bootstrap

**Ticket:** #21  
**Branch:** `fix/21-rascunho-precedencia-exemplo-aleatorio`  
**Escopo:** `site/app.js`, `docs/specs/21-corrigir-precedencia-de-rascunho-e-sorteio-aleatorio-no-bootstrap.md`, testes em `tests/wasm_api.rs`

---

## 1. Contexto e Diagnóstico do Defeito

Na implementação inicial do ticket #20, o rascunho salvo no `localStorage` era lido com sucesso na inicialização (`main()`) em `site/app.js`.
Entretanto, logo abaixo na rotina assíncrona de carga de exemplos do motor WebAssembly (`carregar_exemplos()`), a aplicação executava:

```javascript
const defaultEx = exemplos.find(ex => ex.id === 'camoes');
if (defaultEx) {
  verses = defaultEx.texto.split('\n');
}
```

Essa atribuição ocorria **sempre e incondicionalmente**, sobrescrevendo o array `verses` e descartando o rascunho recuperado segundos antes.
Como resultado, todo `F5` / reload forçava a restauração do soneto fixo de Luís de Camões.

Além disso, a abertura do laboratório poético limitava-se a apresentar sempre o mesmo poema decassílabo de Camões, ocultando da pessoa usuária a riqueza de formas já disponíveis no corpus do Lírica (sonetos shakespearianos, espinelas, septilhas de cordel, etc.).

---

## 2. Árvore de Decisão Técnica e Alternativas Descartadas

Para a Natasha entender as escolhas:

1. **Por que condicionar com `rascunhoSalvo === null` em vez de checar `verses.length`?**
   - *Alternativa descartada:* Checar `if (verses.length === 0)`.
   - *Motivo do descarte:* `verses` já nasce inicializado com um array de versos no escopo global para renderização estática inicial. Checar `rascunhoSalvo === null` explicita a intenção: apenas injeta um exemplo caso o storage do navegador não possua nenhum rascunho persistido.
2. **Por que sortear aleatoriamente entre os exemplos em vez de fixar Camões?**
   - *Decisão:* Se `rascunhoSalvo === null` e o catálogo de `exemplos` possuir itens (`exemplos.length > 0`), seleciona-se:
     ```javascript
     const idxAleatorio = Math.floor(Math.random() * exemplos.length);
     verses = exemplos[idxAleatorio].texto.split('\n');
     ```
   - *Motivo:* Expõe a versatilidade métrica do motor (redondilhas, decassílabos, alexandrinos, rimas ricas e esquemas diversos) a cada nova sessão limpa.
3. **E se o catálogo de exemplos falhar ao carregar?**
   - Se `exemplos` estiver vazio ou o WASM falhar, a aplicação faz fallback gracioso para os versos já declarados estaticamente no topo do arquivo.
4. **O sorteio interfere nas recargas de página?**
   - Não. Assim que a pessoa usuária digita qualquer coisa, o debounce grava o rascunho no `localStorage`. Em qualquer recarga futura, `rascunhoSalvo !== null`, logo o sorteio é ignorado e o texto do usuário é preservado com 100% de fidelidade.

---

## 3. Matriz de Precedência no Bootstrap (`main()`)

A precedência estrita na inicialização do aplicativo é:

```text
1. Prioridade 1 (Alta): sessionStorage.getItem('lirica_custom_poem')
   └── Se o usuário veio de um gabarito/link externo com payload temporário.

2. Prioridade 2 (Normal): localStorage.getItem('lirica_draft_poem')
   └── Se o usuário já editou algo em sessões anteriores (rascunho ativo).

3. Prioridade 3 (Default Dinâmico): Exemplos Aleatórios (Math.random)
   └── Se não houver rascunho e a biblioteca de exemplos estiver carregada.

4. Prioridade 4 (Fallback Estático): Soneto de Camões em memória
   └── Se o motor WASM não estiver disponível ou a biblioteca falhar.
```

---

## 4. Critérios de Aceite

1. **Persistência Confiável no Reload:**
   - Digitar qualquer texto e recarregar a página DEVE manter o texto digitado sem recair para Camões nem para nenhum exemplo aleatório.
2. **Sorteio no Primeiro Acesso:**
   - Em uma sessão sem `lirica_draft_poem`, o poema carregado no bootstrap DEVE vir de um sorteio aleatório entre os `exemplos` do motor.
3. **Cobertura Automatizada em Testes:**
   - Teste de contrato em `tests/wasm_api.rs` verificando a guarda `rascunhoSalvo === null` e a presença do sorteio aleatório (`Math.random`).
