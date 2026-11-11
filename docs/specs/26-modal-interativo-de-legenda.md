# Especificação: Modal Interativo de Legenda Métrica e Rímica (Ticket #26)

## 1. Contexto e Motivação
A interface do *Lírica* exibe badges visuais ao lado de cada verso e no cabeçalho das estrofes contendo símbolos de metro (ex.: `⚔️`, `🏛️`, `🌊`, `🌿`, `👑`) e rima (ex.: `A 🔥`, `✨ Rica: amar × sonhar`).
Em computadores de mesa, tooltips de atributo `title` oferecem pequenas dicas ao passar o mouse. No entanto:
1. Em telas sensíveis ao toque (dispositivos móveis), o `:hover` inexiste.
2. Símbolos como a onda `🌊` (eneassílabo) e o fogo `🔥` (clichê histórico de rima) causavam dúvidas sem documentação contextual explícita.
3. Clicar em um badge deve apresentar imediatamente um glossário completo e didático da notação.

## 2. Decisão Arquitetural (Árvore de Decisões para a Natasha)
- **Alternativa A:** Abrir um balão tooltip flutuante ancorado no badge.
  *Descarte:* Em telas verticais de smartphones, balões ancorados quebram o viewport, cortam textos longos e sobrepõem o poema.
- **Alternativa B:** Rolar a página até o rodapé.
  *Descarte:* Desloca a visão do verso atual que o usuário estava inspecionando, gerando fadiga de navegação.
- **Solução Adotada:**
  - Criação de uma janela modal semântica `#modal-legenda` em `site/index.html` seguindo o mesmo padrão do `#modal-exemplos` (`role="dialog"`, `aria-modal="true"`).
  - Categorização em três seções didáticas:
    1. **Métrica e Ritmo:** decassílabos (heroico, sáfico, martelo, geral), redondilhas (maior, menor), alexandrinos, dodecassílabos, octossílabos, eneassílabos, hendecassílabos e verso livre.
    2. **Rimas e Tradição:** clichê lírico (`🔥`), rima rica (`✨`), preciosa (`💎`), pobre/consoante (`✦`) e imperfeita/toante (`⚡`).
    3. **Regras de Escansão Clássica:** sinalefa (`‿`) e sílaba extramétrica de Castilho (`·`).
  - Listeners de clique/toque em `.meter-badge`, `.rhyme-badge` e botão dedicado de ajuda.
  - Fechamento ergonômico: botão ✕, clique fora no backdrop e tecla `Escape`.

## 3. Critérios de Aceitação
- Elemento `#modal-legenda` presente no DOM com acessibilidade padrão (`role="dialog"`, `aria-modal="true"`).
- Badges `.meter-badge` e `.rhyme-badge` recebem `role="button"`, `tabindex="0"` e cursor de ponteiro com estilo interativo.
- Disparo de evento de clique em `.meter-badge` e `.rhyme-badge` abre a modal semântica `#modal-legenda`.
- Fechamento funciona via botão `#btn-fechar-modal-legenda`, backdrop ou `Escape`.
- Testes automatizados em `tests/wasm_api.rs` garantem a integridade dos contratos de frontend.
