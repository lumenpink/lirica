# Especificação Técnica — Ticket #30

## Contexto & Motivação
A gaveta de auditoria de rimas no topo das estrofes (`.stanza-audit-drawer`) foi projetada originalmente com campos de inspeção interna do motor léxico:
- `Classificação: <Enum>` (redundante com a legenda do próprio botão).
- `Auditoria Fonotática: ClassesDistintas(...)` (vazamento de enum e máscaras binárias incompreensíveis para pessoas usuárias e leitoras de poesia).
- Diagnósticos fragmentados em vez de explicação em prosa límpida.

Natasha precisa de um código que separe rigorosamente o modelo computacional da camada pedagógica da interface. O objetivo deste ticket é transformar a exibição da rima em prosa natural e acessível, expondo as classes gramaticais em português e explicando com clareza o valor e a categoria lírica do par.

## Decisões Técnicas & Arquitetura
1. **Enriquecimento do Modelo WebAssembly (`src/lib.rs` / `ParRimaWeb`)**:
   - Adicionar os campos:
     - `classes_morfologicas: (Vec<String>, Vec<String>)`: Lista em português das classes gramaticais de cada palavra rimante (ex.: `(["Substantivo", "Adjetivo"], ["Adjetivo"])`).
     - `explicacao_prosa: String`: Síntese textual didática em prosa para leigos sobre o par e sua qualidade (Rica, Pobre, Preciosa, Rara ou Provável).
   - Manter retrocompatibilidade de `auditoria` como string interna no JSON para não quebrar consumidores existentes, mas não utilizá-la na UI.

2. **Template Pedagógico em `site/app.js` (`stanza-audit-drawer`)**:
   - Eliminar as linhas de `Auditoria Fonotática` e repetição de `Classificação`.
   - Exibir o confronto direto dos vocábulos com suas classes entre parênteses:
     > **morto** *(substantivo / adjetivo)* × **torto** *(adjetivo)*
   - Renderizar o parágrafo explicativo em prosa (`.audit-prose-text`).
   - Destacar o alerta de clichê histórico de maneira amigável (`🔥 Clichê Lírico:`).
   - Modularizar a função geradora de HTML do diagnóstico para reuso no Ticket #31.

## Alternativas Descartadas
- *Alternativa A: Formatar as strings de classes apenas no frontend JavaScript a partir da string raw `auditoria`.*
  - Descartada: Parser frágil de strings em JavaScript gera dívida técnica e quebra facilmente com atualizações da biblioteca `joaju-lirica`. O backend WebAssembly já tem acesso estruturado aos autômatos léxicos e à `PosMask`.
- *Alternativa B: Manter "Auditoria Fonotática" como texto secundário colapsado.*
  - Descartada: O campo não agrega valor para a análise lírica do usuário comum e causa ruído visual desnecessário.
