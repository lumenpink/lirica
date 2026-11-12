# Especificação Técnica — Ticket #33

## Contexto & Motivação
A avaliação qualitativa de rimas do Lírica baseia-se na identificação das classes gramaticais dos vocábulos rimantes terminais de cada verso. Vocábulos de mesma classe configuram rima pobre; classes distintas configuram rima rica; artifícios enclíticos geram rimas preciosas; e terminações de escassez fonológica atestada produzem rimas raras.

Entretanto, o léxico português apresenta vasta homografia categorial (ex.: *canto* substantivo vs. verbo; *morto*, *torto*, *frio*, *escuro*, *cego*, *mudo* oscilando entre substantivo e adjetivo). O motor original dependia de um lookback imediato restrito a $T_{-1}$ e listas léxicas fixas, resultando em uma faixa excessivamente ampla de pares diagnosticados como `Indeterminada`.

A usuária estabeleceu como diretriz mandatória:
> *"E agora podemos pensar em outras formas de validar as classes das palavras para definir o tipo de rimas? Já está ajudando um monte, mas ainda temos uma faixa muito grande de indefinições. Lembro que as regras devem ser conservadoras. Nunca podemos classificar erroneamente. Na dúvida provável e indeterminado são a resposta padrão."*

Para Natasha manter este sistema de forma sustentável, a camada de resolução morfológica deve expandir deterministicamente as âncoras sintáticas sem gerar falsos positivos.

## Decisões Técnicas & Arquitetura

### 1. Janela Ampliada de Lookback Sintático (Janela de 3 Tokens)
A retroanálise de tokens anteriores ao terminal rimante $T_0$ passa a inspecionar uma janela de até 3 posições ($T_{-1}, T_{-2}, T_{-3}$):

1. **R-DET Ampliado (Determinação Direta e Mediata):**
   - Base de determinantes:
     - Artigos definidos e indefinidos (*o, a, os, as, um, uma, uns, umas*).
     - Contrações prepositivas (*do, da, dos, das, no, na, nos, nas, pelo, pela, pelos, pelas, num, numa, nuns, numas, dum, duma, duns, dumas, ao, aos, à, às*).
     - Possessivos (*meu, minha, meus, minhas, teu, tua, teus, tuas, seu, sua, seus, suas, nosso, nossa, nossos, nossas, vosso, vossa, vossos, vossas*).
     - Demonstrativos (*este, esta, estes, estas, esse, essa, esses, essas, aquele, aquela, aqueles, aquelas, isto, isso, aquilo*, e suas contrações *deste, nesta, naquele*, etc.).
   - Se $T_{-1}$ for determinante e $T_0$ contiver `SUBSTANTIVO`: $T_0$ é resolvido como `SUBSTANTIVO`.
   - Se $T_{-2}$ ou $T_{-3}$ for determinante e os tokens intervenientes forem modificadores nominais (adjetivos ou advérbios, ex.: *"o meu doce canto"*, *"um tão suave pranto"*, *"este perpétuo sono"*): $T_0$ é resolvido como `SUBSTANTIVO`.

2. **R-PREP Ampliado (Regência Preposicional com Intercalação):**
   - Preposições infinitivas: *para, de, a, sem, por, com, até, em*.
   - Se $T_{-1}$ for preposição e $T_0$ tiver terminação infinitiva (*-ar, -er, -ir, -or*): $T_0$ é resolvido como `VERBO`.
   - Se $T_{-2}$ for preposição e $T_{-1}$ for pronome clítico (*me, te, se, nos, vos, lhe, lhes*) ou negação (*não, nunca, jamais*), ex.: *"sem se ver"*, *"de não sofrer"*, *"para me amar"*: $T_0$ é resolvido como `VERBO`.

3. **R-ADV-MOD (Modificação por Intensificador Adjetival):**
   - Advérbios de intensidade: *tão, muito, mais, menos, bem, pouco, quase, bastante, demais*.
   - Se $T_{-1}$ for intensificador e $T_0$ contiver `ADJETIVO` (ex.: *"tão frio"*, *"mais puro"*, *"bem triste"*): $T_0$ é resolvido como `ADJETIVO`.

4. **R-PRON Ampliado (Predicação Pessoal):**
   - Se $T_{-1}$ ou $T_{-2}$ (com clítico/negação intercalada) for pronome reto (*eu, tu, ele, ela, nós, vós, eles, elas*) ou pronome reflexivo/oblíquo e $T_0$ contiver `VERBO`: $T_0$ é resolvido como `VERBO`.

### 2. Regra de Convergência Nominal Conservadora
Quando $T_1$ e $T_2$ pertencem à esfera nominal (ambos contêm `SUBSTANTIVO` e/ou `ADJETIVO` e nenhum é verbo no verso):
- **Convergência Substantivo-Adjetival:** Se ambos compartilham a ambiguidade `{Substantivo, Adjetivo}` (ex.: *morto × torto*, *escuro × puro*, *frio × vazio*), o algoritmo não força um rótulo artificial de 100% de certeza nem abandona em *Indeterminada*; categoriza como `ProvavelPobre` sob o fundamento epistêmico de convergência nominal.
- **Assimetria Funcional Evidenciada:** Se um dos termos foi resolvido sintaticamente como `SUBSTANTIVO` e o outro como `ADJETIVO` (por exemplo, após adjetivo qualificando substantivo antecedente), a rima é promovida a `Rica` ou `ProvavelRica`.
- **Verbo Categórico vs Esfera Nominal:** Se um termo é `VERBO` comprovado e o outro é de esfera nominal, categoriza como `ProvavelRica`.

### 3. Princípio de Salvaguarda Epistêmica
Se não houver ancoragem léxica, morfológica ou sintática suficiente, o par permanece categorizado como `Indeterminada`. O algoritmo é terminantemente proibido de adivinhar ou chutar classes.

## Alternativas Descartadas
- *Alternativa A: Forçar modelo estocástico ou classificador Bayesiano em runtime.*
  - Descartada: Quebraria o determinismo do WebAssembly, aumentaria desnecessariamente o tamanho do binário e criaria comportamentos imprevisíveis para Natasha depurar.
- *Alternativa B: Considerar qualquer par com ambiguidade como Rima Rica para favorecer a métrica do poeta.*
  - Descartada: Viola expressamente a exigência de conservadorismo da usuária; rotularia erroneamente rimas consagradas como pobres ou homófonas como se fossem ricas.
