# Especificação — Ticket #34: Gerenciador de Múltiplos Rascunhos Locais

## 1. Contexto e Motivação
O laboratório Lírica possuía até então persistência automática de um único rascunho poético no `localStorage` sob a chave `lirica_draft_poem` (Ticket #20 e #23). Qualquer criação de novo poema ou teste de múltiplos temas exigia sobrescrever o texto anterior ou copiá-lo externamente.

O objetivo do Ticket #34 é viabilizar o gerenciamento local completo de múltiplos rascunhos poéticos (criar, salvar, alternar, duplicar, renomear, excluir, exportar e importar backups JSON), preservando a soberania da usuária sobre seus dados e mantendo execução 100% client-side sem servidores externos.

---

## 2. Árvore de Decisão e Racional Técnico (Para Natasha)

### Decisão 1: Formato de Armazenamento e Migração Transparente
- **Opção A (Descartada):** Substituir imediatamente `lirica_draft_poem` por `lirica_drafts_v1` descartando o dado anterior.
  - *Problema:* Perda de dados da usuária que já utilizava o laboratório.
- **Opção B (Escolhida):** Migração no bootstrap. Se `lirica_drafts_v1` não existir e houver `lirica_draft_poem`, converte automaticamente o texto legado no primeiro rascunho com ID gerado (`draft-legado-timestamp`), definindo-o como ativo.

### Decisão 2: Esquema do Objeto de Rascunho
```json
{
  "id": "draft_1700000000000_abc123",
  "titulo": "Amor é fogo que arde sem se ver",
  "autor": "Luís de Camões",
  "texto": "# Amor é fogo que arde sem se ver\n## Luís de Camões\n\nAmor é fogo...",
  "atualizadoEm": "2026-11-14T12:00:00.000Z"
}
```
- O texto integral preserva as linhas `# Título` e `## Autor` para manter compatibilidade absoluta com o modo livre e a sincronização bidirecional do laboratório.

### Decisão 3: Esquema do Arquivo de Backup JSON
```json
{
  "schema": "lirica-drafts-v1",
  "versao": "0.1.0",
  "exportadoEm": "2026-11-14T12:00:00.000Z",
  "rascunhos": [
    {
      "id": "...",
      "titulo": "...",
      "autor": "...",
      "texto": "...",
      "atualizadoEm": "..."
    }
  ]
}
```
- A importação valida o campo `schema: "lirica-drafts-v1"` e faz mesclagem inteligente (adiciona rascunhos importados preservando os existentes, evitando colisões de ID).

### Decisão 4: Sincronização com o Autosave Existente
- O autosave existente (`agendarSalvamentoRascunho`) com debounce de 400ms passa a sincronizar tanto o rascunho ativo no array de rascunhos (`lirica_drafts_v1`) quanto a chave de compatibilidade legada (`lirica_draft_poem`), assegurando retrocompatibilidade total.

---

## 3. Contrato de Interface e Comportamento
1. **Botão de Rascunhos no Cabeçalho:**
   - Botão `#btn-abrir-rascunhos` com ícone 📂 e badge numérico `#drafts-count-badge` exibindo a contagem atual de rascunhos.
2. **Modal `#modal-rascunhos`:**
   - Abertura com tecla de atalho ou clique no botão. Fechamento com Escape, clique no overlay ou botões de fechar.
   - Barra de ações: "Novo Rascunho", "Exportar Backup JSON", "Importar Backup JSON".
   - Lista renderizada dinamicamente com título, autor, data formatada, botão "Abrir" (marca como ativo e carrega no laboratório), "Duplicar", "Renomear" e "Excluir".
   - Proteção de integridade: ao excluir o último rascunho, um novo rascunho em branco é automaticamente inicializado.
