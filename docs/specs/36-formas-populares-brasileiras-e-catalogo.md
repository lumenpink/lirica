# Especificação Técnica — Ticket #36

## Contexto & Motivação
O Lírica possuía um catálogo de exemplos e repertório formal fortemente centrado na tradição lírica clássica europeia (Camões, Petrarca, Shakespeare, Bocage, Espinel) e no modernismo/parnasianismo do eixo centro-sul (Bilac, Gonçalves Dias, Vinícius de Moraes, Fernando Pessoa, Cruz e Sousa).

Para democratizar e valorizar a extraordinária riqueza das formas poéticas tradicionais e populares do Brasil, o laboratório incorpora:
1. **O Romance do Pavão Misterioso** (José Camelo de Melo Rezende / Leandro Gomes de Barros), marco canônico da literatura de cordel nordestina.
2. **A Pajada Gaúcha** (tradição do Rio Grande do Sul e do Prata, imortalizada por Jayme Caetano Braun), com a Décima Campeira em Redondilha Maior e ritmo de milonga.
3. **O Galope à Beira-Mar**, forma virtuosística da cantoria e do repente nordestino, em décimas com ritmo galopante e refrão fixo na beira do mar.
4. **O Folheto de Cordel Encadeado**, reconhecendo sequências contínuas de estrofes de cordel e homenageando a marca clássica de 32 estrofes (o folheto tipográfico de 8 páginas).

## Decisões Técnicas & Arquitetura

### 1. Formas Poéticas e Avaliação Estrutural
O validador formal passa a reconhecer as seguintes estruturas canônicas:

1. **Galope à Beira-Mar**:
   - 10 versos em ritmo galopante (decassílabo ou hendecassílabo, métrica entre 10 e 11 sílabas com acentuação ternária/galopante).
   - Rimas no esquema clássico de décima: `ABBAACCDDC`.
   - O 10º verso encerra obrigatoriamente rimando em *-ar* e contendo a deixa/mote *"beira do mar"* ou *"beira-mar"*.
2. **Pajada Gaúcha**:
   - 10 versos em **Redondilha Maior** (7 sílabas poéticas).
   - Rimas no esquema da Décima Espinela (`ABBAACCDDC`).
   - Diferencia-se do martelo por adotar estritamente a redondilha de 7 sílabas e vocabulário/cadência tradicional campeira.
3. **Folheto de Cordel (Sextilhas Encadeadas)**:
   - Poema composto por múltiplas estrofes de sextilha em Redondilha Maior (7 sílabas) com rimas populares `XAXBXB` ou `XAXAXA`.
   - Quando atinge exatamente 32 estrofes, o motor emite uma nota de honra reconhecendo o padrão tipográfico canônico do folheto de 8 páginas (*in-octavo*).

### 2. Catálogo de Exemplos em WebAssembly (`carregar_exemplos`)
Adicionados três novos monumentos poéticos ao acervo pré-carregado:
- `pavao_misterioso`: Extrato canônico de *O Romance do Pavão Misterioso* em sextilhas de cordel.
- `pajada_gaucha`: Décima clássica da Pajada Gaúcha em redondilha maior (*Gauchismo* / Jayme Caetano Braun).
- `galope_beira_mar`: Décima galopante clássica da cantoria nordestina.

### 3. Atualização de Documentação e Gabarito
- Atualizar [site/docs/formas.html](file:///home/lumen/src/lirica/site/docs/formas.html) e [site/docs/gabarito.html](file:///home/lumen/src/lirica/site/docs/gabarito.html) com descrições das novas formas, regras de acentuação, esquemas de rimas e exemplos práticos.

## Alternativas Descartadas
- *Alternativa A: Exigir exatamente 32 estrofes para considerar um poema como Cordel.*
  - Descartada: Causaria rejeição de cordéis curtos ou de folhetos de 16/24/64 páginas, além de criar sobrecarga cognitiva e inviabilizar testes ágeis no laboratório.
