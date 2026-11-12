# Especificação Técnica — Ticket #29

## Contexto & Problema
No Ticket #26, foi introduzido o modal interativo de legendas (`#modal-legenda`) com suporte a fechamento pelo cabeçalho (`#btn-fechar-modal-legenda`) e pelo rodapé (`#btn-fechar-modal-legenda-bottom`).
Ao registrar os ouvintes de evento em `site/app.js`:
```javascript
if (btnFecharModalLegenda) btnFecharModalLegenda.addEventListener('click', fecharModalLegenda);
if (btnFecharModalLegendaBottom) btnFecharModalLegendaBottom.addEventListener('click', fecharModalLegendaBottom);
```
O identificador passado como callback no segundo botão foi `fecharModalLegendaBottom`, que não existe no escopo (o nome da função declarada é `fecharModalLegenda`).

Por ser executado durante o fluxo síncrono da função `main()`, o V8 lança imediatamente uma exceção:
```text
ReferenceError: fecharModalLegendaBottom is not defined
```
Esse erro interrompe a execução de `main()` antes de atingir:
1. `btnToggleRaw.addEventListener('click', toggleRawEditor);` (linha 432).
2. `renderWorkspace();` (linha 451).

Consequência:
- O botão **Modo Texto Livre** não responde ao clique.
- O workspace alinhado da escansão poética permanece vazio, não renderizando os versos na tela.

## Decisão Técnica & Arquitetura
1. **Correção do Callback em `site/app.js`**:
   - Ajustar o ouvinte de `btnFecharModalLegendaBottom` para referenciar a função correta `fecharModalLegenda`:
     ```javascript
     if (btnFecharModalLegendaBottom) btnFecharModalLegendaBottom.addEventListener('click', fecharModalLegenda);
     ```
2. **Defesa para Natasha e Prevenção de Regressão**:
   - Para que a Natasha não passe por outro cenário onde um erro de digitação de callback em `app.js` interrompa a inicialização da UI sem que os testes percebam, vamos implementar um teste de integridade em `tests/wasm_api.rs`.
   - O teste analisa estaticamente as chamadas `.addEventListener('...', <identificador>)` em `site/app.js` e valida que todos os identificadores de função passados diretamente existem como declarações `function <identificador>` no arquivo.

## Alternativas Descartadas
- *Alternativa A: Criar um alias `function fecharModalLegendaBottom() { fecharModalLegenda(); }`.*
  - Descartada: Adicionaria redundância e ruído desnecessário. A função canônica `fecharModalLegenda()` já realiza o fechamento completo.
- *Alternativa B: Testar apenas via inspeção manual no navegador.*
  - Descartada: O erro passou despercebido exatamente porque os testes anteriores não validavam a consistência dos símbolos de callback em `app.js`. Natasha precisa de garantias automatizadas.
