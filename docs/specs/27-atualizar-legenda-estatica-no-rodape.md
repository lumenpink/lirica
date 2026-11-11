# Especificação: Atualizar Legenda Estática do Rodapé (Ticket #27)

## 1. Contexto e Motivação
No rodapé do laboratório (`site/index.html`), o bloco `.guide-legend` oferecia apenas uma listagem parcial dos símbolos do motor. Faltavam:
- Metros: Decassílabo Geral (`📜`), Dodecassílabo Geral (`⚜️`), Octossílabo (`🎵`), Eneassílabo (`🌊`) e Hendecassílabo (`🕊️`).
- Rimas: Clichê Histórico (`🔥`), Rima Rica (`✨`), Rima Preciosa (`💎`), Rima Consoante/Pobre (`✦`) e Rima Toante/Imperfeita (`⚡`).

Isso fazia com que poemas com variação métrica (como o verso em eneassílabo de *Ninguém viva descurado*) ou rimas destacadas com fogo apresentassem símbolos enigmáticos para quem consulta a barra inferior rápida.

## 2. Decisão Arquitetural (Árvore de Decisões para a Natasha)
- **Alternativa A:** Remover a legenda estática e manter apenas o modal interativo.
  *Descarte:* Piora a usabilidade de consulta rápida em telas de desktop, exigindo cliques adicionais para dúvidas simples.
- **Alternativa B:** Expandir a legenda estática em múltiplas linhas organizadas por domínio fonológico.
  *Solução Adotada:*
  - Manter Linha 1: Notação Fonética de Escansão (Tônica, Sinalefa `‿`, Extramétrica `·`, Rima).
  - Linha 2: Ícones de Ritmo e Metros Clássicos (todos os 12 símbolos suportados por `joaju-lirica::icone_metro`).
  - Linha 3: Ícones de Auditoria de Rimas (Clichê `🔥`, Rica `✨`, Preciosa `💎`, Consoante `✦`, Toante `⚡`).
  - Cada item contém atributo `title` explicativo e aciona o modal de detalhamento `#modal-legenda` ao toque/clique.

## 3. Critérios de Aceitação
- A seção `.guide-legend` em `site/index.html` deve conter linhas estruturadas para Ritmo e para Rimas.
- Todos os ícones (`📜`, `🎵`, `🌊`, `🕊️`, `⚜️`, `🔥`, `✨`, `💎`, `✦`, `⚡`) devem estar presentes na legenda do rodapé.
- Testes automatizados em `tests/wasm_api.rs` garantem a presença e integridade da legenda estática.
