# Especificação Técnica: Visualização Expansível de Mutações no Gabarito Canônico

**Ticket:** #17  
**Branch:** `feat/17-visualizacao-expansivel-de-mutacoes-e-corpus-exaustivo`  
**Escopo:** Documentação pública interativa, sincronização de corpus e testes WASM

---

## 1. Visão Geral

Este ciclo integra a nova arquitetura de **Mutações Declarativas por Deltas** do `joaju-lirica` à documentação interativa e interface do `lirica`:
1. Sincronização do arquivo de dados `site/gabarito.json` com os 55 poemas canônicos puros e suas 102 mutações catalogadas.
2. Atualização da página `site/docs/gabarito.html` com acordeões expansíveis sob cada poema, exibindo detalhadamente os testes de mutação e contra-provas disponíveis.
3. Atualização das dependências para consumir o crate `joaju-lirica 0.6.0` do registro `gambi`.
4. Extensão dos testes de integração WASM em `tests/wasm_api.rs` para atestar a aplicação de deltas e conformidade dos diagnósticos em WebAssembly.

---

## 2. Componente de Visualização (Accordion de Mutações)

Cada poema canônico na tabela de `site/docs/gabarito.html` conterá:
- Badge com a quantidade de mutações disponíveis (ex.: `4 mutações`).
- Seção `<details class="gabarito-mutacoes-accordion">` expansível.
- Listagem com:
  - **Identificador e Tipo:** badge temático (`Métrica`, `Rima`, `Estrutura`, `Configuração`, `Combinada`).
  - **Descrição pedagógica** do desvio induzido.
  - **Operações:** texto substituído, inserido ou suprimido.
  - **Resultado Esperado:** faixa de confiança percentual (`nota_min%` a `nota_max%`) e observações obrigatórias geradas.

---

## 3. Critérios de Aceite

1. Todos os 55 poemas do gabarito renderizam corretamente com busca instantânea.
2. Poemas com mutações exibem o controle expansível funcional e responsivo.
3. Suíte de testes `wasm_api.rs` passa com 100% de sucesso.
4. Auditoria Gosplan aprovada (`cccp check --strict`).

---

## 4. Laudo de Conclusão (Fase 9)

- **Status:** Concluído e Aprovado
- **Gabarito Canônico:** 55 poemas de referência
- **Mutações de Estresse:** 102 mutações catalogadas
- **Testes WASM:** 17/17 testes aprovados
- **Qualidade Gosplan:** 100% de conformidade estrita

