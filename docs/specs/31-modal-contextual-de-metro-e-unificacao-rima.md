# Especificação Técnica — Ticket #31

## Contexto & Motivação
Anteriormente, o clique em qualquer `.meter-badge` ou `.rhyme-badge` na linha do verso abria o modal geral (`#modal-legenda`) contendo todo o catálogo estático de metros e rimas ao mesmo tempo. Isso gerava sobrecarga de informação para a pessoa usuária, que desejava entender apenas a métrica ou a rima daquele verso específico. Além disso, havia redundância desnecessária entre a avaliação de rima no topo da estrofe e na lateral do verso.

Natasha precisa de um fluxo coeso e modular:
1. O badge métrico (`.meter-badge`) deve abrir um modal contextual dedicado àquele verso (explicando o metro, o ritmo/tônicas e o contexto do cômputo métrico sem repetir a tira gráfica de sílabas).
2. O badge de rima (`.rhyme-badge`) deve acionar exatamente o mesmo diagnóstico em prosa límpida gerado para a estrofe (`gerarConteudoAuditoriaRima`), garantindo fonte única da verdade e consistência semântica.
3. A legenda geral completa permanece disponível no rodapé do documento para consulta estática de referência.

## Decisões Técnicas & Arquitetura
1. **Modal Contextual de Métrica (`abrirModalMetroContextual(vData, vIndex)`)**:
   - Atualiza dinamicamente o conteúdo do `#modal-legenda` com foco exclusivo no metro do verso:
     - **Cabeçalho:** Ícone do metro + Classificação (ex.: `⚜️ Decassílabo Heróico`) e subtítulo com o número de sílabas poéticas.
     - **Definição Lírica do Metro:** Explicação do metro poético na tradição lusófona (Monossílabo ao Alexandrino / Bárbaro, variações Heróico / Sáfico / Martelo do decassílabo).
     - **Contexto da Avaliação do Verso:** Posições das sílabas tônicas no verso, cômputo pela Regra de Castilho e registro de fusões (sinalefas) sem duplicar a tira visual de escansão.
     - **Navegação:** Botão de fechar e atalho sutil para a legenda geral do rodapé.

2. **Modal/Drawer Contextual de Rima (`abrirModalRimaContextual(par, qualLabel)`)**:
   - Ao clicar no `.rhyme-badge` de um verso:
     - Localiza o `ParRimaWeb` correspondente no poema.
     - Destaca o par na estrofe e os versos rimantes no workspace (`highlightActiveRhymeVerses`).
     - Abre o modal com o mesmo diagnóstico gerado por `gerarConteudoAuditoriaRima(par, qualLabel)`.
     - Caso o usuário prefira interagir direto no topo da estrofe, o drawer da estrofe continua disponível e exibe idêntico conteúdo.

3. **Preservação de Acessibilidade & Teclado**:
   - Foco automático no botão de fechar ao abrir o modal.
   - Fechamento por tecla `Escape` e clique no backdrop.

## Alternativas Descartadas
- *Alternativa A: Criar um segundo elemento `<dialog>` no HTML apenas para o modal contextual.*
  - Descartada: Duplicaria estrutura de backdrop e controle de foco. O container `#modal-legenda` já possui a infraestrutura acessível (`role="dialog"`, `aria-modal="true"`); basta injetar o corpo contextual ou estático conforme o disparador.
- *Alternativa B: Abrir o modal geral e apenas dar scroll até a linha correspondente.*
  - Descartada: Não resolve a sobrecarga cognitiva e polui a leitura do usuário em telas menores.
